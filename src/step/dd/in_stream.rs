use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Delimiter {
    first: char,
    second: char,
}

impl Delimiter {
    pub fn new(s: &str) -> anyhow::Result<Delimiter> {
        s.parse()
    }
}

impl std::str::FromStr for Delimiter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().collect::<Vec<char>>().as_slice() {
            [first, second] => Ok(Delimiter {
                first: *first,
                second: *second,
            }),
            _ => Err(anyhow::Error::msg(
                "invalid in-steam dd statement delimiter: must be 2 characters",
            )),
        }
    }
}

impl std::fmt::Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.first, self.second)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InStream {
    pub(crate) text: String,
    pub(crate) delimiter: Option<Delimiter>,
}

impl InStream {
    pub fn new(text: String) -> InStream {
        InStream {
            text,
            delimiter: None,
        }
    }

    pub fn delimiter(&mut self, delimiter: Delimiter) -> &mut Self {
        self.delimiter = Some(delimiter);

        self
    }
}
