use std::collections::HashMap;

const CONTINUE_MASK: u8 = 0b1000_0000;
const WIRE_TYPE_MASK: u8 = 0b0000_0111;
const FIELD_NUM_MASK: u8 = 0b0111;

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

fn deserialize(bin: &[u8]) -> HashMap<u8, WireType> {
    let bin: Vec<_> = bin.iter().skip_while(|b| *b == &0u8).collect();
    let key = &bin[0];
    let wire_type = map_to_wire_type(**key, 42);
    let mut msg = HashMap::new();
    let wire_s = WireType::Len("Foo".to_string());
    msg.insert(1u8, wire_s);
    msg
}

fn map_to_wire_type(key: u8, value: u64) -> WireType {
    let is_continue = key & CONTINUE_MASK;
    eprintln!("is_continue: {is_continue}");
    let wire_type = key & WIRE_TYPE_MASK;
    eprintln!("wire_type: {wire_type}");
    let field_num = key >> 3 & FIELD_NUM_MASK;
    eprintln!("field_num: {field_num}");
    match field_num {
        0 => WireType::Varint(value),
        2 => WireType::Len("TODO".to_string()),
        _ => panic!("Unsupported Wire-Type"),
    }
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
        let bin: u64 = 0x089601;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Varint(150), *msg.get(&1).unwrap())
    }
}
