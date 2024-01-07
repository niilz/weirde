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

trait StripLeading<I, T> {
    fn strip_leading(self, strip: T) -> Vec<T>;
}

impl<I, T> StripLeading<I, T> for I
where
    I: IntoIterator<Item = T>,
    T: std::cmp::Eq,
{
    fn strip_leading(self, strip: T) -> Vec<T> {
        self.into_iter().skip_while(|t| t == &strip).collect()
    }
}

impl WireFormat for WireType {
    fn serialize(&self, field: u8) -> Vec<u8> {
        let (key, value) = match self {
            WireType::Varint(v) => {
                // 0 => no continuation bit
                //  xxxx => the field number
                //      000 => wire_type: 0 == VARINT
                let key = format!("0{:04b}000", field);
                let key =
                    u64::from_str_radix(&key, 2).expect("varint-key binary-conversion failed");
                let key = key.to_be_bytes().to_vec();
                let value = format!("{v:b}");
                let value = u64::from_str_radix(&value, 2).expect("value binary-conversion failed");
                println!("{v}");
                let value = value.to_be_bytes().to_vec();
                (key, value)
            }
            WireType::Len(s) => {
                let key = format!("0{:04b}010", field);
                let key = u64::from_str_radix(&key, 2).expect("len-key binary-conversion failed");
                let key = key.to_be_bytes().to_vec();
                let value = s.as_bytes().to_vec();
                let len = value.len();
                let len_stripped = u64::from_str_radix(&format!("{:x}", len), 16)
                    .expect("len not converted to hex")
                    .to_be_bytes()
                    .strip_leading(0u8);
                (key, [len_stripped, value].concat())
            }
        };
        [key, value]
            .into_iter()
            .flat_map(|bytes| bytes.strip_leading(0u8))
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

    #[test]
    fn large_num_to_bin() {
        let ser = u64::MAX.serialize(1);
        // 082a
        // 08ff ffff ffff ffff ffff 01
        assert_eq!(vec![8, 255, 255, 255, 255, 255, 255, 255, 255], ser);
    }

    #[test]
    fn basic_len_to_bin() {
        let ser = "Foo".to_string().serialize(1);
        // 0a03466f6f
        assert_eq!(vec![10, 3, 70, 111, 111], ser);
    }

    #[test]
    fn simple_message_to_bin() {
        let msg = SimpleMessage {
            a: 1,
            b: "Foo".to_string(),
        };
        // 082a1203466f6f
        assert_eq!(vec![8, 1, 18, 3, 70, 111, 111], msg.proto_msg());
    }
}
