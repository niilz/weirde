use std::collections::HashMap;

use crate::{
    WireType, CONTINUE_MASK, DROP_CONINUE_BIT, FIELD_NUM_MASK, U64_MAX_LEN, WIRE_TYPE_MASK,
};

pub trait Proto {
    fn proto_msg(&self) -> Vec<u8>;
}

struct NumMessage {
    a: u64,
}
impl Proto for NumMessage {
    fn proto_msg(&self) -> Vec<u8> {
        self.a.serialize(1)
    }
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
                let value_chunks = value
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(7)
                    .map(|c| c.to_vec())
                    .collect::<Vec<_>>();
                let mut value_chunks = value_chunks.into_iter();
                let mut values = Vec::new();
                while let Some(next) = value_chunks.next() {
                    let v = next.iter().collect::<String>();
                    // do we have rest?
                    let has_rest = value_chunks.clone().peekable().next().is_some();
                    values.push(format!("{}{v}", if has_rest { 1 } else { 0 }));
                }
                let value = values
                    .iter()
                    .map(|bin| u8::from_str_radix(bin, 2).expect("bin to u8 failed"))
                    .collect();
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
    use std::io::Write;

    use super::*;

    #[test]
    fn basic_num_to_bin() {
        let ser = 42.serialize(1);
        // 082a
        assert_eq!(vec![8, 42], ser);
    }

    #[test]
    fn large_num_to_bin() {
        let ser = (u64::MAX / 8).serialize(1);
        // 1: 08ffffffffffffffff1f
        assert_eq!(vec![8, 255, 255, 255, 255, 255, 255, 255, 255, 31], ser);
    }

    #[test]
    fn basic_len_to_bin() {
        let ser = "Foo".to_string().serialize(1);
        // 0a03466f6f
        assert_eq!(vec![10, 3, 70, 111, 111], ser);
    }

    #[test]
    fn num_message_max_num_to_bin() {
        // 1: 2305843009213693951 == 1: 1fffffffffffffff
        let num_msg = NumMessage { a: u64::MAX / 8 };

        // 1: 08ffffffffffffffff1f
        let msg_bin = num_msg.proto_msg();

        let hex = vec![8, 255, 255, 255, 255, 255, 255, 255, 255, 31];
        assert_eq!(hex, msg_bin);
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

    #[test]
    fn large_num_three_field_message_to_bin() {
        let msg = SimpleMessage {
            a: u64::MAX / 8,
            b: "I am a slightly larger and more complex string with umlauts ÄöÜ".to_string(),
        };

        // msg:
        // 08ff ffff ffff ffff
        // ff1f 1242 4920 616d
        // 2061 2073 6c69 6768
        // 746c 7920 6c61 7267
        // 6572 2061 6e64 206d
        // 6f72 6520 636f 6d70
        // 6c65 7820 7374 7269
        // 6e67 2077 6974 6820
        // 756d 6c61 7574 7320
        // c384 c3b6 c39c

        assert_eq!(
            vec![
                8, 255, 255, 255, 255, 255, 255, 255, 255, 31, 18, 66, 73, 32, 97, 109, 32, 97, 32,
                115, 108, 105, 103, 104, 116, 108, 121, 32, 108, 97, 114, 103, 101, 114, 32, 97,
                110, 100, 32, 109, 111, 114, 101, 32, 99, 111, 109, 112, 108, 101, 120, 32, 115,
                116, 114, 105, 110, 103, 32, 119, 105, 116, 104, 32, 117, 109, 108, 97, 117, 116,
                115, 32, 195, 132, 195, 182, 195, 156
            ],
            msg.proto_msg()
        );
    }
}
