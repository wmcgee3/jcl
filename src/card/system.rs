pub enum System {
    Any,
    JGlobal,
    JLocal,
    Not(String),
    Nots(Vec<String>),
    Submitting,
    System(String),
    Systems(Vec<String>),
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            System::Any => "ANY",
            System::JGlobal => "JGLOBAL",
            System::JLocal => "JLOCAL",
            System::Not(system) => &format!("-{}", system),
            System::Nots(systems) => &format!("(-{})", systems.join(",")),
            System::Submitting => "*",
            System::System(system) => system,
            System::Systems(systems) => &format!("({})", systems.join(",")),
        };

        write!(f, "{}", s)
    }
}
