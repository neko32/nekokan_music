use crate::types::*;
use crate::validation::FieldErrors;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub data: MusicData,
    pub on_data_change: Callback<MusicData>,
    pub filename: String,
    pub on_filename_change: Callback<String>,
    pub errors: FieldErrors,
    pub on_save: Callback<()>,
    pub focus_title: bool,
    pub on_focus_title_done: Callback<()>,
    /// 既存ファイル名一覧（"xxx.json" 形式）。同名チェックに使用。
    pub existing_filenames: Vec<String>,
    /// 編集中のファイル名（"xxx.json"）。None は新規。同名時は自分を除いて判定。
    pub selected_filename: Option<String>,
    /// ファイル名入力からフォーカスが外れたときに呼ばれる。同名なら親でエラー表示・フォーカス戻し。
    pub on_filename_blur: Callback<String>,
    pub focus_filename: bool,
    pub on_focus_filename_done: Callback<()>,
}

fn err(props: &FormProps, key: &str) -> Option<String> {
    props.errors.get(key).cloned()
}

fn input_class(props: &FormProps, key: &str) -> &'static str {
    if props.errors.contains_key(key) {
        "input input-error"
    } else {
        "input"
    }
}

fn record_year_join(ry: &[i32]) -> String {
    ry.iter().map(|y| y.to_string()).collect::<Vec<_>>().join(", ")
}

/// ファイル名として不適切な文字を除去。スペースは _ に置換する。
fn sanitize_for_filename(s: &str) -> String {
    const INVALID: &[char] = &['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    s.replace(' ', "_")
        .chars()
        .filter(|c| !c.is_control() && !INVALID.contains(c))
        .collect()
}

/// ファイル名入力フォーカス時に自動入力する値を返す。
fn suggested_filename_on_focus(data: &MusicData) -> Option<String> {
    let main = data.janre.main.as_str();
    if main == "Classical" {
        // soloists → conductor → orchestra の順
        data.personnel
            .soloists
            .first()
            .map(|e| sanitize_for_filename(e.name.trim()))
            .or_else(|| {
                data.personnel
                    .conductor
                    .first()
                    .map(|e| sanitize_for_filename(e.name.trim()))
            })
            .or_else(|| {
                data.personnel
                    .orchestra
                    .first()
                    .map(|e| sanitize_for_filename(e.name.trim()))
            })
            .filter(|s| !s.is_empty())
    } else if main == "Jazz" || main == "Fusion" {
        data.personnel.leader.first().and_then(|entry| {
            let name = sanitize_for_filename(entry.name.trim());
            if name.is_empty() {
                return None;
            }
            let title = sanitize_for_filename(data.title.trim());
            Some(if title.is_empty() {
                name
            } else {
                format!("{}__{}", name, title)
            })
        })
    } else {
        None
    }
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let sub_opts = sub_janres_for_main(&props.data.janre.main);
    let title_input_ref = use_node_ref();
    let filename_input_ref = use_node_ref();
    let record_year_text = use_state(|| record_year_join(&props.data.record_year));

    let on_save = props.on_save.clone();
    let filename = props.filename.clone();
    let on_filename_change = props.on_filename_change.clone();
    let on_filename_blur = props.on_filename_blur.clone();

    {
        let ry = props.data.record_year.clone();
        let record_year_text = record_year_text.clone();
        use_effect_with(ry, move |r| {
            record_year_text.set(record_year_join(r));
            || ()
        });
    }

    {
        let focus_title = props.focus_title;
        let title_input_ref = title_input_ref.clone();
        let on_focus_title_done = props.on_focus_title_done.clone();
        use_effect_with(focus_title, move |f| {
            if *f {
                if let Some(inp) = title_input_ref.cast::<web_sys::HtmlInputElement>() {
                    let _ = inp.focus();
                }
                on_focus_title_done.emit(());
            }
            || ()
        });
    }

    {
        let focus_filename = props.focus_filename;
        let filename_input_ref = filename_input_ref.clone();
        let on_focus_filename_done = props.on_focus_filename_done.clone();
        use_effect_with(focus_filename, move |f| {
            if *f {
                if let Some(inp) = filename_input_ref.cast::<web_sys::HtmlInputElement>() {
                    let _ = inp.focus();
                }
                on_focus_filename_done.emit(());
            }
            || ()
        });
    }

    html! {
        <form class="music-form" onsubmit={Callback::from(move |e: SubmitEvent| { e.prevent_default(); on_save.emit(()); })}>
            <div class="form-section">
                <h3>{"Basic Information"}</h3>
                <div class="field">
                    <label>{"Title"}</label>
                    <input
                        ref={title_input_ref.clone()}
                        type="text"
                        class={input_class(props, "title")}
                        value={props.data.title.clone()}
                        oninput={update_str(props.data.clone(), props.on_data_change.clone(), |d, v| d.title = v)}
                        maxlength="128"
                    />
                    { for err(props, "title").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>

                <div class="field">
                    <label>{"Main Janre"}</label>
                    <select
                        class={input_class(props, "janre.main")}
                        value={props.data.janre.main.clone()}
                        onchange={update_str_select(props.data.clone(), props.on_data_change.clone(), |d, v| d.janre.main = v)}
                    >
                        { for MAIN_JANRES.iter().map(|&v| {
                            let is_selected = props.data.janre.main == v;
                            if is_selected {
                                html! { <option value={v} selected={true}>{ v }</option> }
                            } else {
                                html! { <option value={v}>{ v }</option> }
                            }
                        }) }
                    </select>
                    { for err(props, "janre.main").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>

                <div class="field">
                    <label>{"Sub Janre"}</label>
                    <select
                        class={input_class(props, "janre.sub")}
                        multiple={true}
                        value={props.data.janre.sub.join(",")}
                        onchange={update_multi_sub(props.data.clone(), props.on_data_change.clone())}
                    >
                        { for sub_opts.iter().map(|&v| {
                            let is_selected = props.data.janre.sub.contains(&v.to_string());
                            if is_selected {
                                html! { <option value={v} selected={true}>{ v }</option> }
                            } else {
                                html! { <option value={v}>{ v }</option> }
                            }
                        }) }
                    </select>
                    { for err(props, "janre.sub").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>

                <div class="field">
                    <label>{"Label"}</label>
                    <input
                        type="text"
                        class={input_class(props, "label")}
                        value={props.data.label.clone()}
                        oninput={update_str(props.data.clone(), props.on_data_change.clone(), |d, v| d.label = v)}
                        maxlength="64"
                    />
                    { for err(props, "label").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>

                <div class="field">
                    <label>{"Id"}</label>
                    <input
                        type="text"
                        class={input_class(props, "id")}
                        value={props.data.id.clone()}
                        oninput={update_str(props.data.clone(), props.on_data_change.clone(), |d, v| d.id = v)}
                        maxlength="64"
                    />
                    { for err(props, "id").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>

                <div class="field">
                    <label>{"Release Year"}</label>
                    <input
                        type="number"
                        class={input_class(props, "release_year")}
                        value={props.data.release_year.to_string()}
                        oninput={update_i32(props.data.clone(), props.on_data_change.clone(), |d, v| d.release_year = v)}
                        min="1900"
                        max="2099"
                    />
                    { for err(props, "release_year").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>

                <div class="field">
                    <label>{"Recording Year"}</label>
                    <input
                        type="text"
                        class={input_class(props, "record_year")}
                        value={(*record_year_text).clone()}
                        oninput={record_year_input(record_year_text.clone())}
                        onblur={record_year_blur(record_year_text.clone(), props.data.clone(), props.on_data_change.clone())}
                        placeholder="例: 1991, 1992"
                    />
                    { for err(props, "record_year").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>
            </div>

            <PersonnelSection data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />

            <TracksSection data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />

            <div class="form-section">
                <h3>{"評価・日付"}</h3>
                <div class="field">
                    <label>{"Score"}</label>
                    <select
                        class={input_class(props, "score")}
                        value={props.data.score.to_string()}
                        onchange={update_score(props.data.clone(), props.on_data_change.clone())}
                    >
                        { for [1,2,3,4,5,6].iter().map(|&v| {
                            let is_selected = props.data.score == v;
                            if is_selected {
                                html! { <option value={v.to_string()} selected={true}>{ v }</option> }
                            } else {
                                html! { <option value={v.to_string()}>{ v }</option> }
                            }
                        }) }
                    </select>
                    { for err(props, "score").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>
                <div class="field">
                    <label>{"Comment"}</label>
                    <textarea
                        class="input"
                        rows="4"
                        value={props.data.comment.clone()}
                        oninput={update_str(props.data.clone(), props.on_data_change.clone(), |d, v| d.comment = v)}
                    />
                </div>
                <div class="field">
                    <label>{"Date"}</label>
                    <input
                        type="text"
                        class={input_class(props, "date")}
                        value={props.data.date.clone()}
                        oninput={update_str(props.data.clone(), props.on_data_change.clone(), |d, v| d.date = v)}
                        placeholder="YYYY/MM/DD"
                    />
                    { for err(props, "date").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                </div>
            </div>

            <ReferencesSection data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />

            <div class="form-section">
                <div class="field">
                    <label>{"ファイル名"}</label>
                    <input
                        ref={filename_input_ref.clone()}
                        type="text"
                        class={input_class(props, "filename")}
                        value={filename}
                        onfocus={{
                            let data = props.data.clone();
                            let on_filename_change = props.on_filename_change.clone();
                            Callback::from(move |_: FocusEvent| {
                                if let Some(s) = suggested_filename_on_focus(&data) {
                                    on_filename_change.emit(s);
                                }
                            })
                        }}
                        onblur={{
                            let on_filename_blur = on_filename_blur.clone();
                            Callback::from(move |e: FocusEvent| {
                                if let Some(target) = e.target() {
                                    if let Ok(inp) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                        let v: String = inp.value();
                                        let v = v.trim().to_string();
                                        if !v.is_empty() {
                                            on_filename_blur.emit(v);
                                        }
                                    }
                                }
                            })
                        }}
                        oninput={Callback::from(move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                            if let Some(inp) = input {
                                on_filename_change.emit(inp.value());
                            }
                        })}
                        placeholder="例: Artist__Album"
                    />
                    { for err(props, "filename").into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                    <span class="hint">{"保存時に .json が付きます"}</span>
                </div>
                <button type="submit" class="btn-save">{"保存"}</button>
            </div>
        </form>
    }
}

fn update_str<F>(data: MusicData, on_data_change: Callback<MusicData>, f: F) -> Callback<InputEvent>
where
    F: Fn(&mut MusicData, String) + 'static,
{
    Callback::from(move |e: InputEvent| {
        let target = match e.target() {
            Some(t) => t,
            None => return,
        };
        let value = target
            .dyn_ref::<web_sys::HtmlInputElement>()
            .map(|el| el.value())
            .or_else(|| target.dyn_ref::<web_sys::HtmlTextAreaElement>().map(|el| el.value()))
            .unwrap_or_default();
        let mut d = data.clone();
        f(&mut d, value);
        on_data_change.emit(d);
    })
}

fn update_str_select<F>(data: MusicData, on_data_change: Callback<MusicData>, f: F) -> Callback<Event>
where
    F: Fn(&mut MusicData, String) + 'static,
{
    Callback::from(move |e: Event| {
        let select = e.target_dyn_into::<web_sys::HtmlSelectElement>();
        if let Some(sel) = select {
            let mut d = data.clone();
            f(&mut d, sel.value());
            on_data_change.emit(d);
        }
    })
}

fn update_i32<F>(data: MusicData, on_data_change: Callback<MusicData>, f: F) -> Callback<InputEvent>
where
    F: Fn(&mut MusicData, i32) + 'static,
{
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            if let Ok(v) = inp.value().parse::<i32>() {
                let mut d = data.clone();
                f(&mut d, v);
                on_data_change.emit(d);
            }
        }
    })
}

fn record_year_input(record_year_text: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let target = match e.target() {
            Some(t) => t,
            None => return,
        };
        if let Some(inp) = target.dyn_ref::<web_sys::HtmlInputElement>() {
            record_year_text.set(inp.value());
        }
    })
}

fn record_year_blur(
    record_year_text: UseStateHandle<String>,
    data: MusicData,
    on_data_change: Callback<MusicData>,
) -> Callback<FocusEvent> {
    Callback::from(move |_| {
        let years: Vec<i32> = (*record_year_text)
            .split(',')
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.parse().ok())
            .collect();
        let mut d = data.clone();
        d.record_year = years;
        on_data_change.emit(d);
    })
}

fn update_multi_sub(data: MusicData, on_data_change: Callback<MusicData>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let select = e.target_dyn_into::<web_sys::HtmlSelectElement>();
        if let Some(sel) = select {
            let opts = sel.selected_options();
            let mut selected = Vec::new();
            for i in 0..opts.length() {
                let opt: Option<web_sys::HtmlOptionElement> = opts
                    .get_with_index(i)
                    .and_then(|el| el.dyn_into::<web_sys::HtmlOptionElement>().ok());
                if let Some(opt) = opt {
                    if opt.selected() {
                        selected.push(opt.value());
                    }
                }
            }
            let mut d = data.clone();
            d.janre.sub = selected;
            on_data_change.emit(d);
        }
    })
}

fn update_score(data: MusicData, on_data_change: Callback<MusicData>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let select = e.target_dyn_into::<web_sys::HtmlSelectElement>();
        if let Some(sel) = select {
            if let Ok(v) = sel.value().parse::<i32>() {
                let mut d = data.clone();
                d.score = v;
                on_data_change.emit(d);
            }
        }
    })
}

// --- Personnel section ---
#[derive(Properties, PartialEq)]
struct PersonnelSectionProps {
    data: MusicData,
    on_data_change: Callback<MusicData>,
    errors: FieldErrors,
}

#[function_component(PersonnelSection)]
fn personnel_section(props: &PersonnelSectionProps) -> Html {
    html! {
        <div class="form-section">
            <h3>{"Personnel"}</h3>
            <ConductorBlock entries={props.data.personnel.conductor.clone()} data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />
            <OrchestraBlock entries={props.data.personnel.orchestra.clone()} data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />
            <CompanyBlock entries={props.data.personnel.company.clone()} data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />
            <SoloistsBlock entries={props.data.personnel.soloists.clone()} data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />
            <LeaderBlock entries={props.data.personnel.leader.clone()} data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />
            <SidemenBlock entries={props.data.personnel.sidemen.clone()} data={props.data.clone()} on_data_change={props.on_data_change.clone()} errors={props.errors.clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PersonnelBlockProps<T: PartialEq + Clone> {
    entries: Vec<T>,
    data: MusicData,
    on_data_change: Callback<MusicData>,
    errors: FieldErrors,
}

fn conductor_row(
    data: MusicData,
    on_data_change: Callback<MusicData>,
    entry: &ConductorEntry,
    i: usize,
    errors: &FieldErrors,
) -> Html {
    let key_name = format!("personnel.conductor[{}].name", i);
    let key_tracks = format!("personnel.conductor[{}].tracks", i);
    let err_name = errors.get(&key_name).cloned();
    let err_tracks = errors.get(&key_tracks).cloned();
    html! {
        <>
            <span class="input-wrap">
                <input type="text" placeholder="Name" value={entry.name.clone()}
                    oninput={update_conductor(data.clone(), on_data_change.clone(), i, true)}
                    class={if errors.contains_key(&key_name) { "input input-error" } else { "input" }}/>
                { for err_name.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Tracks" value={entry.tracks.clone()}
                    oninput={update_conductor(data.clone(), on_data_change.clone(), i, false)}
                    class={if errors.contains_key(&key_tracks) { "input input-error" } else { "input" }}/>
                { for err_tracks.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
        </>
    }
}

fn update_conductor(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, is_name: bool) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(e) = d.personnel.conductor.get_mut(idx) {
                if is_name {
                    e.name = v;
                } else {
                    e.tracks = v;
                }
            }
            on_data_change.emit(d);
        }
    })
}

fn orchestra_row(
    data: MusicData,
    on_data_change: Callback<MusicData>,
    entry: &OrchestraEntry,
    i: usize,
    errors: &FieldErrors,
) -> Html {
    let key_name = format!("personnel.orchestra[{}].name", i);
    let key_tracks = format!("personnel.orchestra[{}].tracks", i);
    let err_name = errors.get(&key_name).cloned();
    let err_tracks = errors.get(&key_tracks).cloned();
    html! {
        <>
            <span class="input-wrap">
                <input type="text" placeholder="Orchestra Name" value={entry.name.clone()}
                    oninput={update_orchestra(data.clone(), on_data_change.clone(), i, true)} class={if errors.contains_key(&key_name) { "input input-error" } else { "input" }}/>
                { for err_name.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Tracks" value={entry.tracks.clone()}
                    oninput={update_orchestra(data.clone(), on_data_change.clone(), i, false)} class={if errors.contains_key(&key_tracks) { "input input-error" } else { "input" }}/>
                { for err_tracks.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
        </>
    }
}

fn update_orchestra(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, is_name: bool) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(e) = d.personnel.orchestra.get_mut(idx) {
                if is_name {
                    e.name = v;
                } else {
                    e.tracks = v;
                }
            }
            on_data_change.emit(d);
        }
    })
}

fn company_row(
    data: MusicData,
    on_data_change: Callback<MusicData>,
    entry: &CompanyEntry,
    i: usize,
    errors: &FieldErrors,
) -> Html {
    let key_name = format!("personnel.company[{}].name", i);
    let key_tracks = format!("personnel.company[{}].tracks", i);
    let err_name = errors.get(&key_name).cloned();
    let err_tracks = errors.get(&key_tracks).cloned();
    html! {
        <>
            <span class="input-wrap">
                <input type="text" placeholder="Company Name" value={entry.name.clone()}
                    oninput={update_company(data.clone(), on_data_change.clone(), i, true)} class={if errors.contains_key(&key_name) { "input input-error" } else { "input" }}/>
                { for err_name.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Tracks" value={entry.tracks.clone()}
                    oninput={update_company(data.clone(), on_data_change.clone(), i, false)} class={if errors.contains_key(&key_tracks) { "input input-error" } else { "input" }}/>
                { for err_tracks.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
        </>
    }
}

fn update_company(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, is_name: bool) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(e) = d.personnel.company.get_mut(idx) {
                if is_name {
                    e.name = v;
                } else {
                    e.tracks = v;
                }
            }
            on_data_change.emit(d);
        }
    })
}

fn soloist_row(
    data: MusicData,
    on_data_change: Callback<MusicData>,
    entry: &SoloistEntry,
    i: usize,
    errors: &FieldErrors,
) -> Html {
    let key_name = format!("personnel.soloists[{}].name", i);
    let key_inst = format!("personnel.soloists[{}].instrument", i);
    let key_tracks = format!("personnel.soloists[{}].tracks", i);
    let err_name = errors.get(&key_name).cloned();
    let err_inst = errors.get(&key_inst).cloned();
    let err_tracks = errors.get(&key_tracks).cloned();
    html! {
        <>
            <span class="input-wrap">
                <input type="text" placeholder="Name" value={entry.name.clone()} oninput={update_soloist(data.clone(), on_data_change.clone(), i, 0)} class={if errors.contains_key(&key_name) { "input input-error" } else { "input" }}/>
                { for err_name.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Instrument" value={entry.instrument.clone()} oninput={update_soloist(data.clone(), on_data_change.clone(), i, 1)} class={if errors.contains_key(&key_inst) { "input input-error" } else { "input" }}/>
                { for err_inst.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Tracks" value={entry.tracks.clone()} oninput={update_soloist(data.clone(), on_data_change.clone(), i, 2)} class={if errors.contains_key(&key_tracks) { "input input-error" } else { "input" }}/>
                { for err_tracks.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
        </>
    }
}

fn update_soloist(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, field: u8) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(e) = d.personnel.soloists.get_mut(idx) {
                match field {
                    0 => e.name = v,
                    1 => e.instrument = v,
                    _ => e.tracks = v,
                }
            }
            on_data_change.emit(d);
        }
    })
}

fn leader_row(
    data: MusicData,
    on_data_change: Callback<MusicData>,
    entry: &LeaderEntry,
    i: usize,
    errors: &FieldErrors,
) -> Html {
    let key_name = format!("personnel.leader[{}].name", i);
    let key_inst = format!("personnel.leader[{}].instruments", i);
    let key_tracks = format!("personnel.leader[{}].tracks", i);
    let err_name = errors.get(&key_name).cloned();
    let err_inst = errors.get(&key_inst).cloned();
    let err_tracks = errors.get(&key_tracks).cloned();
    html! {
        <>
            <span class="input-wrap">
                <input type="text" placeholder="Name" value={entry.name.clone()} oninput={update_leader(data.clone(), on_data_change.clone(), i, 0)} class={if errors.contains_key(&key_name) { "input input-error" } else { "input" }}/>
                { for err_name.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Instruments" value={entry.instruments.clone()} oninput={update_leader(data.clone(), on_data_change.clone(), i, 1)} class={if errors.contains_key(&key_inst) { "input input-error" } else { "input" }}/>
                { for err_inst.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Tracks" value={entry.tracks.clone()} oninput={update_leader(data.clone(), on_data_change.clone(), i, 2)} class={if errors.contains_key(&key_tracks) { "input input-error" } else { "input" }}/>
                { for err_tracks.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
        </>
    }
}

fn update_leader(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, field: u8) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(e) = d.personnel.leader.get_mut(idx) {
                match field {
                    0 => e.name = v,
                    1 => e.instruments = v,
                    _ => e.tracks = v,
                }
            }
            on_data_change.emit(d);
        }
    })
}

fn sidemen_row(
    data: MusicData,
    on_data_change: Callback<MusicData>,
    entry: &SidemenEntry,
    i: usize,
    errors: &FieldErrors,
) -> Html {
    let key_name = format!("personnel.sidemen[{}].name", i);
    let key_inst = format!("personnel.sidemen[{}].instruments", i);
    let key_tracks = format!("personnel.sidemen[{}].tracks", i);
    let err_name = errors.get(&key_name).cloned();
    let err_inst = errors.get(&key_inst).cloned();
    let err_tracks = errors.get(&key_tracks).cloned();
    html! {
        <>
            <span class="input-wrap">
                <input type="text" placeholder="Name" value={entry.name.clone()} oninput={update_sidemen(data.clone(), on_data_change.clone(), i, 0)} class={if errors.contains_key(&key_name) { "input input-error" } else { "input" }}/>
                { for err_name.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Instruments" value={entry.instruments.clone()} oninput={update_sidemen(data.clone(), on_data_change.clone(), i, 1)} class={if errors.contains_key(&key_inst) { "input input-error" } else { "input" }}/>
                { for err_inst.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
            <span class="input-wrap">
                <input type="text" placeholder="Tracks" value={entry.tracks.clone()} oninput={update_sidemen(data.clone(), on_data_change.clone(), i, 2)} class={if errors.contains_key(&key_tracks) { "input input-error" } else { "input" }}/>
                { for err_tracks.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            </span>
        </>
    }
}

fn update_sidemen(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, field: u8) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(e) = d.personnel.sidemen.get_mut(idx) {
                match field {
                    0 => e.name = v,
                    1 => e.instruments = v,
                    _ => e.tracks = v,
                }
            }
            on_data_change.emit(d);
        }
    })
}

#[function_component(ConductorBlock)]
fn conductor_block(props: &PersonnelBlockProps<ConductorEntry>) -> Html {
    let add = { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.conductor.push(Default::default()); on_data_change.emit(d); }) };
    let remove = |i: usize| { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.conductor.remove(i); on_data_change.emit(d); }) };
    html! {
        <div class="personnel-block">
            <h4>{"Conductor"}</h4>
            { for props.entries.iter().enumerate().map(|(i, entry)| html! {
                <div class="personnel-row" key={i}>
                    { conductor_row(props.data.clone(), props.on_data_change.clone(), entry, i, &props.errors) }
                    <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                </div>
            }) }
            <button type="button" class="btn-add" onclick={add}>{"追加"}</button>
        </div>
    }
}

#[function_component(OrchestraBlock)]
fn orchestra_block(props: &PersonnelBlockProps<OrchestraEntry>) -> Html {
    let add = { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.orchestra.push(Default::default()); on_data_change.emit(d); }) };
    let remove = |i: usize| { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.orchestra.remove(i); on_data_change.emit(d); }) };
    html! {
        <div class="personnel-block">
            <h4>{"Orchestra"}</h4>
            { for props.entries.iter().enumerate().map(|(i, entry)| html! {
                <div class="personnel-row" key={i}>
                    { orchestra_row(props.data.clone(), props.on_data_change.clone(), entry, i, &props.errors) }
                    <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                </div>
            }) }
            <button type="button" class="btn-add" onclick={add}>{"追加"}</button>
        </div>
    }
}

#[function_component(CompanyBlock)]
fn company_block(props: &PersonnelBlockProps<CompanyEntry>) -> Html {
    let add = { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.company.push(Default::default()); on_data_change.emit(d); }) };
    let remove = |i: usize| { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.company.remove(i); on_data_change.emit(d); }) };
    html! {
        <div class="personnel-block">
            <h4>{"Company"}</h4>
            { for props.entries.iter().enumerate().map(|(i, entry)| html! {
                <div class="personnel-row" key={i}>
                    { company_row(props.data.clone(), props.on_data_change.clone(), entry, i, &props.errors) }
                    <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                </div>
            }) }
            <button type="button" class="btn-add" onclick={add}>{"追加"}</button>
        </div>
    }
}

#[function_component(SoloistsBlock)]
fn soloists_block(props: &PersonnelBlockProps<SoloistEntry>) -> Html {
    let add = { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.soloists.push(Default::default()); on_data_change.emit(d); }) };
    let remove = |i: usize| { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.soloists.remove(i); on_data_change.emit(d); }) };
    html! {
        <div class="personnel-block">
            <h4>{"Soloists"}</h4>
            { for props.entries.iter().enumerate().map(|(i, entry)| html! {
                <div class="personnel-row" key={i}>
                    { soloist_row(props.data.clone(), props.on_data_change.clone(), entry, i, &props.errors) }
                    <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                </div>
            }) }
            <button type="button" class="btn-add" onclick={add}>{"追加"}</button>
        </div>
    }
}

#[function_component(LeaderBlock)]
fn leader_block(props: &PersonnelBlockProps<LeaderEntry>) -> Html {
    let add = { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.leader.push(Default::default()); on_data_change.emit(d); }) };
    let remove = |i: usize| { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.leader.remove(i); on_data_change.emit(d); }) };
    html! {
        <div class="personnel-block">
            <h4>{"Leader"}</h4>
            { for props.entries.iter().enumerate().map(|(i, entry)| html! {
                <div class="personnel-row" key={i}>
                    { leader_row(props.data.clone(), props.on_data_change.clone(), entry, i, &props.errors) }
                    <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                </div>
            }) }
            <button type="button" class="btn-add" onclick={add}>{"追加"}</button>
        </div>
    }
}

#[function_component(SidemenBlock)]
fn sidemen_block(props: &PersonnelBlockProps<SidemenEntry>) -> Html {
    let add = { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.sidemen.push(Default::default()); on_data_change.emit(d); }) };
    let remove = |i: usize| { let data = props.data.clone(); let on_data_change = props.on_data_change.clone(); Callback::from(move |_| { let mut d = data.clone(); d.personnel.sidemen.remove(i); on_data_change.emit(d); }) };
    html! {
        <div class="personnel-block">
            <h4>{"Sidemen"}</h4>
            { for props.entries.iter().enumerate().map(|(i, entry)| html! {
                <div class="personnel-row" key={i}>
                    { sidemen_row(props.data.clone(), props.on_data_change.clone(), entry, i, &props.errors) }
                    <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                </div>
            }) }
            <button type="button" class="btn-add" onclick={add}>{"追加"}</button>
        </div>
    }
}

// --- Tracks section ---
#[derive(Properties, PartialEq)]
struct TracksSectionProps {
    data: MusicData,
    on_data_change: Callback<MusicData>,
    errors: FieldErrors,
}

#[function_component(TracksSection)]
fn tracks_section(props: &TracksSectionProps) -> Html {
    let add = {
        let data = props.data.clone();
        let on_data_change = props.on_data_change.clone();
        Callback::from(move |_| {
            let mut d = data.clone();
            d.tracks.push(Track {
                disc_no: 1,
                no: (d.tracks.len() + 1) as i32,
                title: String::new(),
                composer: String::new(),
                length: String::new(),
            });
            on_data_change.emit(d);
        })
    };
    let remove = |i: usize| {
        let data = props.data.clone();
        let on_data_change = props.on_data_change.clone();
        Callback::from(move |_| {
            let mut d = data.clone();
            d.tracks.remove(i);
            on_data_change.emit(d);
        })
    };
    let tracks_section_err = props.errors.get("tracks").cloned();
    html! {
        <div class="form-section">
            <h3>{"Tracks"}</h3>
            { for tracks_section_err.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
            { for props.data.tracks.iter().enumerate().map(|(i, t)| {
                let key_title = format!("tracks[{}].title", i);
                let key_composer = format!("tracks[{}].composer", i);
                let key_length = format!("tracks[{}].length", i);
                let err_title = props.errors.get(&key_title).cloned();
                let err_composer = props.errors.get(&key_composer).cloned();
                let err_length = props.errors.get(&key_length).cloned();
                let data = props.data.clone();
                let on_data_change = props.on_data_change.clone();
                html! {
                    <div class="track-row" key={i}>
                        <span>{"Disc No:"}</span><input type="number" class="input track-no" placeholder="Disc" value={t.disc_no.to_string()}
                            oninput={update_track_field(data.clone(), on_data_change.clone(), i, 0)}/>
                        <span>{"Track No:"}</span><input type="number" class="input track-no" placeholder="No" value={t.no.to_string()}
                            oninput={update_track_field(data.clone(), on_data_change.clone(), i, 1)}/>
                        <span class="input-wrap">
                            <input type="text" class={if props.errors.contains_key(&key_title) { "input input-error" } else { "input" }} placeholder="Title" value={t.title.clone()}
                                oninput={update_track_field_str(data.clone(), on_data_change.clone(), i, 2)}/>
                            { for err_title.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                        </span>
                        <span class="input-wrap">
                            <input type="text" class={if props.errors.contains_key(&key_composer) { "input input-error" } else { "input" }} placeholder="Composer" value={t.composer.clone()}
                                oninput={update_track_field_str(data.clone(), on_data_change.clone(), i, 3)}/>
                            { for err_composer.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                        </span>
                        <span class="input-wrap">
                            <input type="text" class={if props.errors.contains_key(&key_length) { "input input-error" } else { "input" }} placeholder="Length (MM:SS or M:SS)" value={t.length.clone()}
                                oninput={update_track_field_str(data.clone(), on_data_change.clone(), i, 4)}/>
                            { for err_length.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                        </span>
                        <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                    </div>
                }
            }) }
            <button type="button" class="btn-add" onclick={add}>{"トラック追加"}</button>
        </div>
    }
}

fn update_track_field(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, field: u8) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            if let Ok(v) = inp.value().parse::<i32>() {
                let mut d = data.clone();
                if let Some(t) = d.tracks.get_mut(idx) {
                    match field {
                        0 => t.disc_no = v,
                        1 => t.no = v,
                        _ => {}
                    }
                }
                on_data_change.emit(d);
            }
        }
    })
}

fn update_track_field_str(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, field: u8) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(t) = d.tracks.get_mut(idx) {
                match field {
                    2 => t.title = v,
                    3 => t.composer = v,
                    4 => t.length = v,
                    _ => {}
                }
            }
            on_data_change.emit(d);
        }
    })
}

// --- References section ---
#[derive(Properties, PartialEq)]
struct ReferencesSectionProps {
    data: MusicData,
    on_data_change: Callback<MusicData>,
    errors: FieldErrors,
}

#[function_component(ReferencesSection)]
fn references_section(props: &ReferencesSectionProps) -> Html {
    let add = {
        let data = props.data.clone();
        let on_data_change = props.on_data_change.clone();
        Callback::from(move |_| {
            let mut d = data.clone();
            d.references.push(Reference::default());
            on_data_change.emit(d);
        })
    };
    let remove = |i: usize| {
        let data = props.data.clone();
        let on_data_change = props.on_data_change.clone();
        Callback::from(move |_| {
            let mut d = data.clone();
            d.references.remove(i);
            on_data_change.emit(d);
        })
    };
    html! {
        <div class="form-section">
            <h3>{"References"}</h3>
            { for props.data.references.iter().enumerate().map(|(i, r)| {
                let key_name = format!("references[{}].name", i);
                let key_url = format!("references[{}].url", i);
                let err_name = props.errors.get(&key_name).cloned();
                let err_url = props.errors.get(&key_url).cloned();
                html! {
                    <div class="ref-row" key={i}>
                        <span class="input-wrap">
                            <input type="text" class={if props.errors.contains_key(&key_name) { "input input-error" } else { "input" }} placeholder="Name" value={r.name.clone()}
                                oninput={update_ref(props.data.clone(), props.on_data_change.clone(), i, true)}/>
                            { for err_name.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                        </span>
                        <span class="input-wrap">
                            <input type="text" class={if props.errors.contains_key(&key_url) { "input input-error" } else { "input" }} placeholder="URL" value={r.url.clone()}
                                oninput={update_ref(props.data.clone(), props.on_data_change.clone(), i, false)}/>
                            { for err_url.into_iter().map(|e| html! { <span class="error-text">{ e }</span> }) }
                        </span>
                        <button type="button" class="btn-remove" onclick={remove(i)}>{"削除"}</button>
                    </div>
                }
            }) }
            <button type="button" class="btn-add" onclick={add}>{"参照追加"}</button>
        </div>
    }
}

fn update_ref(data: MusicData, on_data_change: Callback<MusicData>, idx: usize, is_name: bool) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
        if let Some(inp) = input {
            let v = inp.value();
            let mut d = data.clone();
            if let Some(r) = d.references.get_mut(idx) {
                if is_name {
                    r.name = v;
                } else {
                    r.url = v;
                }
            }
            on_data_change.emit(d);
        }
    })
}
