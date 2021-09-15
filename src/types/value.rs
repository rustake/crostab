use std::collections::HashMap;
use std::fmt;

use veho::vector::Mappers;

pub enum Value {
    Nul,
    Boo(bool),
    Num(i32),
    Str(String),
    Arr(Vec<Value>),
    Obj(HashMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nul => { write!(f, "Null") }
            Value::Num(n) => { write!(f, "{}", n) }
            Value::Boo(b) => { write!(f, "{}", b) }
            Value::Str(t) => { write!(f, "{}", t) }
            Value::Arr(v) => {
                let t = v.mapper(|x| format!("{}", x)).join(", ");
                write!(f, "[ {} ]", t)
            }
            Value::Obj(m) => {
                let t = m.mapper(|(k, v)| format!("{}: {}", k, v)).join(", ");
                write!(f, "{{ {} }}", t)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use serde_json::{json, Value};

    use veho::entries::into_hashmap;

    use crate::types::value::Value;

    #[test]
    fn test() {
        let a: Value = Value::Boo(true);
        let v = Value::Arr(vec![Value::Num(1), Value::Num(2), Value::Num(3)]);
        let m = Value::Arr(vec![
            Value::Arr(vec![Value::Num(1), Value::Num(2), Value::Num(3)]),
            Value::Arr(vec![Value::Num(1), Value::Num(2), Value::Num(3)]),
            Value::Arr(vec![Value::Num(1), Value::Num(2), Value::Num(3)]),
        ]);
        let l = Value::Obj(into_hashmap(vec![
            ("foo".to_string(), Value::Num(1)),
            ("bar".to_string(), Value::Num(2)),
            ("zen".to_string(), Value::Num(3)),
        ]));
        println!("{}", a);
        println!("{}", v);
        println!("{}", m);
        println!("{}", l);
    }
}