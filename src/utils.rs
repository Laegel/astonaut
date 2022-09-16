use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PrecedenceValueJSON {
    Integer(i32),
    Name(String),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
enum RuleJSON {
    ALIAS {
        content: Box<RuleJSON>,
        named: bool,
        value: String,
    },
    BLANK,
    STRING {
        value: String,
    },
    PATTERN {
        value: String,
    },
    SYMBOL {
        name: String,
    },
    CHOICE {
        members: Vec<RuleJSON>,
    },
    FIELD {
        name: String,
        content: Box<RuleJSON>,
    },
    SEQ {
        members: Vec<RuleJSON>,
    },
    REPEAT {
        content: Box<RuleJSON>,
    },
    REPEAT1 {
        content: Box<RuleJSON>,
    },
    PREC_DYNAMIC {
        value: i32,
        content: Box<RuleJSON>,
    },
    PREC_LEFT {
        value: PrecedenceValueJSON,
        content: Box<RuleJSON>,
    },
    PREC_RIGHT {
        value: PrecedenceValueJSON,
        content: Box<RuleJSON>,
    },
    PREC {
        value: PrecedenceValueJSON,
        content: Box<RuleJSON>,
    },
    TOKEN {
        content: Box<RuleJSON>,
    },
    IMMEDIATE_TOKEN {
        content: Box<RuleJSON>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct GrammarJSON {
    pub(crate) name: String,
    pub rules: Map<String, Value>,
    precedences: Vec<Vec<RuleJSON>>,
    conflicts: Vec<Vec<String>>,
    externals: Vec<RuleJSON>,
    extras: Vec<RuleJSON>,
    inline: Vec<String>,
    supertypes: Vec<String>,
    word: Option<String>,
}
