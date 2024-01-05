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
    let mut msg = HashMap::new();
    let mut rest = &bin[..];
    loop {
        let key = &rest[0];
        eprintln!("key: {key}");
        let (next, field, wire_type) = map_to_wire_type(*key, &rest[1..]);
        eprintln!("field: {field}, next: {next:?}, wire_type: {wire_type:?})");
        msg.insert(field, wire_type);
        rest = next;
        if rest == &[] {
            break;
        }
    }
    msg
}

fn map_to_wire_type(key: u8, rest: &[u8]) -> (&[u8], u8, WireType) {
    let wire_type = key & WIRE_TYPE_MASK;
    //eprintln!("wire_type: {wire_type}");
    let field_num = key >> 3 & FIELD_NUM_MASK;
    //eprintln!("field_num: {field_num}");
    let (next_rest, wire_type) = match wire_type {
        0 => {
            let (next_rest, value) = read_num(&rest);
            (next_rest, WireType::Varint(value))
        }
        2 => {
            let len = *&rest[0] as usize;
            let value = &rest[1..len + 1];
            let s = String::from_utf8(value.to_vec()).expect("no valid utf-8");
            (&rest[len + 1..], WireType::Len(s))
        }
        _ => panic!("Unsupported Wire-Type"),
    };
    (next_rest, field_num, wire_type)
}

fn read_num(rest: &[u8]) -> (&[u8], u64) {
    let mut value = Vec::new();
    //eprintln!("rest-start: {rest:?}");
    let mut cursor = 0;
    for num in rest.iter() {
        cursor += 1;
        let is_continue = num & CONTINUE_MASK;
        //eprintln!("is_continue: {is_continue}");
        //eprintln!("raw-num: {num}");
        let num_no_continue_bit = num & DROP_CONINUE_BIT;
        let num = format!("{num_no_continue_bit:07b}");
        //eprintln!("num-bin: {num}");
        value.push(num);
        if is_continue != CONTINUE_MASK {
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
    (
        &rest[cursor..],
        u64::from_str_radix(&num_total_bin, 2).expect("Could not convert to hex"),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const BASIC_MSG: &str = "{'a': 150}";

    //#[test]
    fn basic_msg_to_bin() {
        let ser = BASIC_MSG.serialize();
        assert_eq!([8, 96, 1], ser);
    }

    #[test]
    fn basic_bin_to_msg() {
        // { a: 150 }
        let bin: u64 = 0x089601;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Varint(150), *msg.get(&1).unwrap());
    }

    #[test]
    fn number_more_bytes() {
        // { a: 123456789123456 }
        let bin: u64 = 0x0880a3beb088891c;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Varint(123456789123456), *msg.get(&1).unwrap());
    }

    #[test]
    fn map_two_varint_fields() {
        // {
        //   a: 42,
        //   b: 43
        // }
        let bin: u64 = 0x082a102b;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Varint(42), *msg.get(&1).unwrap());
        assert_eq!(WireType::Varint(43), *msg.get(&2).unwrap());
    }

    #[test]
    fn single_len_field() {
        // { a: "Foo" }
        let bin: u64 = 0x0a03466f6f;
        let msg = deserialize(&bin.to_be_bytes());
        assert_eq!(WireType::Len("Foo".to_string()), *msg.get(&1).unwrap());
    }
}
