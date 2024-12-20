use std::fmt::Display;

use card::Card;
use serde::{Deserialize, Serialize};
use step::Step;

pub mod card;
pub mod step;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Job {
    card: Card,
    steps: Vec<Step>,
}

impl Job {
    pub fn new<T>(name: T) -> Job
    where
        T: Display,
    {
        Job {
            card: Card::new(name),
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: Step) -> &mut Self {
        self.steps.push(step);

        self
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.card.fmt(f)?;

        for step in self.steps.iter() {
            step.fmt(f)?;
        }

        Ok(())
    }
}
