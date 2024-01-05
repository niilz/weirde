pub fn call_lib() {
    println!("Lib got called");
}

#[cfg(test)]
mod test {
    #[test]
    fn works() {
        assert!(true);
    }
}
