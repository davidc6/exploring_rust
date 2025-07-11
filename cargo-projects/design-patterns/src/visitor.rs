use std::fmt::format;

trait Visitor {
    type Value;

    fn visit_device(&self, v: Device) -> Self::Value;
    fn visit_ip(&self, v: Ip) -> Self::Value;
}

struct Device {
    id: String,
}

enum Ip {
    V4(String),
    V6(String),
}

struct Parser;
impl Visitor for Parser {
    type Value = String;

    fn visit_device(&self, v: Device) -> Self::Value {
        v.id
    }

    fn visit_ip(&self, v: Ip) -> Self::Value {
        match v {
            Ip::V4(v) => format!("V4 {v}"),
            Ip::V6(v) => format!("V6 {v}"),
        }
    }
}

// impl Visitor for Device {
//     type Value = Device;

//     fn visit(&self, v: Vec<i32>) -> Self::Value {
//         Device {
//             id: v[0].to_string(),
//         }
//     }
// }

// impl Visitor for Ip {
//     type Value = Ip;

//     fn visit(&self, v: Vec<i32>) -> Self::Value {
//         Ip::V4
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
