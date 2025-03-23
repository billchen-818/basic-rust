pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_again() {
        let result = add(6, 3);
        assert_ne!(result, 6);
    }

    #[test]
    #[should_panic]
    fn it_works_should_panic() {
        //let result = add(6, 3);
        assert_eq!(1, 2);
    }

    #[test]
    #[ignore]
    fn it_works_ignore() {
        let result = add(6, 3);
        assert_eq!(result, 9);
    }
}
