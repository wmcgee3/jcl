use std::fmt::Display;

use dd::DdGroup;
use serde::{Deserialize, Serialize};

pub mod dd;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Step {
    name: String,
    program: String,
    dd_groups: Vec<DdGroup>,
}

impl Step {
    pub fn new<T, U>(name: T, program: U) -> Step
    where
        T: Display,
        U: Display,
    {
        Step {
            name: name.to_string(),
            program: program.to_string(),
            dd_groups: Vec::new(),
        }
    }

    pub fn add_dd_group(&mut self, dd_group: DdGroup) -> &mut Self {
        self.dd_groups.push(dd_group);

        self
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "//{:8} EXEC PGM={}", self.name, self.program)?;

        for dd_group in self.dd_groups.iter() {
            dd_group.fmt(f)?;
        }

        writeln!(f, "//*")
    }
}
