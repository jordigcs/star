use yew::prelude::*;

mod state;
mod components;
use state::StarData;
use state::StarAction;
use components::*;

#[function_component]
fn Star() -> Html {
    let mut data = StarData::new();
    let state = use_reducer(|| data);
    let create_card = {
        let state = state.clone();
        Callback::from(move |card_type:CardType| state.dispatch(StarAction::AddCard(card_type)))
    };

    let set_priority_card = {
        let state = state.clone();
        Callback::from(move |card_type:CardType| state.dispatch(StarAction::SetPriorityCard(card_type)))
    };

    let destroy_card = {
        let state = state.clone();
        Callback::from(move |index:usize| state.dispatch(StarAction::DestroyCard(index)))
    };

    let mut cards = state.cards.clone();
    if let Some(card) = state.priority_card.clone() {
        cards.push_front(card);
    }
    html! {
        <div class="container">
            <h1 class="title">{ "Star" }<span class="material-icons star">{ "star" }</span></h1>
            <p class="subtitle">{ "Barista Helper" }</p>
            <div class="card_column">
            {
                for cards.iter().enumerate().map(|(index, card)| { 
                    html! {
                        <Card card_type={ card.card_type } index={index} create_card={create_card.clone()} set_priority_card={set_priority_card.clone()} destroy_card={destroy_card.clone()} />
                    }
                })
            }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<Star>::new().render();
}
