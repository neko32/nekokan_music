use crate::api;
use crate::types::MusicData;
use crate::validation::{validate_form, FieldErrors};
use js_sys::Date;
use wasm_bindgen::JsValue;
use yew::prelude::*;

fn log_validation_errors(errs: &FieldErrors) {
    web_sys::console::log_1(&JsValue::from_str("[nekokan_music_wa] バリデーションエラー:"));
    for (key, msg) in errs {
        web_sys::console::log_2(&JsValue::from_str(key), &JsValue::from_str(msg));
    }
}

fn today_str() -> String {
    let d = Date::new_0();
    let y = d.get_full_year();
    let m = d.get_month() + 1;
    let day = d.get_date();
    format!("{:04}/{:02}/{:02}", y, m, day)
}

/// 新規追加用のクリーンなフォームデータ（Main=Classical, Sub=Classicists）
fn new_music_data() -> MusicData {
    let mut d = MusicData::default();
    d.date = today_str();
    d.release_year = 2000;
    d.score = 1;
    d.janre.main = "Classical".into();
    d.janre.sub = vec!["Classicists".into()];
    d.tracks.push(crate::types::Track {
        disc_no: 1,
        no: 1,
        title: String::new(),
        composer: String::new(),
        length: String::new(),
    });
    d
}

#[function_component(App)]
pub fn app() -> Html {
    let file_list = use_state(|| Vec::<api::ListEntryWithLabel>::new());
    let loading = use_state(|| true);
    let selected = use_state(|| None::<String>);
    let form_data = use_state(|| new_music_data());
    let form_filename = use_state(|| String::new());
    let errors = use_state(|| FieldErrors::new());
    let save_status = use_state(|| None::<Result<(), String>>);
    let save_in_progress = use_state(|| false);
    let focus_title = use_state(|| false);
    let focus_filename = use_state(|| false);

    {
        let file_list = file_list.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            let file_list = file_list.clone();
            let loading = loading.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::list_with_labels().await {
                    Ok(list) => {
                        file_list.set(list);
                    }
                    Err(_) => {
                        file_list.set(vec![]);
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_select_file = {
        let form_data = form_data.clone();
        let form_filename = form_filename.clone();
        let selected = selected.clone();
        let errors = errors.clone();
        Callback::from(move |name: String| {
            let form_data = form_data.clone();
            let form_filename = form_filename.clone();
            let selected = selected.clone();
            let errors = errors.clone();
            let base = name.strip_suffix(".json").unwrap_or(&name).to_string();
            selected.set(Some(name.clone()));
            form_filename.set(base.clone());
            errors.set(FieldErrors::new());
            wasm_bindgen_futures::spawn_local(async move {
                match api::get_file(&name).await {
                    Ok(data) => {
                        form_data.set(data);
                    }
                    Err(_) => {}
                }
            });
        })
    };

    let on_add_new = {
        let form_data = form_data.clone();
        let form_filename = form_filename.clone();
        let selected = selected.clone();
        let errors = errors.clone();
        let focus_title = focus_title.clone();
        Callback::from(move |_| {
            form_data.set(new_music_data());
            form_filename.set(String::new());
            selected.set(None);
            errors.set(FieldErrors::new());
            focus_title.set(true);
        })
    };

    let on_focus_title_done = {
        let focus_title = focus_title.clone();
        Callback::from(move |()| focus_title.set(false))
    };

    // ファイル名 blur 時: 新規入力時のみ、同名が既に存在すればエラー表示しフォーカスを戻す。編集時は対象外（上書き保存は正当）。
    let on_filename_blur = {
        let file_list = file_list.clone();
        let selected = selected.clone();
        let errors = errors.clone();
        let focus_filename = focus_filename.clone();
        Callback::from(move |value: String| {
            if selected.is_some() {
                return;
            }
            let base = value.trim();
            let base = if base.ends_with(".json") {
                base.strip_suffix(".json").unwrap_or(base)
            } else {
                base
            };
            if base.is_empty() {
                return;
            }
            let existing: Vec<&str> = file_list
                .iter()
                .map(|e| e.filename.strip_suffix(".json").unwrap_or(e.filename.as_str()))
                .collect();
            let is_duplicate = existing.iter().any(|&s| s == base);
            if is_duplicate {
                let mut errs = FieldErrors::new();
                errs.insert("filename".into(), "同名ファイルが既に存在します".into());
                errors.set(errs);
                focus_filename.set(true);
            }
        })
    };

    let on_focus_filename_done = {
        let focus_filename = focus_filename.clone();
        Callback::from(move |()| focus_filename.set(false))
    };

    let on_save = {
        let form_data = form_data.clone();
        let form_filename = form_filename.clone();
        let errors = errors.clone();
        let file_list = file_list.clone();
        let save_status = save_status.clone();
        let save_in_progress = save_in_progress.clone();
        Callback::from(move |()| {
            let data = (*form_data).clone();
            let filename = (*form_filename).clone();
            let errs = validate_form(&data, &filename);
            if !errs.is_empty() {
                log_validation_errors(&errs);
                errors.set(errs);
                save_status.set(Some(Err("バリデーションエラー".into())));
                return;
            }
            errors.set(FieldErrors::new());
            save_in_progress.set(true);
            let file_list = file_list.clone();
            let save_status = save_status.clone();
            let save_in_progress = save_in_progress.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let save_fut = api::save_file(&filename, &data);
                let timeout_fut = gloo_timers::future::TimeoutFuture::new(10_000);
                futures::pin_mut!(save_fut, timeout_fut);
                match futures::future::select(save_fut, timeout_fut).await {
                    futures::future::Either::Left((res, _)) => {
                        let result: Result<(), String> = res;
                        save_status.set(Some(result.clone()));
                        if result.is_ok() {
                            if let Ok(list) = api::list_with_labels().await {
                                file_list.set(list);
                            }
                        }
                    }
                    futures::future::Either::Right(((), _)) => {
                        save_status.set(Some(Err(
                            "保存がタイムアウトしました（10秒）".into(),
                        )));
                    }
                }
                save_in_progress.set(false);
            });
        })
    };

    let form_data_clone = (*form_data).clone();
    let on_data_change = Callback::from(move |new_data: MusicData| form_data.set(new_data));
    let form_filename_val = (*form_filename).clone();
    let on_filename_change = Callback::from(move |s: String| form_filename.set(s));
    let errors_val = (*errors).clone();
    let has_validation_errors = !errors_val.is_empty();
    let errors_list: Vec<(String, String)> = errors_val
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    html! {
        <div class="layout">
            if *save_in_progress {
                <div class="save-modal-overlay" aria-busy="true" aria-live="polite">
                    <div class="save-modal-box">
                        <div class="save-modal-spinner" aria-hidden="true"></div>
                        <p class="save-modal-text">{"保存中..."}</p>
                    </div>
                </div>
            }
            <aside class="sidebar">
                <h2 class="sidebar-title">{"Nekokan Music Data"}</h2>
                if *loading {
                    <p class="sidebar-loading">{"読込中..."}</p>
                } else {
                    <ul class="file-list">
                        { for file_list.iter().map(|entry| {
                            let filename = entry.filename.clone();
                            let is_selected = selected.as_deref() == Some(filename.as_str());
                            let display_label = if entry.display_label.chars().count() >= 40 {
                                format!("{}...", entry.display_label.chars().take(37).collect::<String>())
                            } else {
                                entry.display_label.clone()
                            };
                            let filename_for_click = entry.filename.clone();
                            let on_select_file = on_select_file.clone();
                            html! {
                                <li key={filename.clone()}>
                                    <button
                                        class={if is_selected { "file-item selected" } else { "file-item" }}
                                        title={filename.clone()}
                                        onclick={move |_| on_select_file.emit(filename_for_click.clone())}
                                    >
                                        { display_label }
                                    </button>
                                </li>
                            }
                        }) }
                    </ul>
                    <br />
                    <br />
                    <a
                        href="#"
                        class="add-new-link"
                        onclick={move |e: MouseEvent| { e.prevent_default(); on_add_new.emit(()); }}
                    >
                        {"Add New Music"}
                    </a>
                }
            </aside>
            <main class="content">
                <div class="content-inner">
                    <h1 class="app-title">{"Nekokan Music"}</h1>
                    if has_validation_errors {
                        <div class="form-section validation-errors-summary" id="validation-errors-box">
                            <h3>{"バリデーションエラー"}</h3>
                            <p class="error-count">{ format!("{} 件のエラー", errors_list.len()) }</p>
                            <ul class="error-list">
                                { for errors_list.iter().map(|(k, v)| html! {
                                    <li class="error-item"><span class="error-key">{ k.clone() }</span>{ ": " }{ v.clone() }</li>
                                }) }
                            </ul>
                        </div>
                    }
                    <crate::form::Form
                        data={form_data_clone}
                        on_data_change={on_data_change}
                        filename={form_filename_val}
                        on_filename_change={on_filename_change}
                        errors={errors_val}
                        on_save={on_save}
                        focus_title={*focus_title}
                        on_focus_title_done={on_focus_title_done}
                        existing_filenames={file_list.iter().map(|e| e.filename.clone()).collect::<Vec<_>>()}
                        selected_filename={(*selected).clone()}
                        on_filename_blur={on_filename_blur}
                        focus_filename={*focus_filename}
                        on_focus_filename_done={on_focus_filename_done}
                    />
                    if let Some(ref status) = *save_status {
                        <p class={if status.is_ok() { "save-ok" } else { "save-err" }}>
                            { if status.as_ref().ok().is_some() {
                                "保存しました。".to_string()
                            } else {
                                status.as_ref().err().cloned().unwrap_or_default()
                            } }
                        </p>
                    }
                </div>
            </main>
        </div>
    }
}
