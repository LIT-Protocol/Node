use std::fmt::Display;
use std::str::FromStr;

use clap::builder::PossibleValue;
use clap::ValueEnum;

pub struct CliGlobalOpts {
    verbose: bool,
    quiet: bool,
    output: Option<OutputType>,
}

impl CliGlobalOpts {
    pub fn new(verbose: bool, quiet: bool, output: Option<OutputType>) -> CliGlobalOpts {
        Self { verbose, quiet, output }
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn quiet(&self) -> bool {
        self.quiet
    }

    pub fn output(&self) -> Option<OutputType> {
        self.output
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum OutputType {
    Json,
    Yaml,
    Wide,
}

impl Display for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {s}"))
    }
}

// Hand-rolled so it can work even when `derive` feature is disabled
impl ValueEnum for OutputType {
    fn value_variants<'a>() -> &'a [Self] {
        &[OutputType::Json, OutputType::Yaml, OutputType::Wide]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            OutputType::Json => PossibleValue::new("json"),
            OutputType::Yaml => PossibleValue::new("yaml"),
            OutputType::Wide => PossibleValue::new("wide"),
        })
    }
}
