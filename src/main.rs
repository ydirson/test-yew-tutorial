use yew::prelude::*;

use gloo_net::http::Request;
use serde::Deserialize;
use yew_autoprops::autoprops;

const APP_NAME: &str = "General's Familiar";

const GET_ARMY_BASE_URL: &str = "https://army-forge.onepagerules.com/api/tts";

const ARMY_ID: &str = "ybjR2-7kHUNY";
//const ARMY_ID: &str = "VV8Zy0GIfOUX";

#[derive(Clone, PartialEq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Army {
    id: String,
    name: String,
    game_system: String,
    points: usize,
    points_limit: usize,
    units: Vec<Unit>,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
struct Unit {
    id: String,
    name: String,
    size: usize,
    quality: usize,
    defense: usize,
    equipment: Vec<Equipment>,
}

#[derive(Clone, PartialEq, Debug, Deserialize)]
struct Equipment {
    id: String,
    name: String,
    range: usize,
    attacks: usize,
    count: usize,
}

//

#[autoprops]
#[function_component(EquipmentList)]
fn equipment_list(equipment: &Vec<Equipment>) -> Html {
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
            </p>
        }
    }).collect()
}

//

#[autoprops]
#[function_component(ArmyList)]
fn army_list(army: &Army, on_click: &Callback<Unit>) -> Html {
    html! {
        <>
            <ybc::Title size={ybc::HeaderSize::Is2}>{army.game_system.to_uppercase()}{" - "}{army.name.clone()}</ybc::Title>
            <ybc::Tile vertical={true}>
                <ybc::Title size={ybc::HeaderSize::Is3}>{"Units"}</ybc::Title>
                <UnitsList units={army.units.clone()} on_click={on_click.clone()} />
            </ybc::Tile>
        </>
    }
}

//

#[autoprops]
#[function_component(UnitsList)]
fn units_list(units: &Vec<Unit>, on_click: &Callback<Unit>) -> Html {
    let on_click = on_click.clone();
    units.iter().map(|unit| {
        let on_unit_select = {
            let on_click = on_click.clone();
            let unit = unit.clone();
            Callback::from(move |_| {
                on_click.emit(unit.clone())
            })
        };

        html! {
            <ybc::Tile>
            <p onclick={on_unit_select}>{
                format!("{name} [{size}]", name=unit.name, size=unit.size)
            }</p>
            </ybc::Tile>
        }
    }).collect()
}

//

#[autoprops]
#[function_component(UnitDetails)]
fn unit_details(unit: &Unit) -> Html {
    html! {
        <div>
            <ybc::Title size={ybc::HeaderSize::Is3}>{
                format!("{name} [{size}]: Q{q} D{d}", name=unit.name,
                        size=unit.size, q=unit.quality, d=unit.defense) }
            </ybc::Title>
            <ybc::Tile vertical={true}>
                <EquipmentList equipment={unit.equipment.clone()} />
            </ybc::Tile>
        </div>
    }
}

//

#[function_component(App)]
fn app() -> Html {

    let army = use_state(|| None);
    {
        let army = army.clone();
        use_effect_with_deps(move |_| {
            let army = army.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_army: Army = Request::get(format!("{base_url}?id={id}",
                                                              base_url=GET_ARMY_BASE_URL,
                                                              id=ARMY_ID).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                army.set(Some(fetched_army));
            });
            || ()
        },
        ());
    }

    //

    let selected_unit = use_state(|| None);

    let on_unit_select = {
        let selected_unit = selected_unit.clone();
        Callback::from(move |unit: Unit| {
            selected_unit.set(Some(unit))
        })
    };

    let details = selected_unit.as_ref().map(|unit| html! {
        <UnitDetails unit={unit.clone()} />
    });

    //

    html! {
    <>
        <ybc::Navbar
            classes={classes!("is-success")}
            padded=true
            navbrand={html!{
                <ybc::NavbarItem>
                    <ybc::Title classes={classes!("has-text-white")} size={ybc::HeaderSize::Is4}>{APP_NAME}</ybc::Title>
                </ybc::NavbarItem>
            }}
            navstart={html!{}}
            navend={html!{}}
        />
        if !army.is_none() {
            <>

                <ybc::Container fluid=true>
                    <ArmyList army={(*army).clone().unwrap()} on_click={on_unit_select.clone()} />
                    { for details }
                </ybc::Container>
            </>
        }
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
