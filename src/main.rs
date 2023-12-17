use yew::prelude::*;

mod static_data;

use crate::static_data::JSON_DATA;
use serde::Deserialize;

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
}

//

#[derive(Properties, PartialEq)]
struct ArmyListProps {
    army: Army,
    on_click: Callback<Unit>,
}

#[function_component(ArmyList)]
fn army_list(ArmyListProps { army, on_click }: &ArmyListProps) -> Html {
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

#[derive(Properties, PartialEq)]
struct UnitsListProps {
    units: Vec<Unit>,
    on_click: Callback<Unit>,
}

#[function_component(UnitsList)]
fn units_list(UnitsListProps { units, on_click }: &UnitsListProps) -> Html {
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
            <p key={unit.id.clone()} onclick={on_unit_select}>{
                format!("{name} [{size}]", name=unit.name, size=unit.size)
            }</p>
        }
    }).collect()
}

//

#[derive(Properties, PartialEq)]
struct UnitsDetailsProps {
    unit: Unit,
}

#[function_component(UnitDetails)]
fn unit_details(UnitsDetailsProps { unit }: &UnitsDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ format!("{name} [{size}]: Q{q} D{d}", name=unit.name,
                          size=unit.size, q=unit.quality, d=unit.defense) }</h3>
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
                let fetched_army: Army = serde_json::from_str(JSON_DATA)
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
        if !army.is_none() {
            <>
                <ArmyList army={(*army).clone().unwrap()} on_click={on_unit_select.clone()} />
                { for details }
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
