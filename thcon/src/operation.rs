use std::fmt;

#[derive(Debug)]
pub enum Operation {
    Darken,
    Lighten,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match &self {
            Self::Darken => "darken",
            Self::Lighten => "lighten",
        };

        write!(f, "{}", as_str)
    }
}
