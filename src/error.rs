use thiserror::Error;

#[derive(Debug, Error)]
pub struct Error {
    message: String,
    origin: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.message, self.origin
        )
    }
}

#[allow(dead_code)]
impl Error {
    pub fn new<T: std::error::Error>(
        message: String,
        source: T,
    ) -> Self {
        let formatted_source = format!("{:?}", source);
        let source_name = match formatted_source.split_whitespace().nth(0) {
            Some(x) => x.to_string(),
            None => formatted_source,
        };
        Error {
            message,
            origin: format!("{source_name} ({})", source.to_string()),
        }
    }

    pub fn new_sourceless(message: String) -> Self {
        Error {
            message,
            origin: "None".to_string(),
        }
    }

    pub fn empty() -> Self {
        Error {
            message: "".to_string(),
            origin: "None".to_string(),
        }
    }
}
