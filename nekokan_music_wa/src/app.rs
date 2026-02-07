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

#[function_component(App)]
pub fn app() -> Html {
    let file_list = use_state(|| Vec::<String>::new());
    let loading = use_state(|| true);
    let selected = use_state(|| None::<String>);
    let form_data = use_state(|| {
        let mut d = MusicData::default();
        d.date = today_str();
        d.release_year = 2000;
        d.score = 1;
        d.tracks.push(crate::types::Track {
            disc_no: 1,
            no: 1,
            title: String::new(),
            composer: String::new(),
            length: String::new(),
        });
        d
    });
    let form_filename = use_state(|| String::new());
    let errors = use_state(|| FieldErrors::new());
    let save_status = use_state(|| None::<Result<(), String>>);

    {
        let file_list = file_list.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            let file_list = file_list.clone();
            let loading = loading.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::list_files().await {
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

    let on_save = {
        let form_data = form_data.clone();
        let form_filename = form_filename.clone();
        let errors = errors.clone();
        let file_list = file_list.clone();
        let save_status = save_status.clone();
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
            let file_list = file_list.clone();
            let save_status = save_status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = api::save_file(&filename, &data).await;
                save_status.set(Some(res.clone()));
                if res.is_ok() {
                    if let Ok(list) = api::list_files().await {
                        file_list.set(list);
                    }
                }
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
            <aside class="sidebar">
                <h2 class="sidebar-title">{"JSON音楽データ"}</h2>
                if *loading {
                    <p class="sidebar-loading">{"読込中..."}</p>
                } else {
                    <ul class="file-list">
                        { for file_list.iter().map(|name| {
                            let is_selected = selected.as_deref() == Some(name.as_str());
                            let name_owned = name.clone();
                            let name_for_click = name.clone();
                            let on_select_file = on_select_file.clone();
                            html! {
                                <li key={name_owned.clone()}>
                                    <button
                                        class={if is_selected { "file-item selected" } else { "file-item" }}
                                        onclick={move |_| on_select_file.emit(name_for_click.clone())}
                                    >
                                        { name_owned }
                                    </button>
                                </li>
                            }
                        }) }
                    </ul>
                }
            </aside>
            <main class="content">
                <div class="content-inner">
                    <h1 class="app-title">{"nekokan_music_wa"}</h1>
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
