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

fn deserialize(bin: &[u8]) -> HashMap<u8, WireType> {
    let bin: Vec<_> = bin.iter().skip_while(|b| *b == &0u8).collect();
    let key = &bin[0];
    let continue_mask = 0b1000_0000;
    let is_continue = *key & continue_mask;
    eprintln!("is_continue: {is_continue}");
    let wire_type_mask = 0b0000_0111;
    let wire_type = *key & wire_type_mask;
    eprintln!("wire_type: {wire_type}");
    let field_num_mask = 0b0111;
    let field_num = *key >> 3 & field_num_mask;
    eprintln!("field_num: {field_num}");
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
        let bin: u64 = 0x089601;
        let bin: u64 = 0x0a0141;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Varint(150), *msg.get(&1).unwrap())
    }
}
