use std::time::Duration;

use yew::prelude::*;
use wasm_timer::{Instant, Interval};
use wasm_logger::*;
use yew_hooks::use_interval;
#[derive(Properties, PartialEq, Clone)]
pub struct TimerData {
    pub active:bool,
    #[prop_or(1800)]
    pub time_left:u64, // Seconds
    #[prop_or_default]
    pub callback:Callback<u16>,
}

impl TimerData {
    pub fn format_time_left(time_left:u64) -> String {
        let minutes = time_left / 60;
        let seconds = time_left - (minutes * 60);
        
        format!("{:02}:{:02}", minutes, seconds)
    }
}

#[function_component]
pub fn Timer(data:&TimerData) -> Html {
    let data = data.clone();
    let last_time = use_state(|| Instant::now());
    let time_left = use_state(|| data.time_left.to_owned());
    Interval::new(Duration::from_secs(1));
    let interval = Interval::new(1000, move || {
        if (*last_time).elapsed().as_secs() >= 1 {
            time_left.set(*time_left - (*last_time).elapsed().as_secs());
            last_time.set(Instant::now());
        }
    });
    html! {
        <h2 class="timer_label">{ TimerData::format_time_left(*time_left) }</h2>
    }
}


#[derive(PartialEq)]
pub enum CsState {
    NotStarted,
    Started,
    Flex
}

#[derive(Properties, PartialEq)]
pub struct CsData {
    pub destroy_card_callback:Callback<usize>,
    #[prop_or_default]
    pub cycle_started:bool,
    #[prop_or(1800)]
    pub current_cycle_length:u16,
}

#[function_component]
pub fn CsCycle(data:&CsData) -> Html {
    let state = use_state(|| CsState::NotStarted);
    let destroy_card = data.destroy_card_callback.clone();
    let start_cycle = {
        let state = state.clone();
        Callback::from(move |_| state.set(CsState::Started))
    };
    let stop_cycle = Callback::from(move |_| destroy_card.emit(0));
    html! {
        <>
            <div class="timer">
            <Timer active={ *state == CsState::Started }/>
            if *state == CsState::NotStarted {
                <button class="button" onclick={start_cycle}>{ "Start Cycle" }</button>
            }
            else {
                <button class="button outlined" onclick={stop_cycle}>{ "Stop Cycle" }</button>
            }
            </div>
            <hr/>
            if *state == CsState::NotStarted {
                <p>{ "The CS cycle begins with brewing coffee. Brew some coffee and click " }<b>{ "Start Cycle" }</b>{ " to begin." }</p>
            } else {
                <Checkbox text="Brew Coffee" default_value={true} />
                <Checkbox text="Cafe Check"/>
                <Checkbox text="Flex"/>
            }
        </>
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CardType {
    StartNewTask,
    Info,
    AboutUs,
    CsCycle
}


#[derive(Properties, PartialEq, Clone)]
pub struct CardData {
    #[prop_or(CardType::StartNewTask)]
    pub card_type:CardType,
    pub index:usize,
    pub create_card:Callback<CardType>,
    pub set_priority_card:Callback<CardType>,
    pub destroy_card:Callback<usize>,
}

impl CardData {
    pub fn new(card_type:CardType) -> Self {
        CardData { card_type, index:0, create_card:Callback::noop(), set_priority_card:Callback::noop(), destroy_card:Callback::noop() }
    }

    pub fn get_title(&self) -> String {
        match self.card_type {
            CardType::StartNewTask => "What can I help you with today?".to_string(),
            CardType::Info => "Information".to_string(),
            CardType::AboutUs => "About Star".to_string(),
            CardType::CsCycle => "".to_string(),
            _ => "Invalid Card".to_string(),
        }
    }

    pub fn is_priority(&self) -> bool {
        return self.card_type == CardType::CsCycle
    }

    pub fn get_content(&self) -> Html {
        match self.card_type {
            CardType::StartNewTask => {
                let create_cs_cycle = {
                    let set_priority_card = self.set_priority_card.clone();
                    Callback::from(move |_| {
                        set_priority_card.emit(CardType::CsCycle);
                    })
                };
                let create_about_us = {
                    let create_card = self.create_card.clone();
                    Callback::from(move |_| {
                    create_card.emit(CardType::AboutUs);
                    })
                };
                html! {
                    <div class="card-multioption">
                    <a class="card-multioption_button" onclick={ create_cs_cycle }>
                    <span class="icon material-icons">{ "update" }</span>
                    { "Start a new CS cycle" }
                    </a>
                    <a class="card-multioption_button" onclick={ create_about_us }>
                    <span class="icon material-icons">{ "account_circle" }</span>
                    { "About Us" }
                    </a>
                    </div>
                }
            },
            CardType::Info => html! {
                <img src="https://www.w3schools.com/tags/img_girl.jpg" />
            },
            CardType::AboutUs => html! {
                <p>{ "Made mainly as a side project, Star is a tool to help Starbucks baristas with their daily tasks. Made by a barista, for baristas. Star is dedicated to improving the partner experience."}</p>
            },
            CardType::CsCycle => {
                html! { <CsCycle destroy_card_callback={self.destroy_card.clone()}/> }
            },
            _ => {
                html! {
                    <></>
                }
            }
        }
    }
}

#[function_component]
pub fn Card(data:&CardData) -> Html {
    html! {
        if data.is_priority() {
            <div class="card filled">
                <h2 class="title">{ data.get_title() }</h2>
                { data.get_content() }
            </div>
        } else {
        <div class="card elevated">
            <h2 class="title">{ data.get_title() }</h2>
            { data.get_content() }
        </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct CheckboxData {
    #[prop_or("Checkbox".to_string())]
    pub text:String,
    #[prop_or_default]
    pub default_value:bool,
    #[prop_or(true)]
    pub is_list_item:bool,
}

#[function_component]
pub fn Checkbox(data:&CheckboxData) -> Html {
    let state = use_state(|| data.default_value);

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(!*state)
        })
    };
    html! {
        <a class="checkbox" onclick={onclick}>
            if *state {
                <span class=" material-icons checkbox_radio">
                    { "check_circle" }
                </span>
            }
            else {
                <span class="material-icons checkbox_radio">
                    { "radio_button_unchecked" }
                </span>
            }
            <span class={ if data.is_list_item && *state { "checkbox_striked" } else { "" } }>
            { data.text.clone() }
            </span>
        </a>
    }
}