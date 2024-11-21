use std::{error::Error, fmt::Display};

use infer_schema::Type;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// #[derive(Serialize, Deserialize)]
// struct Test {
//     x: u64,
// }

// #[derive(Serialize, Deserialize)]
// enum Bla {
//     Blub(Test),
//     Blorg(Test),
// }

fn main() {
    let v: Value = serde_json::from_str(r#"[{"blub": 123}, {"blub": 1234}, 123]"#).unwrap();
    println!("{}", v.to_string());
    dbg!(&v);
    let ty: Type = v.serialize(infer_schema::Serializer).unwrap();
    dbg!(&ty);
    // println!("{}", serde_json::to_string(&ty).unwrap());
}
