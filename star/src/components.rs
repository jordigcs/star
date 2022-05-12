use std::{cell::Cell, rc::Rc};

use web_sys::{window, HtmlAudioElement, HtmlElement, HtmlInputElement};
use yew::{prelude::*, props};
use gloo_timers::callback::Interval;
use wasm_logger::*;
use wasm_timer::Instant;
use crate::state::{TimerData, TimerAction};

#[function_component]
pub fn Timer(data:&TimerData) -> Html {
    log::warn!("Render");
    if data.time_left.get() <= 0 {
        if data.timer_sound.paused() {
            data.timer_sound.play();
        }
    }
    html! {
        <>
        <h2 class={ "timer_label".to_owned() + (if data.time_left.get() < 0 { " timer_expired"} else { "" })}>{ 
            TimerData::format_time_left(data.time_left.get()) 
        }</h2>
        </>
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
    let timer_data = TimerData {
        time_left:use_state(|| Cell::new(2)),
        running: false,
        callback:Callback::noop(),
        timer_interval_id: -1,
        timer_sound: HtmlAudioElement::new_with_src("src").expect("Could not load timer sound.")
    };
    let state = use_state(|| CsState::NotStarted);
    let timer_state = use_reducer(|| timer_data);
    
    let start_time_value = use_state(|| 1800);
    let start_time_input_str = use_state(|| TimerData::format_time_left(*start_time_value));
    let start_time_input_ref = use_node_ref();

    let start_cycle = {
        let state = state.clone();
        let timer_state = timer_state.clone();
        let start_time_value = start_time_value.clone();
        Callback::from(move |_| {
            timer_state.dispatch(TimerAction::Start(*start_time_value));
            state.set(CsState::Started)
        })
    };

    let destroy_card = data.destroy_card_callback.clone();
    let stop_cycle = {
        let state = state.clone();
        let timer_state = timer_state.clone();
        Callback::from(move |_| {
            timer_state.dispatch(TimerAction::Stop);
            state.set(CsState::NotStarted);
        })
    };

    let start_time_changed = {
        let timer_state = timer_state.clone();
        let start_time_input_ref = start_time_input_ref.clone();
        let start_time_value = start_time_value.clone();
        let start_time_input_str = start_time_input_str.clone();
        Callback::from(move |_| {
            let input = start_time_input_ref.cast::<HtmlInputElement>().expect("Timer not initialized correctly.");
            let mut formatted_input_value:String = String::new();
            for c in input.value().chars().into_iter() {
                if !c.is_numeric() && c != ':' {
                    continue;
                }
                formatted_input_value.push(c);
            }
            if let Ok(start_time) = TimerData::seconds_from_str(formatted_input_value) {
                start_time_value.set(start_time.into());
                start_time_input_str.set(TimerData::format_time_left(start_time.into()));
            }
            else {
                start_time_input_str.set(String::new());
            }
        })
    };

    html! {
        <>
            <div class="timer">
            if *state == CsState::NotStarted {
                <input ref={ start_time_input_ref } onchange={ start_time_changed } value={ (*start_time_input_str).clone() }class="timer_input" size="1" type="text" />
                <button class="button" onclick={start_cycle}><span class="material-icons icon">{ "update" }</span>{ " Start Cycle" }</button>
            }
            else {
                <Timer time_left={ timer_state.time_left.clone() } />
                <button class="button outlined" onclick={stop_cycle}>{ "Stop Cycle" }</button>
            }
            </div>
            <hr/>
            if *state == CsState::NotStarted {
                <p>{ "The CS cycle begins with brewing coffee. Brew some coffee and click " }<b>{ "Start Cycle" }</b>{ " to begin." }</p>
            } else {
                <p><b>{ "Tasks" }</b></p>
                <Checkbox text="Brew Coffee" default_value={true} />
                <Checkbox text="Cafe Check"/>
                <button class="button outlined" ><span class="material-icons">{ "add" }</span>{ " Schedule a new task" }</button>
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