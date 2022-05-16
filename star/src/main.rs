use yew::prelude::*;

mod state;
mod components;
use state::StarData;
use state::StarAction;
use components::*;

#[function_component]
fn Star() -> Html {
    let data = StarData::new();
    let state = use_reducer(|| data);
    let create_card = {
        let state = state.clone();
        Callback::from(move |card_type:CardType| state.dispatch(StarAction::AddCard(card_type)))
    };

    let set_priority_card = {
        let state = state.clone();
        Callback::from(move |card_type:CardType| state.dispatch(StarAction::SetPriorityCard(card_type)))
    };

    let clear_priority = {
        let state = state.clone();
        Callback::<usize>::from(move |_| state.dispatch(StarAction::ClearPriorityCard()))
    };

    let destroy_card = {
        let state = state.clone();
        Callback::from(move |index:usize| state.dispatch(StarAction::DestroyCard(index)))
    };

    let cards = state.cards.clone();
    html! {
        <div class="container">
            <h1 class="title">{ "Star" }<span class="material-symbols-outlined star">{ "star" }</span></h1>
            <p class="subtitle">{ "Barista Helper" }</p>
            <div class="card_column">
            {
                if let Some(card) = state.priority_card.clone() {
                    html! {
                        <Card card_type={ card.card_type } is_priority={true} index={0} create_card={create_card.clone()} set_priority_card={set_priority_card.clone()} clear_priority={clear_priority.clone()} destroy_card={destroy_card.clone()} />
                    }
                }
                else {
                    html! {}
                }
            }
            {
                for cards.iter().enumerate().map(|(index, card)| {
                    html! {
                        <Card card_type={ card.card_type } index={index} create_card={create_card.clone()} set_priority_card={set_priority_card.clone()} clear_priority={clear_priority.clone()}  destroy_card={destroy_card.clone()} />
                    }
                })
            }
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Star>::new().render();
}
