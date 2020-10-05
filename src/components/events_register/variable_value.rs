use std::cmp;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum VariableValue {
    Bool(bool),
    Num(i32),
    Str(String),
}

impl PartialEq for VariableValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Num(a), Self::Num(b)) => a == b,
            (Self::Str(a), Self::Str(b)) => a == b,
            (a, b) => {
                eprintln!(
                    "[WARNING VariableValue::partial_eq]\n    Can't compare \
                     values of different types:\n    {:?} and {:?}",
                    a, b
                );
                false
            }
        }
    }
}

impl PartialOrd for VariableValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Num(a), Self::Num(b)) => a.partial_cmp(b),
            (Self::Str(a), Self::Str(b)) => a.partial_cmp(b),
            (a, b) => {
                eprintln!(
                    "[WARNING VariableValue::partial_cmp]\n    Can't compare \
                     values of different types:\n    {:?} and {:?}",
                    a, b
                );
                None
            }
        }
    }
}
