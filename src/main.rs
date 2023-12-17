use yew::prelude::*;

mod static_data;

use crate::static_data::JSON_DATA;
use serde::Deserialize;

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
                format!("{name} [{size}]: Q{q} D{d}", name=unit.name,
                        size=unit.size, q=unit.quality, d=unit.defense)
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
            <h3>{ unit.name.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

//

#[function_component(App)]
fn app() -> Html {

    let units = use_state(|| vec![]);
    {
        let units = units.clone();
        use_effect_with((), move |_| {
            let units = units.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_units: Vec<Unit> = serde_json::from_str(JSON_DATA)
                    .unwrap();
                units.set(fetched_units);
            });
            || ()
        });
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
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Units to watch"}</h3>
                <UnitsList units={(*units).clone()} on_click={on_unit_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
