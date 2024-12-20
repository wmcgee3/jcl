use serde::{Deserialize, Serialize};

pub mod system;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Card {
    name: String,
    accounts: Option<Vec<String>>,
    programmer: Option<String>,
}

impl Card {
    pub fn new(name: impl ToString) -> Card {
        Card {
            name: name.to_string(),
            accounts: None,
            programmer: None,
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(accounts) = &self.accounts {
            params.push(format!("({})", accounts.join(",")));
        }
        if let Some(programmer) = &self.programmer {
            params.push(format!("'{}'", programmer));
        }

        writeln!(f, "//{:8} JOB {}", self.name, params.join(","))?;

        writeln!(f, "//*")
    }
}

pub struct CardBuilder {
    name: String,
    accounts: Option<Vec<String>>,
    programmer: Option<String>,
}

impl CardBuilder {
    pub fn new(name: impl ToString) -> Self {
        CardBuilder {
            name: name.to_string(),
            accounts: None,
            programmer: None,
        }
    }

    pub fn account(&mut self, account: impl ToString) -> &mut Self {
        match &mut self.accounts {
            Some(acct) => acct.push(account.to_string()),
            None => self.accounts = Some(vec![account.to_string()]),
        }

        self
    }

    pub fn accounts(&mut self, accounts: &[impl ToString]) -> &mut Self {
        self.accounts = Some(accounts.iter().map(|a| a.to_string()).collect());

        self
    }

    pub fn programmer(&mut self, programmer: impl ToString) -> &mut Self {
        self.programmer = Some(programmer.to_string());

        self
    }

    pub fn build(self) -> Card {
        Card {
            name: self.name,
            accounts: self.accounts,
            programmer: self.programmer,
        }
    }
}
