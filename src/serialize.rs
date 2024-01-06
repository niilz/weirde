use std::collections::HashMap;

use crate::{
    WireType, CONTINUE_MASK, DROP_CONINUE_BIT, FIELD_NUM_MASK, U64_MAX_LEN, WIRE_TYPE_MASK,
};

pub trait Proto {
    fn serialize(&self, field: u8) -> Vec<u8>;
}

impl Proto for u64 {
    fn serialize(&self, field: u8) -> Vec<u8> {
        let wire_type = WireType::Varint(*self);
        wire_type.serialize(field)
    }
}
//const CONTINUE_MASK: u8 = 0b1000_0000;
//const DROP_CONINUE_BIT: u8 = 0b0111_1111;
//const WIRE_TYPE_MASK: u8 = 0b0000_0111;
//const FIELD_NUM_MASK: u8 = 0b0111;
//const U64_MAX_LEN: usize = 16;

impl Proto for WireType {
    fn serialize(&self, field: u8) -> Vec<u8> {
        let (key, value) = match self {
            WireType::Varint(v) => {
                // 0 => no continuation bit
                //  xxxx => the field number
                //      000 => wire_type: 0 == VARINT
                let key = format!("0{:04b}000", field);
                let key = u64::from_str_radix(&key, 2).expect("key binary-conversion failed");
                let value = format!("{v:b}");
                let value = u64::from_str_radix(&value, 2).expect("value binary-conversion failed");
                println!("{v}");
                (key, value)
            }
            WireType::Len(_) => todo!(),
        };
        [key.to_be_bytes(), value.to_be_bytes()]
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

    const BASIC_MSG: &str = "1: 42";

    #[test]
    fn basic_msg_to_bin() {
        let ser = 42.serialize(1);
        // 082a
        assert_eq!(vec![8, 42], ser);
    }
}
