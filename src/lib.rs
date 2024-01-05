use std::collections::HashMap;

pub trait Proto {
    fn serialize(&self) -> &[u8];
}

impl Proto for &str {
    fn serialize(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum WireType {
    Varint(u64),
    Len(String),
}

fn deserialize(bin: u64) -> HashMap<u8, WireType> {
    let mut msg = HashMap::new();
    let wire_s = WireType::Len("Foo".to_string());
    msg.insert(1u8, wire_s);
    msg
}

#[cfg(test)]
mod test {
    use super::*;

    const BASIC_MSG: &str = "{'a': 150}";

    #[test]
    fn basic_msg_to_bin() {
        let ser = BASIC_MSG.serialize();
        assert_eq!([8, 96, 1], ser);
    }

    #[test]
    fn basic_bin_to_msg() {
        let bin = 0x089601;
        let msg = deserialize(bin);
        assert_eq!(WireType::Varint(150), *msg.get(&1).unwrap())
    }
}
