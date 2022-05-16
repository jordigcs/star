use std::{rc::Rc, collections::VecDeque, borrow::BorrowMut, cell::Cell};

use web_sys::{Window, window, HtmlAudioElement};
use gloo_timers::callback::Interval;
use wasm_timer::Instant;
use yew::prelude::*;

use crate::*;

// Star
#[derive(Debug)]
pub enum StarAction {
    AddCard(CardType),
    SetPriorityCard(CardType),
    ClearPriorityCard(),
    DestroyCard(usize),
}

#[derive(Properties, PartialEq)]
pub struct StarData {
    pub priority_card:Option<CardData>,
    pub cards:VecDeque<CardData>,
}

impl StarData {
    pub fn new() -> Self {
        let cards = [
            CardData::new(CardType::StartNewTask)];
        StarData { priority_card:None, cards:VecDeque::from(cards) }
    }
}

impl Reducible for StarData {
    type Action = StarAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            StarAction::AddCard(card_type) => {
                let mut cards = self.cards.clone();
                let mut has_card = false;
                for card in &cards {
                    if card.card_type == card_type {
                        has_card = true;
                        break;
                    }
                }
                if !has_card {
                    cards.insert(1, CardData::new(card_type));
                }
                StarData {
                    priority_card: self.priority_card.clone(),
                    cards,
                }.into()
            },
            StarAction::SetPriorityCard(card_type) => {
                StarData {
                    priority_card: Some(CardData::new_priority(card_type)),
                    cards: self.cards.clone(),
                }.into()
            },
            StarAction::ClearPriorityCard() => {
                StarData {
                    priority_card: None,
                    cards: self.cards.clone(),
                }.into()
            },
            StarAction::DestroyCard(index) => {
                let mut cards = self.cards.clone();
                if let Some(_) = cards.get(index) {
                    cards.remove(index);
                }
                StarData {
                    priority_card: self.priority_card.clone(),
                    cards,
                }.into()
            },
            _ => {
                StarData {
                    priority_card: self.priority_card.clone(),
                    cards: self.cards.clone(),
                }.into()
            }
        }
    }
}

// Timer
pub enum TimerAction {
    Start(i32),
    Stop,
    SetCallback(Callback<u16>)
}

#[derive(Properties, PartialEq, Clone)]
pub struct TimerData {
    pub time_left:UseStateHandle<Cell<i32>>, // Seconds,
    #[prop_or_default]
    pub running:bool,
    #[prop_or_default]
    pub callback:Callback<u16>,
    #[prop_or_default]
    pub timer_interval_id:i32,
    #[prop_or(HtmlAudioElement::new().unwrap())]
    pub timer_sound:HtmlAudioElement,
}

pub enum TimerParseError {
    InvalidInput
}

impl TimerData {
    pub fn format_time_left(mut time_left: i32) -> String {
        let mut is_negative:bool = false;
        if time_left < 0 {
            time_left = time_left.abs();
            is_negative = true;
        }
        let minutes = time_left / 60;
        let seconds = time_left - (minutes * 60);
        (if is_negative { "-" } else { "" }).to_owned() + &format!("{:02}:{:02}", minutes, seconds)
    }

    
    pub fn seconds_from_str(time:String) -> Result<u16, TimerParseError>  {
        let split_time:Vec<&str> = time.split(':').collect();
        let mut minutes: u16 = 0;
        let mut seconds: u16 = 0;
        if split_time.len() > 1 { // Time has minutes
            // Parse Minutes
            if let Ok(val) = split_time[0].parse::<u16>() {
                minutes = val;
            }
            else {
                return Err(TimerParseError::InvalidInput);
            }
            // Parse seconds
            if let Ok(val) = split_time[1].parse::<u16>() {
                seconds = val;
            }
            else {
                return Err(TimerParseError::InvalidInput);
            }
        }
        else {
            // Parse seconds
            if let Ok(val) = split_time[0].parse::<u16>() {
                seconds = val;
            }
            else {
                return Err(TimerParseError::InvalidInput);
            }
        }
        Ok((minutes * 60) + seconds)
    }

    pub fn stop(&self) {
        if self.timer_interval_id > -1 {
            if let Some(window) = window() {
                window.clear_interval_with_handle(self.timer_interval_id);
            }
        }
    }
}

impl Reducible for TimerData {
    type Action = TimerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TimerAction::Start(st) => {
                let mut last_time = Instant::now();
                let time_left = self.time_left.clone();
                (*time_left).set((st - 1).into());
                let timer_interval = Interval::new(350, move || {
                    if last_time.elapsed().as_secs() >= 1 {
                        (*time_left).set((*time_left).get() - last_time.elapsed().as_secs() as i32);
                        time_left.set((*time_left).clone());
                        last_time = Instant::now();
                    }
                });
                let id = timer_interval.forget();

                Self {
                    time_left:self.time_left.clone(),
                    running: true,
                    callback:self.callback.clone(),
                    timer_interval_id: id,
                    timer_sound: self.timer_sound.clone()
                }.into()
            },
            TimerAction::Stop => {
                self.stop();
                Self {
                    time_left:self.time_left.clone(),
                    running: false,
                    callback:self.callback.clone(),
                    timer_interval_id: -1,
                    timer_sound: self.timer_sound.clone()
                }.into()
            },
            TimerAction::SetCallback(callback) => {
                Self {
                    time_left: self.time_left.clone(),
                    running: self.running,
                    callback,
                    timer_interval_id: self.timer_interval_id,
                    timer_sound: self.timer_sound.clone()
                }.into()
            }
        }
    }
}