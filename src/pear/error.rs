use super::lexer::TokenPosition;
use colored::Colorize;

pub struct ResponseLocation {
    position: TokenPosition,
    span:     usize,
}

impl ResponseLocation {
    pub fn new(position: TokenPosition, span: usize) -> Self {
        ResponseLocation {
            position,
            span
        }
    }
}

pub enum ResponseType {
    Error,
    Warning,
}

pub struct Response {
    location: ResponseLocation,
    kind:     ResponseType,
    message:  String,
}

impl Response {
    pub fn display(&self, lines: Vec<String>) {
        let (color, kind) = match self.kind {
            ResponseType::Error   => ("red",    "error"),
            ResponseType::Warning => ("yellow", "warn"),
        };

        let message = format!(
            "{}{}{}\n", kind.color(color).bold(),
            ": ".white().bold(),
            self.message.bold(),
        );
        
        let line_number = self.location.position.line;
        
        let prefix = format!("{:5} |", line_number + 1).blue().bold();
        let line   = format!("{}{}", prefix, lines.get(line_number).unwrap());

        let indicator = format!(
                            "{:offset$}{:^<count$}", " ", " ".color(color).bold(),
                            offset = prefix.len() + self.location.position.col - 2,
                            count  = self.location.span + 1,
                        );

        println!("{}{}\n{}", message, line, indicator)
    }
}

pub fn make_error(location: ResponseLocation, message: String) -> Response {
    Response {
        location,
        kind: ResponseType::Error,
        message,
    }
}

pub type ResResult<T> = Result<T, Response>;
