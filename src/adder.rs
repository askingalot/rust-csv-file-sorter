pub fn adder(a: isize, b: isize) -> isize {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, adder(1, 1));
    }
}
