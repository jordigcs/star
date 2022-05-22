use std::{cell::Cell};

use chrono::{Date, Local, Datelike, Weekday, Duration, Timelike};
use serde::{Serialize, Deserialize};
use web_sys::{HtmlAudioElement, HtmlInputElement};
use yew::{prelude::*};



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

#[derive(Properties, PartialEq, Serialize, Deserialize)]
pub struct CsData {
    #[serde(skip)]
    pub destroy_card_callback:Callback<usize>,
    #[prop_or_default]
    pub cycle_started:bool,
    #[prop_or(1800)]
    pub current_cycle_length:u16,
}

#[derive(PartialEq)]
pub enum CoffeeRoast {
    Pike,
    Blonde,
    Dark
}

impl CoffeeRoast {
    pub fn to_string(&self) -> String {
        match self {
            CoffeeRoast::Blonde => {
                String::from("Blonde")
            },
            CoffeeRoast::Dark => {
                String::from("Dark")
            }
            CoffeeRoast::Pike => {
                String::from("Pike")
            },
        }
    }
}

pub struct CoffeesToBrew(CoffeeRoast, Option<CoffeeRoast>);

impl CoffeesToBrew {
    pub fn to_string(&self) -> String {
        let mut val = self.0.to_string() + ", ";
        if let Some(roast) = &self.1 {
            val.push_str(&roast.to_string());
        }
        val
    }

    pub fn get_next(&self) -> CoffeesToBrew {
        let mut c = CoffeesToBrew(CoffeeRoast::Pike, None);
        if chrono::Local::now().hour() < 11 {
            if let Some(roast) = &self.1 {
                match roast {
                    CoffeeRoast::Blonde => {
                        c.1 = Some(CoffeeRoast::Dark);
                    },
                    _ => {
                        c.1 = Some(CoffeeRoast::Blonde);
                    }
                }
            }
        }
        c
    }
}

#[function_component]
pub fn CsCycle() -> Html {
    let state = use_state(|| CsState::NotStarted);

    let last_brewed = use_state(|| CoffeesToBrew(CoffeeRoast::Pike, 
        if chrono::Local::now().hour() < 11 { Some(CoffeeRoast::Blonde) } else { None }));

    //Timer initialization
    let start_time_value = use_state(|| 1800);
    let start_time_input_str = use_state(|| TimerData::format_time_left(*start_time_value));
    let start_time_input_ref = use_node_ref();
    let timer_data = TimerData {
        time_left:use_state(|| Cell::new(2)),
        running: false,
        callback:Callback::noop(),
        timer_interval_id: -1,
        timer_sound: HtmlAudioElement::new_with_src("src").expect("Could not load timer sound.")
    };
    let timer_state = use_reducer(|| timer_data);

    // Callbacks
    let start_cycle = {
        let state = state.clone();
        let timer_state = timer_state.clone();
        let start_time_value = start_time_value.clone();
        let last_brewed = last_brewed.clone();
        Callback::from(move |_| {
            timer_state.dispatch(TimerAction::Start(*start_time_value));
            last_brewed.set((*last_brewed).get_next());
            state.set(CsState::Started)
        })
    };
    
    let stop_cycle = {
        let state = state.clone();
        let timer_state = timer_state.clone();
        Callback::from(move |_| {
            timer_state.dispatch(TimerAction::Stop);
            state.set(CsState::NotStarted);
        })
    };

    let start_time_changed = {
        let _timer_state = timer_state.clone();
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
    let next_to_brew = (*last_brewed).get_next();
    html! {
        <>
            <div class="timer">
            if *state == CsState::NotStarted {
                <input ref={ start_time_input_ref } onchange={ start_time_changed } value={ (*start_time_input_str).clone() } class="timer_input" size="1" type="text" />
                <button class="button" onclick={start_cycle}><span class="material-symbols-outlined icon">{ "update" }</span>{ " Start Cycle" }</button>
            }
            else {
                <Timer time_left={ timer_state.time_left.clone() } />
                <button class="button outlined" onclick={stop_cycle}>{ "Stop Cycle" }</button>
            }
            </div>
            <hr/>
            if *state == CsState::NotStarted {
                <p>{ "The CS cycle begins with brewing coffee. Brew some coffee and click " }<b>{ "Start Cycle" }</b>{ " to begin." }</p>
                <hr/>
                if next_to_brew.1 != None {
                    <p><b>{ "Next coffee to brew:" }</b><br/>{
                        next_to_brew.0.to_string() +
                        &(if let Some(p) = next_to_brew.1 {
                            " & ".to_owned() + &p.to_string()
                        } else {
                            "".to_owned()
                        })
                        }
                    </p>
                }
            } else {
                <p><b>{ "Tasks" }</b></p>
                <Checkbox text="Brew Coffee" default_value={true} />
                <Checkbox text="Cafe Check"/>
                <button class="button outlined" ><span class="material-symbols-outlined">{ "add" }</span>{ " Schedule a new task" }</button>
                if next_to_brew.1 != None {
                    <hr/>
                    <p><b>{ "Last coffee brewed:" }</b><br/>{
                        (*last_brewed).0.to_string() +
                        &(if let Some(p) = &(*last_brewed).1 {
                            " & ".to_owned() + &p.to_string()
                        } else {
                            "".to_owned()
                        })
                    }</p>
                }
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct DaydotData {
    date:Date<Local>
}

#[function_component]
pub fn Daydot(data:&DaydotData) -> Html {
    let weekday = data.date.weekday();
    let mut weekday_str:String = String::new();
    match weekday {
        Weekday::Mon => {
            weekday_str.push_str("Monday");
        },
        Weekday::Tue => {
            weekday_str.push_str("Tuesday");
        },
        Weekday::Wed => {
            weekday_str.push_str("Wednesday");
        },
        Weekday::Thu => {
            weekday_str.push_str("Thursday");
        }
        Weekday::Fri => {
            weekday_str.push_str("Friday");
        }
        Weekday::Sat => {
            weekday_str.push_str("Saturday");
        }
        Weekday::Sun => {
            weekday_str.push_str("Sunday");
        }
    }

    let date = data.date.month().to_string() + "/" + &data.date.day().to_string();
    html! {
        <div class="daydot">
        <div class={ "daydot_day ".to_owned() + &weekday_str.to_lowercase() }>
        <b><p>{ weekday_str }</p></b>
        </div>
        <p>{ date }</p>
        </div>
    }
}

#[derive(PartialEq, Clone)]
pub struct DaydotProduct(String, u8);

#[derive(Properties, PartialEq)]
pub struct DaydotCardData {
    pub hb_products:Vec<DaydotProduct>,
    pub cbs_products:Vec<DaydotProduct>
}

#[function_component]
pub fn Daydots(data:&DaydotCardData) -> Html {
    let today = Local::today();
    let cbs_dates = use_state(|| {
        let mut c = Vec::new();
        for product in &data.cbs_products {
            c.push(
                html! {
                    <div class="date_card">
                    <h3>{ product.0.clone() }</h3>
                    <Daydot date={ today.clone() + Duration::days(product.1.into())} />
                    </div>
                }
            );
        }
        c
    });
    let hb_dates = use_state(|| {
        let mut h = Vec::new();
        for product in &data.hb_products {
            h.push(
                html! {
                    <div class="date_card">
                    <h3>{ product.0.clone() }</h3>
                    <Daydot date={ today.clone() + Duration::days(product.1.into())} />
                    </div>
                }
            );
        }
        h
    });

    let hb_dates_shown = use_state(|| false);
    let cbs_dates_shown = use_state(|| false);
    
    let toggle_hb_dates_shown = {
        let hb_dates_shown = hb_dates_shown.clone();
        Callback::<MouseEvent>::from(move |_| {
            hb_dates_shown.set(!(*hb_dates_shown));
        })
    };

    let toggle_cbs_dates_shown = {
        let cbs_dates_shown = cbs_dates_shown.clone();
        Callback::<MouseEvent>::from(move |_| {
            cbs_dates_shown.set(!(*cbs_dates_shown));
        })
    };

    let search_results = use_state(|| Vec::<Html>::new());
    let search_results_ref = use_node_ref();
    let search_results_changed = {
        let search_results_ref = search_results_ref.clone();
        let search_results = search_results.clone();
        let h = data.hb_products.clone();
        let c = data.cbs_products.clone();
        Callback::<InputEvent>::from(move |_| {
            let search = search_results_ref.cast::<HtmlInputElement>().expect("Search not found.");
            if search.value().len() > 0 {
                let mut joined_vec = h.clone();
                joined_vec.append(&mut c.clone());

                let mut results = Vec::<DaydotProduct>::new();

                for p in joined_vec {
                    let p_name = p.0.to_lowercase();
                    let split = p_name.split(" ");
                    for name in split {
                        if name.starts_with(&search.value().to_lowercase()) {
                            let mut product_already_exists:bool = false;
                            for i in &results {
                                if i.0 == p.0 {
                                    product_already_exists = true;
                                    break;
                                }
                            }
                            if product_already_exists {
                                continue;
                            }
                            results.push(p);
                            break;
                        }
                    }
                }

                let mut h = Vec::new();
                for product in results {
                    h.push(
                        html! {
                            <div class="date_card">
                            <h3>{ product.0.clone() }</h3>
                            <Daydot date={ today.clone() + Duration::days(product.1.into())} />
                            </div>
                        }
                    );
                }
                search_results.set(h);
            }
            else {
                search_results.set(Vec::new());
            }
    })
    };
    html! {
        <>
        <h2 class="title_white">{ "Daydots" }</h2>
        <p><b>{ "Today is "}</b><Daydot date={chrono::Local::today()} /></p>
        <span class="material-symbols-outlined" style="font-size:1.5rem; display:inline;">{ "search" }</span><input ref={search_results_ref} oninput={search_results_changed} class="text_input" size="1" type="text" placeholder="Search" />
        if (*search_results).len() > 0 {
            <h3 class="" >{"Search Results"}</h3>
            <div class="date_grid">
            {
                (*search_results).clone()
            }
            </div>
            <hr />
        }
        <h2 class="clickable" onclick={toggle_hb_dates_shown} >{"Hot Bar "} <span class="material-symbols-outlined">{ "coffee" }</span><span class="material-symbols-outlined">{ if *hb_dates_shown { "expand_less" } else { "expand_more" } }</span></h2>
        if *hb_dates_shown {
            <div class="date_grid">
            {
                (*hb_dates).clone()
            }
            </div>
        }
        <h2 class="clickable" onclick={toggle_cbs_dates_shown} >{"Cold Bar "} <span class="material-symbols-outlined">{ "blender" }</span><span class="material-symbols-outlined">{ if *cbs_dates_shown { "expand_less" } else { "expand_more" } }</span></h2>
        if *cbs_dates_shown {
            <div class="date_grid">
            {
                (*cbs_dates).clone()
            }
            </div>
        }
        </>
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CardType {
    StartNewTask,
    Info,
    AboutUs,
    Daydots,
    CsCycle
}


#[derive(Properties, PartialEq, Clone)]
pub struct CardData {
    #[prop_or(CardType::StartNewTask)]
    pub card_type:CardType,
    #[prop_or_default]
    pub is_priority:bool,
    pub index:usize,
    pub create_card:Callback<CardType>,
    pub add_priority_card:Callback<CardType>,
    pub destroy_priority:Callback<usize>,
    pub destroy_card:Callback<usize>,
}

impl CardData {
    pub fn new(card_type:CardType) -> Self {
        CardData { card_type, is_priority: false, index:0, create_card:Callback::noop(), add_priority_card:Callback::noop(), destroy_priority:Callback::noop(), destroy_card:Callback::noop() }
    }

    pub fn new_priority(card_type:CardType) -> Self {
        let mut c = Self::new(card_type);
        c.is_priority = true;
        c
    }

    pub fn get_title(&self) -> String {
        match self.card_type {
            CardType::StartNewTask => "What can I help you with today?".to_string(),
            CardType::CsCycle => String::new(),
            CardType::Info => "Information".to_string(),
            CardType::AboutUs => "About Star".to_string(),
            CardType::Daydots => String::new(),
            _ => "Invalid Card".to_string(),
        }
    }

    pub fn get_content(&self) -> Html {
        match self.card_type {
            CardType::StartNewTask => {
                let create_cs_cycle = {
                    let set_priority_card = self.add_priority_card.clone();
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
                let create_daydot_card = {
                    let set_priority_card = self.add_priority_card.clone();
                    Callback::from(move |_| {
                        set_priority_card.emit(CardType::Daydots);
                    })
                };

                html! {
                    <div class="card-multioption">
                    <a class="card-multioption_button" onclick={ create_cs_cycle }>
                    <span class="icon material-symbols-outlined">{ "update" }</span>
                    { "Start a new CS cycle" }
                    </a>
                    <a class="card-multioption_button" onclick={ create_daydot_card }>
                    <span class="icon material-symbols-outlined">{ "event" }</span>
                    { "Daydot some backups" }
                    </a>
                    <a class="card-multioption_button" onclick={ create_about_us }>
                    <span class="icon material-symbols-outlined">{ "account_circle" }</span>
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
                html! { <CsCycle/> }
            },
            CardType::Daydots => {
                let hb_products = vec!(
                    DaydotProduct(String::from("Mocha"), 1 ),
                    DaydotProduct(String::from("White Mocha"), 14),
                    DaydotProduct(String::from("Chai"), 1 ),
                    DaydotProduct(String::from("Whipped Cream"), 1 ),
                );
                let cb_products = vec!(
                    DaydotProduct(String::from("Refresher Base"), 3),
                    DaydotProduct(String::from("Refresher Fruit Inclusions"), 5),
                    DaydotProduct(String::from("Lemonade"), 2),
                    DaydotProduct(String::from("Vanilla Sweet Cream"), 2 ),
                    DaydotProduct(String::from("Whipped Cream"), 1 ),
                    DaydotProduct(String::from("Frap Roast"), 2 ),
                    DaydotProduct(String::from("Frap Chips"), 7),
                    DaydotProduct(String::from("Mocha"), 1),
                    DaydotProduct(String::from("Cold Brew"), 7),
                    DaydotProduct(String::from("Powder Inclusions"), 7),
                    DaydotProduct(String::from("Caramel Drizzle"), 14),
                );
                html! {
                    <Daydots hb_products={hb_products} cbs_products={cb_products} />
                }
            }
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
    let destroy_card = {
        let index = data.index.clone();
        let destroy_card = data.destroy_card.clone();
        Callback::from(move |_| {
            destroy_card.emit(index);
        })
    };
    let destroy_priority = {
        let index = data.index.clone();
        let destroy_priority = data.destroy_priority.clone();
        Callback::from(move |_| {
            destroy_priority.emit(index);
        })
    };

    html! {
        if data.is_priority {
            <div class="card filled">
                <a class="card_close_button" onclick={destroy_priority}><span class="material-symbols-outlined">{ "close" }</span></a>
                <h2 class="title">{ data.get_title() }</h2>
                { data.get_content() }
            </div>
        } else {
        <div class="card elevated">
            if data.card_type != CardType::StartNewTask {
                <a class="card_close_button" onclick={destroy_card}><span class="material-symbols-outlined">{ "close" }</span></a>
            }
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
                <span class=" material-symbols-outlined checkbox_radio">
                    { "check_circle" }
                </span>
            }
            else {
                <span class="material-symbols-outlined checkbox_radio">
                    { "radio_button_unchecked" }
                </span>
            }
            <span class={ if data.is_list_item && *state { "checkbox_striked" } else { "" } }>
            { data.text.clone() }
            </span>
        </a>
    }
}