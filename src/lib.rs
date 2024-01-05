pub trait Proto {
    fn serialize(&self) -> &[u8];
}

impl Proto for &str {
    fn serialize(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BASIC_MSG: &str = "{'a': 150}";

    #[test]
    fn basic_msg() {
        let ser = BASIC_MSG.serialize();
        assert_eq!([8, 96, 1], ser);
    }
}
