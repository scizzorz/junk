//! Parser for the Junk data format.
//!
//! Junk is a lightweight data language for describing game entities. It translates 1:1 into JSON.

use clap::Parser as ClapParser;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[grammar = "junk.pest"]
struct JunkParser;

#[derive(Debug)]
enum Value {
    /// An ordered sequence of values: `[a, b, c]`
    List(Vec<Value>),
    /// A named object with key-value definitions: `#name { key: val }`
    Object(Vec<Def>),
    /// A signed 64-bit integer.
    Int(i64),
    /// A boolean.
    Bool(bool),
    /// A string literal.
    Str(String),
}

impl<'i> From<Pair<'i, Rule>> for Value {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::bool_literal => match pair.as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                _ => unreachable!(),
            },
            Rule::int_literal => Value::Int(pair.as_str().parse().unwrap()),
            Rule::str_literal => Value::Str(pair.into_inner().as_str().to_string()),
            Rule::list_value => Value::List(pair.into_inner().map(Value::from).collect()),
            Rule::object_value => {
                let mut pairs = pair.into_inner();
                let id = pairs.next().unwrap().as_str().to_string();
                let mut defs = vec![Def {
                    key: "id".to_string(),
                    value: Value::Str(id),
                }];
                defs.extend(pairs.map(Def::from));
                Value::Object(defs)
            }
            rule => unreachable!("unexpected rule in value position: {rule:?}"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::List(vals) => {
                let items: Vec<_> = vals.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Object(defs) => {
                let items: Vec<_> = defs.iter().map(|d| d.to_string()).collect();
                write!(f, "{{{}}}", items.join(", "))
            }
            Value::Int(val) => write!(f, "{val}"),
            Value::Bool(val) => write!(f, "{val}"),
            Value::Str(val) => write!(f, "\"{val}\""),
        }
    }
}

/// A key-value definition within an object body.
///
/// Definitions come in three forms:
/// - Positive flag: `key` → `key: true`
/// - Negative flag: `!key` → `key: false`
/// - Value assignment: `key: value`
#[derive(Debug)]
struct Def {
    key: String,
    value: Value,
}

impl<'i> From<Pair<'i, Rule>> for Def {
    fn from(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::pos_flag_def => {
                let key = pair.into_inner().next().unwrap().as_str().to_string();
                Def {
                    key,
                    value: Value::Bool(true),
                }
            }
            Rule::neg_flag_def => {
                let key = pair.into_inner().next().unwrap().as_str().to_string();
                Def {
                    key,
                    value: Value::Bool(false),
                }
            }
            Rule::value_def => {
                let mut inner = pair.into_inner();
                let key = inner.next().unwrap().as_str().to_string();
                let value = Value::from(inner.next().unwrap());
                Def { key, value }
            }
            rule => unreachable!("unexpected rule in def position: {rule:?}"),
        }
    }
}

impl fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\": {}", self.key, self.value)
    }
}

/// Parses a top-level sequence of values into a [`Value::List`].
fn parse_body(pairs: Pairs<Rule>) -> Value {
    Value::List(pairs.map(Value::from).collect())
}

#[derive(ClapParser)]
#[command(about = "Convert .junk files to JSON")]
struct Cli {
    /// Input .junk files
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Output directory for generated .json files
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut failed = false;

    for path in &cli.files {
        let contents = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error reading {}: {e}", path.display());
                failed = true;
                continue;
            }
        };

        let json = match JunkParser::parse(Rule::root, &contents) {
            Ok(mut root) => {
                let body = root.next().unwrap();
                parse_body(body.into_inner()).to_string()
            }
            Err(err) => {
                eprintln!("{}: {err}", path.display());
                failed = true;
                continue;
            }
        };

        let stem = path.file_stem().unwrap_or_default();
        let out_path = cli.output.join(stem).with_extension("json");
        if let Err(e) = fs::write(&out_path, json) {
            eprintln!("error writing {}: {e}", out_path.display());
            failed = true;
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
