//! Parser for the Junk data format.
//!
//! Junk is a lightweight data language for describing game entities. It translates 1:1 into JSON.

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::fmt;

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
    /// A 64-bit floating-point number.
    Float(f64),
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
            Rule::float_literal => Value::Float(pair.as_str().parse().unwrap()),
            Rule::int_literal => Value::Int(pair.as_str().parse().unwrap()),
            Rule::str_literal => Value::Str(unescape(pair.into_inner().as_str())),
            Rule::list_value => Value::List(pair.into_inner().map(Value::from).collect()),
            Rule::object_value => {
                let mut pairs = pair.into_inner().peekable();
                let mut defs = vec![];
                if pairs.peek().map(|p| p.as_rule()) == Some(Rule::id) {
                    let id = pairs.next().unwrap().as_str().to_string();
                    defs.push(Def {
                        key: "id".to_string(),
                        value: Value::Str(id),
                    });
                }
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
            Value::Float(val) => write!(f, "{val}"),
            Value::Bool(val) => write!(f, "{val}"),
            Value::Str(val) => {
                write!(f, "\"")?;
                for c in val.chars() {
                    match c {
                        '"' => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\r' => write!(f, "\\r")?,
                        '\t' => write!(f, "\\t")?,
                        c => write!(f, "{c}")?,
                    }
                }
                write!(f, "\"")
            }
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

fn unescape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                _ => unreachable!("invalid escape — grammar should have rejected this"),
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn parse_junk(input: &str) -> Result<String, pest::error::Error<Rule>> {
    let mut root = JunkParser::parse(Rule::root, input)?;
    let body = root.next().unwrap();
    let value = match body.as_rule() {
        Rule::root_list => Value::List(body.into_inner().map(Value::from).collect()),
        Rule::root_object => Value::Object(body.into_inner().map(Def::from).collect()),
        _ => unreachable!(),
    };
    Ok(value.to_string())
}
