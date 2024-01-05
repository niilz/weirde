use std::collections::HashMap;

const CONTINUE_MASK: u8 = 0b1000_0000;
const DROP_CONINUE_BIT: u8 = 0b0111_1111;
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
    let bin: Vec<u8> = bin.iter().skip_while(|b| **b == 0u8).map(|n| *n).collect();
    let key = &bin[0];
    let (field, wire_type) = map_to_wire_type(*key, &bin[1..]);
    eprintln!("field: {field}, wire_type: {wire_type:?})");
    let mut msg = HashMap::new();
    msg.insert(field, wire_type);
    msg
}

fn map_to_wire_type(key: u8, rest: &[u8]) -> (u8, WireType) {
    let wire_type = key & WIRE_TYPE_MASK;
    //eprintln!("wire_type: {wire_type}");
    let field_num = key >> 3 & FIELD_NUM_MASK;
    //eprintln!("field_num: {field_num}");
    let wire_type = match wire_type {
        0 => {
            let value = read_num(&rest);
            WireType::Varint(value)
        }
        2 => WireType::Len("TODO".to_string()),
        _ => panic!("Unsupported Wire-Type"),
    };
    (field_num, wire_type)
}

fn read_num(rest: &[u8]) -> u64 {
    let mut value = Vec::new();
    //eprintln!("rest-start: {rest:?}");
    for num in rest {
        let is_continue = num & CONTINUE_MASK;
        //eprintln!("is_continue: {is_continue}");
        //eprintln!("raw-num: {num}");
        let num_no_continue_bit = num & DROP_CONINUE_BIT;
        let num = format!("{num_no_continue_bit:07b}");
        //eprintln!("num-bin: {num}");
        value.push(num);
        if !is_continue == CONTINUE_MASK {
            break;
        }
    }
    //eprintln!("value-vec: {value:?}");
    let num_total_bin = value
        .iter()
        .map(|s| s.as_str())
        .rev()
        .collect::<Vec<_>>()
        .join("");
    //eprintln!("total-bin: {concat}");
    u64::from_str_radix(&num_total_bin, 2).expect("Could not convert to hex")
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
        assert_eq!(WireType::Varint(150), *msg.get(&1).unwrap());
    }

    #[test]
    fn number_more_bytes() {
        let bin: u64 = 0x0880a3beb088891c;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Varint(123456789123456), *msg.get(&1).unwrap());
    }

    #[test]
    fn map_two_varint_fields() {
        let bin: u64 = 0x089601;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Varint(42), *msg.get(&1).unwrap());
        assert_eq!(WireType::Varint(43), *msg.get(&2).unwrap());
    }
}
