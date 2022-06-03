use yew::prelude::*;

mod state;
mod components;
use state::StarData;
use state::StarAction;
use components::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
        Callback::from(move |card_type:CardType| state.dispatch(StarAction::AddPriorityCard(card_type)))
    };

    let destroy_priority = {
        let state = state.clone();
        Callback::<usize>::from(move |index| state.dispatch(StarAction::DestroyPriorityCard(index)))
    };

    let destroy_card = {
        let state = state.clone();
        Callback::from(move |index:usize| state.dispatch(StarAction::DestroyCard(index)))
    };

    let cards = state.cards.clone();
    let p_cards = state.priority_cards.clone();
    html! {
        <>
        <div id="modal_host"></div>
        <div class="container">
            <h1 class="title">{ "Star" }<span class="material-symbols-outlined star">{ "star" }</span></h1>
            <p class="subtitle">{ "Barista Helper" }</p>
            <div class="card_column">
            {
                for p_cards.iter().enumerate().map(|(index, card)| {
                    html! {
                        <Card card_type={ card.card_type } is_priority={true} index={index} create_card={create_card.clone()} add_priority_card={set_priority_card.clone()} destroy_priority={destroy_priority.clone()}  destroy_card={destroy_card.clone()} />
                    }
                })
            }
            {
                for cards.iter().enumerate().map(|(index, card)| {
                    html! {
                        <Card card_type={ card.card_type } index={index} create_card={create_card.clone()} add_priority_card={set_priority_card.clone()} destroy_priority={destroy_priority.clone()}  destroy_card={destroy_card.clone()} />
                    }
                })
            }
            </div>
        </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Star>::new().render();
}
