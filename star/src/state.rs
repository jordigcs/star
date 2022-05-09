use std::{rc::Rc, collections::VecDeque};

use yew::prelude::*;

use crate::*;

#[derive(Debug)]
pub enum StarAction {
    AddCard(CardType),
    SetPriorityCard(CardType),
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
                cards.insert(1, CardData::new(card_type),);
                StarData {
                    priority_card: self.priority_card.clone(),
                    cards,
                }.into()
            },
            StarAction::SetPriorityCard(card_type) => {
                StarData {
                    priority_card: Some(CardData::new(card_type)),
                    cards: self.cards.clone(),
                }.into()
            },
            StarAction::DestroyCard(index) => {
                let mut cards = self.cards.clone();
                let mut priority_card = self.priority_card.clone();
                let mut removed_card:bool = false;
                if let Some(card) = &self.priority_card {
                    if index == 0 {
                        priority_card = None;
                        removed_card = true;
                    }
                }
                if !removed_card {
                    if let Some(_) = cards.get(index) {
                        cards.remove(index);
                    }
                }
                StarData {
                    priority_card: priority_card,
                    cards: cards,
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