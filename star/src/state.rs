use std::{rc::Rc, collections::VecDeque, cell::Cell};

use chrono::{DateTime, Utc};
use gloo::storage::{Storage, LocalStorage, errors::StorageError};
use serde::{Serialize, Deserialize};
use storage::StorableData;
use web_sys::{window, HtmlAudioElement};
use gloo::timers::callback::Interval;
use wasm_timer::Instant;
use yew::prelude::*;

use crate::*;

// Star
#[derive(Debug)]
pub enum StarAction {
    AddCard(CardType),
    AddPriorityCard(CardType),
    DestroyPriorityCard(usize),
    DestroyCard(usize),
}

#[derive(Properties, PartialEq)]
pub struct StarData {
    pub priority_cards:VecDeque<CardData>,
    pub cards:VecDeque<CardData>,
}

impl StorableData for StarData {
    fn load() -> Self {
        let mut priority_cards:Option<VecDeque<CardData>> = None;
        let mut cards:Option<VecDeque<CardData>> = None;

        let cards_from_storage:Result<VecDeque<CardData>, gloo::storage::errors::StorageError> = LocalStorage::get("cards");
        if let Ok(stored_cards) = cards_from_storage {
            cards = Some(stored_cards)
        }

        let priority_cards_from_storage:Result<VecDeque<CardData>, gloo::storage::errors::StorageError> = LocalStorage::get("priority_cards");
        match priority_cards_from_storage {
            Ok(stored_cards) => priority_cards = Some(stored_cards),
            _ => {}
        }
        

        StarData { priority_cards: priority_cards.unwrap_or_default(), cards: cards.unwrap_or_else(|| VecDeque::from([CardData::new(CardType::StartNewTask)])) }
    }

    fn save(self) -> Self {
        if let Err(err) = LocalStorage::set("cards", self.cards.clone()) {
            log::error!("{:?}", err);
        }
        if let Err(err) = LocalStorage::set("priority_cards", self.priority_cards.clone()) {
            log::error!("{:?}", err);
        }
        self
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
                    cards.push_back(CardData::new(card_type));
                }
                StarData {
                    priority_cards: self.priority_cards.clone(),
                    cards,
                }.save().into()
            },
            StarAction::AddPriorityCard(card_type) => {
                let mut priority_cards = self.priority_cards.clone();
                let mut has_card = false;
                for card in &priority_cards {
                    if card.card_type == card_type {
                        has_card = true;
                        break;
                    }
                }
                if !has_card {
                    priority_cards.push_front(CardData::new(card_type));
                }
                if let Some(window) = window() {
                    window.scroll_with_x_and_y(0.0, 0.0);
                }
                StarData {
                    priority_cards,
                    cards: self.cards.clone(),
                }.save().into()
            },
            StarAction::DestroyPriorityCard(index) => {
                let mut priority_cards = self.priority_cards.clone();
                if priority_cards.get(index).is_some() {
                    priority_cards.remove(index);
                }
                StarData {
                    priority_cards,
                    cards: self.cards.clone(),
                }.save().into()
            },
            StarAction::DestroyCard(index) => {
                let mut cards = self.cards.clone();
                if cards.get(index).is_some() {
                    cards.remove(index);
                }
                StarData {
                    priority_cards: self.priority_cards.clone(),
                    cards,
                }.save().into()
            },
            _ => {
                StarData {
                    priority_cards: self.priority_cards.clone(),
                    cards: self.cards.clone(),
                }.into()
            }
        }
    }
}

// Task list
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub task: String,
    pub completed: bool,
}

impl Task {
    pub fn new(task:&'static str) -> Task {
        Task { task: String::from(task), completed: false }
    }
    pub fn complete(&mut self) {
        self.completed = true;
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Daypart {
    Opening,
    Mid,
    Closing,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DaypartTasks {
    pub daypart:Daypart,
    pub daypart_tasks: Vec<Task>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: [DaypartTasks; 3],
    pub date_of_use: DateTime<Utc>,
}

impl Default for Tasks {
    fn default() -> Self {
        Tasks {
            tasks: [
                DaypartTasks { daypart: Daypart::Opening, daypart_tasks: vec![] },
                DaypartTasks { daypart: Daypart::Mid, daypart_tasks: vec![
                        Task::new("Floors"),
                        Task::new("Ovens"),
                        Task::new("18 Hour Pull"),
                        Task::new("Safe Count"),
                        Task::new("Mid Temps"),
                    ]
                },
                DaypartTasks { daypart: Daypart::Closing, daypart_tasks: vec![
                        Task::new("Bar Syrups"),
                        Task::new("Bar Breakdowns"),
                        Task::new("Closing Dishes"),
                        Task::new("Backups"),
                        Task::new("Throw out expiring backups"),
                        Task::new("Bag & donate expiring food"),
                        Task::new("Temps"),
                        Task::new("Tills"),
                    ]
                }
            ],
            date_of_use: Utc::now(),
        }
    }
}

impl StorableData for Tasks {
    fn load() -> Self {
        let storage:Result<Tasks, StorageError> = LocalStorage::get("daily_tasks");
        match storage {
            Ok(tasks) => {
                if tasks.date_of_use.date() == Utc::today() {
                    tasks
                }
                else {
                    Self::default()
                }
            },
            Err(err) => {
                log::error!("{:?}", err);
                Self::default()
            }
        }
    }

    fn save(self) -> Self {
        LocalStorage::set("daily_tasks", self.clone());
        self
    }
}

impl Daypart {
    pub fn to_string(self) -> &'static str {
        match self {
            Daypart::Opening => "Opening",
            Daypart::Mid => "Mid-day",
            Daypart::Closing => "Closing",
        }
    }

    pub fn get_time_period(&self) -> &'static str {
        match self {
            Daypart::Opening => "Open-12pm",
            Daypart::Mid => "12-4pm",
            Daypart::Closing => "4pm-Close",
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
                (*time_left).set(st - 1);
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