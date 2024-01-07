use std::collections::HashMap;

use crate::{
    WireType, CONTINUE_MASK, DROP_CONINUE_BIT, FIELD_NUM_MASK, U64_MAX_LEN, WIRE_TYPE_MASK,
};

pub trait Proto {
    fn proto_msg(&self) -> Vec<u8>;
}

struct SimpleMessage {
    a: u64,
    b: String,
}

impl Proto for SimpleMessage {
    fn proto_msg(&self) -> Vec<u8> {
        let a = self.a.serialize(1);
        let b = self.b.serialize(2);
        [a, b].concat()
    }
}

pub trait WireFormat {
    fn serialize(&self, field: u8) -> Vec<u8>;
}

impl WireFormat for u64 {
    fn serialize(&self, field: u8) -> Vec<u8> {
        let wire_type = WireType::Varint(*self);
        wire_type.serialize(field)
    }
}

impl WireFormat for String {
    fn serialize(&self, field: u8) -> Vec<u8> {
        let wire_type = WireType::Len(self.to_string());
        wire_type.serialize(field)
    }
}
//const CONTINUE_MASK: u8 = 0b1000_0000;
//const DROP_CONINUE_BIT: u8 = 0b0111_1111;
//const WIRE_TYPE_MASK: u8 = 0b0000_0111;
//const FIELD_NUM_MASK: u8 = 0b0111;
//const U64_MAX_LEN: usize = 16;

impl WireFormat for WireType {
    fn serialize(&self, field: u8) -> Vec<u8> {
        let key = format!("0{:04b}000", field);
        let key = u64::from_str_radix(&key, 2).expect("key binary-conversion failed");
        let key = key.to_be_bytes().to_vec();
        let value = match self {
            WireType::Varint(v) => {
                // 0 => no continuation bit
                //  xxxx => the field number
                //      000 => wire_type: 0 == VARINT
                let value = format!("{v:b}");
                let value = u64::from_str_radix(&value, 2).expect("value binary-conversion failed");
                println!("{v}");
                value.to_be_bytes().to_vec()
            }
            WireType::Len(s) => s.as_bytes().to_vec(),
        };
        [key, value]
            .iter()
            .flat_map(|bytes| {
                bytes
                    .iter()
                    .skip_while(|b| *b == &0u8)
                    .map(|n| *n)
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_num_to_bin() {
        let ser = 42.serialize(1);
        // 082a
        assert_eq!(vec![8, 42], ser);
    }
}
