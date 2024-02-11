use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn parse_env_file_to_map(filename: &str, to_upper: bool) -> Result<HashMap<String, String>> {
    let mut res: HashMap<String, String> = HashMap::new();

    let lines = parse_env_file(filename)?;
    for (key, val) in lines {
        res.insert(if to_upper { key.to_uppercase() } else { key }, val);
    }

    Ok(res)
}

pub fn parse_env_file(filename: &str) -> Result<Vec<(String, String)>> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    parse_env(&mut reader)
}

pub fn parse_env_to_map<R: Read>(
    reader: &mut BufReader<R>, to_upper: bool,
) -> Result<HashMap<String, String>> {
    let mut res: HashMap<String, String> = HashMap::new();

    let lines = parse_env(reader)?;
    for (key, val) in lines {
        res.insert(if to_upper { key.to_uppercase() } else { key }, val);
    }

    Ok(res)
}

pub fn parse_env<R: Read>(reader: &mut BufReader<R>) -> Result<Vec<(String, String)>> {
    let mut lines: Vec<(String, String)> = Vec::new();

    let mut substitution_data = HashMap::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        match parse_line(line.as_str(), &mut substitution_data) {
            Ok(Some(parsed_line)) => {
                lines.push(parsed_line);
            }
            Ok(None) => {}
            Err(err) => return Err(err),
        }
    }

    Ok(lines)
}

//
// Shamelessly stolen from dotenv.
//
// see: https://github.com/dotenv-rs/dotenv/blob/master/dotenv/src/parse.rs
// reason: Doesn't expose as pub, can only load env (I want to extract only).
//

pub type Result<T> = std::result::Result<T, Error>;

// for readability's sake
pub type ParsedLine = Result<Option<(String, String)>>;

#[derive(Debug)]
pub enum Error {
    LineParse(String, usize),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LineParse(line, error_index) => {
                write!(fmt, "Error parsing line: '{line}', error at line index: {error_index}")
            }
        }
    }
}

pub fn parse_line(
    line: &str, substitution_data: &mut HashMap<String, Option<String>>,
) -> ParsedLine {
    let mut parser = LineParser::new(line, substitution_data);
    parser.parse_line()
}

struct LineParser<'a> {
    original_line: &'a str,
    substitution_data: &'a mut HashMap<String, Option<String>>,
    line: &'a str,
    pos: usize,
}

impl<'a> LineParser<'a> {
    fn new(
        line: &'a str, substitution_data: &'a mut HashMap<String, Option<String>>,
    ) -> LineParser<'a> {
        LineParser {
            original_line: line,
            substitution_data,
            line: line.trim_end(), // we don’t want trailing whitespace
            pos: 0,
        }
    }

    fn err(&self) -> Error {
        Error::LineParse(self.original_line.into(), self.pos)
    }

    fn parse_line(&mut self) -> ParsedLine {
        self.skip_whitespace();
        // if its an empty line or a comment, skip it
        if self.line.is_empty() || self.line.starts_with('#') {
            return Ok(None);
        }

        let mut key = self.parse_key()?;
        self.skip_whitespace();

        // export can be either an optional prefix or a key itself
        if key == "export" {
            // here we check for an optional `=`, below we throw directly when it’s not found.
            if self.expect_equal().is_err() {
                key = self.parse_key()?;
                self.skip_whitespace();
                self.expect_equal()?;
            }
        } else {
            self.expect_equal()?;
        }
        self.skip_whitespace();

        if self.line.is_empty() || self.line.starts_with('#') {
            self.substitution_data.insert(key.clone(), None);
            return Ok(Some((key, String::new())));
        }

        let parsed_value = parse_value(self.line, self.substitution_data)?;
        self.substitution_data.insert(key.clone(), Some(parsed_value.clone()));

        Ok(Some((key, parsed_value)))
    }

    fn parse_key(&mut self) -> Result<String> {
        if !self.line.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
            return Err(self.err());
        }
        let index =
            match self.line.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.')) {
                Some(index) => index,
                None => self.line.len(),
            };
        self.pos += index;
        let key = String::from(&self.line[..index]);
        self.line = &self.line[index..];
        Ok(key)
    }

    fn expect_equal(&mut self) -> Result<()> {
        if !self.line.starts_with('=') {
            return Err(self.err());
        }
        self.line = &self.line[1..];
        self.pos += 1;
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        if let Some(index) = self.line.find(|c: char| !c.is_whitespace()) {
            self.pos += index;
            self.line = &self.line[index..];
        } else {
            self.pos += self.line.len();
            self.line = "";
        }
    }
}

#[derive(Eq, PartialEq)]
enum SubstitutionMode {
    None,
    Block,
    EscapedBlock,
}

fn parse_value(
    input: &str, substitution_data: &mut HashMap<String, Option<String>>,
) -> Result<String> {
    let mut strong_quote = false; // '
    let mut weak_quote = false; // "
    let mut escaped = false;
    let mut expecting_end = false;

    //FIXME can this be done without yet another allocation per line?
    let mut output = String::new();

    let mut substitution_mode = SubstitutionMode::None;
    let mut substitution_name = String::new();

    for (index, c) in input.chars().enumerate() {
        //the regex _should_ already trim whitespace off the end
        //expecting_end is meant to permit: k=v #comment
        //without affecting: k=v#comment
        //and throwing on: k=v w
        if expecting_end {
            if c == ' ' || c == '\t' {
                continue;
            } else if c == '#' {
                break;
            } else {
                return Err(Error::LineParse(input.to_owned(), index));
            }
        } else if escaped {
            //TODO I tried handling literal \r but various issues
            //imo not worth worrying about until there's a use case
            //(actually handling backslash 0x10 would be a whole other matter)
            //then there's \v \f bell hex... etc
            match c {
                '\\' | '\'' | '"' | '$' | ' ' => output.push(c),
                'n' => output.push('\n'), // handle \n case
                _ => {
                    return Err(Error::LineParse(input.to_owned(), index));
                }
            }

            escaped = false;
        } else if strong_quote {
            if c == '\'' {
                strong_quote = false;
            } else {
                output.push(c);
            }
        } else if substitution_mode != SubstitutionMode::None {
            if c.is_alphanumeric() {
                substitution_name.push(c);
            } else {
                match substitution_mode {
                    SubstitutionMode::None => unreachable!(),
                    SubstitutionMode::Block => {
                        if c == '{' && substitution_name.is_empty() {
                            substitution_mode = SubstitutionMode::EscapedBlock;
                        } else {
                            apply_substitution(
                                substitution_data,
                                &substitution_name.drain(..).collect::<String>(),
                                &mut output,
                            );
                            if c == '$' {
                                substitution_mode = if !strong_quote && !escaped {
                                    SubstitutionMode::Block
                                } else {
                                    SubstitutionMode::None
                                }
                            } else {
                                substitution_mode = SubstitutionMode::None;
                                output.push(c);
                            }
                        }
                    }
                    SubstitutionMode::EscapedBlock => {
                        if c == '}' {
                            substitution_mode = SubstitutionMode::None;
                            apply_substitution(
                                substitution_data,
                                &substitution_name.drain(..).collect::<String>(),
                                &mut output,
                            );
                        } else {
                            substitution_name.push(c);
                        }
                    }
                }
            }
        } else if c == '$' {
            substitution_mode = if !strong_quote && !escaped {
                SubstitutionMode::Block
            } else {
                SubstitutionMode::None
            }
        } else if weak_quote {
            if c == '"' {
                weak_quote = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                output.push(c);
            }
        } else if c == '\'' {
            strong_quote = true;
        } else if c == '"' {
            weak_quote = true;
        } else if c == '\\' {
            escaped = true;
        } else if c == ' ' || c == '\t' {
            expecting_end = true;
        } else {
            output.push(c);
        }
    }

    //XXX also fail if escaped? or...
    if substitution_mode == SubstitutionMode::EscapedBlock || strong_quote || weak_quote {
        let value_length = input.len();
        Err(Error::LineParse(
            input.to_owned(),
            if value_length == 0 { 0 } else { value_length - 1 },
        ))
    } else {
        apply_substitution(
            substitution_data,
            &substitution_name.drain(..).collect::<String>(),
            &mut output,
        );
        Ok(output)
    }
}

fn apply_substitution(
    substitution_data: &mut HashMap<String, Option<String>>, substitution_name: &str,
    output: &mut String,
) {
    if let Ok(environment_value) = std::env::var(substitution_name) {
        output.push_str(&environment_value);
    } else {
        let stored_value = substitution_data.get(substitution_name).unwrap_or(&None).to_owned();
        output.push_str(&stored_value.unwrap_or_default());
    };
}
