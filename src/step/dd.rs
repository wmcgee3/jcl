use in_stream::InStream;
use serde::{Deserialize, Serialize};

pub mod in_stream;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DdGroup {
    name: String,
    statements: Vec<DdStatement>,
}

impl DdGroup {
    pub fn new<T>(name: T) -> DdGroup
    where
        T: std::fmt::Display,
    {
        DdGroup {
            name: name.to_string(),
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: DdStatement) -> &mut Self {
        self.statements.push(statement);

        self
    }
}

impl std::fmt::Display for DdGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut statements = self.statements.iter();

        if let Some(statement) = statements.next() {
            writeln!(f, "//{:8} DD {}", self.name, statement)?;
        } else {
            return Ok(());
        }

        for statement in statements {
            writeln!(f, "//         DD {}", statement)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DdStatement {
    InStream(InStream),
}

impl std::fmt::Display for DdStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DdStatement::InStream(in_stream_dd) => match in_stream_dd {
                InStream {
                    text,
                    delimiter: None,
                } => {
                    write!(f, "*\n{text}\n/*")
                }
                InStream {
                    text,
                    delimiter: Some(delimiter),
                } => {
                    write!(f, "*,DLM={delimiter}\n{text}\n{delimiter}")
                }
            },
        }
    }
}
