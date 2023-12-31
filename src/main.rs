use yew::prelude::*;

use gloo_net::http::Request;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::iter::zip;
use std::rc::Rc;
use yew_autoprops::autoprops;

const APP_NAME: &str = "General's Familiar";

const GET_ARMY_BASE_URL: &str = "https://army-forge.onepagerules.com/api/tts";

const ARMY_IDS: [&str; 2] = ["ybjR2-7kHUNY", "VV8Zy0GIfOUX"];

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Army {
    id: String,
    name: String,
    game_system: String,
    points: usize,
    points_limit: usize,
    units: Vec<Rc<Unit>>,
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Unit {
    id: String,
    name: String,
    size: usize,
    quality: usize,
    defense: usize,
    special_rules: Vec<Rc<SpecialRule>>,
    equipment: Vec<Rc<Equipment>>,
}

#[derive(PartialEq, Debug, Deserialize)]
struct SpecialRule {
    name: String,
    #[serde(default)]
    rating: String,
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Equipment {
    id: String,
    name: String,
    range: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    attacks: usize,
    count: usize,
    special_rules: Vec<Rc<SpecialRule>>,
}

//

#[autoprops]
#[function_component(SpecialRulesList)]
fn special_rule_list(special_rules: &Vec<Rc<SpecialRule>>) -> Html {
    special_rules.iter()
        // render each rule
        .map(|special_rule| {
            let rating = match special_rule.rating.as_str() {
                "" => { "".to_string() },
                rating => { format!("({})", rating) },
            };
            html! {{
                format!("{name}{rating}", name=special_rule.name, rating=rating)
            }}
        })
        // zip with commas and skip the leading one
        .flat_map(|x| vec![html! { {", "} }, x])
        .skip(1)
        .collect()
}

//

#[autoprops]
#[function_component(EquipmentList)]
fn equipment_list(equipment: &Vec<Rc<Equipment>>) -> Html {
    equipment.iter().map(|equipment| {
        html! {
            <p>
                if equipment.count != 1 {
                    {format!("{}x ", equipment.count)}
                }
                {format!("{}: ", equipment.name)}
                if equipment.range != 0 {
                    {format!(r#" {}", "#, equipment.range )}
                }
                {format!("A{}", equipment.attacks)}
                if ! equipment.special_rules.is_empty() {
                    {", "}
                }
                <SpecialRulesList special_rules={equipment.special_rules.clone()} />
            </p>
        }
    }).collect()
}

//

#[autoprops]
#[function_component(ArmyList)]
fn army_list(army: &Rc<Army>, on_click: &Callback<Rc<Unit>>) -> Html {
    html! {
        <>
            <h2>{army.game_system.to_uppercase()}{" - "}{army.name.clone()}</h2>
            <div>
                <h3>{"Units"}</h3>
                <UnitsList units={army.units.clone()} on_click={on_click.clone()} />
            </div>
        </>
    }
}

//

#[autoprops]
#[function_component(UnitsList)]
fn units_list(units: &Vec<Rc<Unit>>, on_click: &Callback<Rc<Unit>>) -> Html {
    let on_click = on_click.clone();
    units.iter().map(|unit| {
        let on_unit_select = {
            let on_click = on_click.clone();
            let unit = Rc::clone(&unit);
            Callback::from(move |_| {
                on_click.emit(Rc::clone(&unit))
            })
        };

        html! {
            <p onclick={on_unit_select}>{
                format!("{name} [{size}]", name=unit.name, size=unit.size)
            }</p>
        }
    }).collect()
}

//

#[autoprops]
#[function_component(UnitDetails)]
fn unit_details(unit: &Rc<Unit>) -> Html {
    html! {
        <div>
            <h3>{ format!("{name} [{size}]: Q{q} D{d}", name=unit.name,
                          size=unit.size, q=unit.quality, d=unit.defense) }</h3>
            <SpecialRulesList special_rules={unit.special_rules.clone()} />
            <ul>
                <EquipmentList equipment={unit.equipment.clone()} />
            </ul>
        </div>
    }
}

//

enum AppStateAction {
    AddArmy{index: usize, army: Rc<Army>},
    SelectUnit{unit: Rc<Unit>},
    DeselectUnit{army_index: usize},
}

struct AppState {
    armies: Vec<Rc<Army>>,
    selected_units: Vec<Option<Rc<Unit>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { armies: vec!(), selected_units: vec!() }
    }
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut armies = self.armies.clone();
        let mut selected_units = self.selected_units.clone();
        match action {
            AppStateAction::AddArmy{index, army} => {
                assert!(index <= armies.len());
                if index == armies.len() {
                    armies.resize_with(index + 1, || Rc::clone(&army));
                    selected_units.resize_with(index + 1, || None);
                } else {
                    armies[index] = army;
                    selected_units[index] = None;
                }
            },
            AppStateAction::SelectUnit{unit} => {
                if let Some(index) = 'army_index: {
                    for (index, army) in self.armies.iter().enumerate() {
                        if army.units.iter().any(|e| Rc::<Unit>::ptr_eq(&e, &unit)) {
                            break 'army_index Some(index);
                        }
                    }
                    None
                } {
                    selected_units[index] = Some(unit);
                } else {
                    log::warn!("selecting unknown unit");
                }
            },
            AppStateAction::DeselectUnit{army_index} => {
                selected_units[army_index] = None;
            },
        }

        Self { armies, selected_units }.into()
    }
}

//

#[function_component(App)]
fn app() -> Html {

    let app_state = use_reducer(AppState::default);

    {
        let app_state = app_state.clone();
        use_effect_with_deps(move |_| {
            let _ = ARMY_IDS.iter().enumerate().map(
                |(i, army_id)| {
                    let app_state = app_state.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let fetched_army: Rc<Army> =
                            Request::get(format!("{base_url}?id={id}",
                                                 base_url=GET_ARMY_BASE_URL,
                                                 id=army_id).as_str())
                            .send()
                            .await
                            .expect("should get an HTTP answer")
                            .json()
                            .await
                            .expect("should deserialize Army from JSON content");
                        app_state.dispatch(AppStateAction::AddArmy{index:i, army:fetched_army});
                    });
                }).collect::<()>();
            || ()
        },
        ());
    }

    //

    let on_unit_select = {
        let app_state = app_state.clone();
        Callback::from(move |unit: Rc<Unit>| {
            let selected_item = app_state.selected_units
                .iter()
                .enumerate()
                .find(|(_, e)| match e {None => false,
                                        Some(e) => Rc::<Unit>::ptr_eq(&e, &unit)});
            match selected_item {
                None =>
                    app_state.dispatch(AppStateAction::SelectUnit{unit: Rc::clone(&unit)}),
                Some((army_index, _)) =>
                    app_state.dispatch(AppStateAction::DeselectUnit{army_index}),
            }
        })
    };

    //

    let armies = zip(app_state.armies.iter(),
                     app_state.selected_units.iter())
        .map(|(army, selected_unit)| {
            let details = selected_unit.as_ref().map(|unit| html! {
                <UnitDetails unit={Rc::clone(&unit)} />
            });

            html! {
                <div style="flex-grow: 1">
                    <ArmyList army={Rc::clone(&army)}
                              on_click={on_unit_select.clone()} />
                    { for details }
                </div>
            }
        });

    html! {
        <>
            <h1>{APP_NAME}</h1>
            <div style="display: flex">
                { for armies }
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    // document name
    let doc = web_sys::window()
        .expect("should have a window")
        .document()
        .expect("window should have a document");
    let title = doc.create_element("title").expect("should create title");
    title.set_text_content(Some(APP_NAME));
    doc
        .head()
        .expect("document should have a head")
        .append_child(&title)
        .expect("should set document title");

    yew::Renderer::<App>::new().render();
}
