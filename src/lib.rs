use std::collections::HashMap;

const CONTINUE_MASK: u8 = 0b1000_0000;
const DROP_CONINUE_BIT: u8 = 0b0111_1111;
const WIRE_TYPE_MASK: u8 = 0b0000_0111;
const FIELD_NUM_MASK: u8 = 0b0111;
const U64_MAX_LEN: usize = 16;

pub mod deserialize;
pub mod serialize;
