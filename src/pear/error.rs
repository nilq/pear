use super::lexer::TokenPosition;
use colored::Colorize;

pub enum ResponseType {
    Error,
    Warning,
}

pub struct Response {
    position: Option<TokenPosition>,
    kind:     ResponseType,
    message:  String,
}

impl Response {
    pub fn display(&self, lines: &Vec<String>, path: &str) {
        let (color, kind) = match self.kind {
            ResponseType::Error   => ("red",    "wrong"),
            ResponseType::Warning => ("yellow", "weird"),
        };

        let message = format!(
            "{}{}{}\n",

            kind.color(color).bold(),
            ": ".white().bold(),
            self.message.bold(),
        );

        if let Some(ref position) = self.position {

            let line_number = position.line;

            let prefix = format!("{:5} |  ", line_number).blue().bold();
            let line   = format!("{:5} {}\n{}{}", " ", "|".blue().bold(), prefix, lines.get(line_number - 1).unwrap());

            let indicator = format!(
                                "{:6}{}{:offset$}{:^<count$}", " ", "|".bold().blue(), " ", " ".color(color).bold(),
                                offset = position.col,
                                count  = 2,
                            );

            let path_line = format!("{:5}{}{}", " ", "--> ".blue().bold(), path);

            println!("{}{}\n{}\n{}", message, path_line, line, indicator)
            
        } else {
            println!("{}", message);
        }
    }
}

pub fn make_error(position: Option<TokenPosition>, message: String) -> Response {
    Response {
        position,
        kind: ResponseType::Error,
        message,
    }
}

pub type ResResult<T> = Result<T, Response>;
