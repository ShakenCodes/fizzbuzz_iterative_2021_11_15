#[cfg(test)]
mod tests {
    use fizzbuzz::fizzbuzz;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!("1", fizzbuzz(1));
        assert_eq!("2", fizzbuzz(2));
        assert_eq!("fizz", fizzbuzz(3));
        assert_eq!("4", fizzbuzz(4));
        assert_eq!("buzz", fizzbuzz(5));
        assert_eq!("fizz", fizzbuzz(6));
        assert_eq!("7", fizzbuzz(7));
        assert_eq!("8", fizzbuzz(8));
        assert_eq!("fizz", fizzbuzz(9));
        assert_eq!("buzz", fizzbuzz(10));
        assert_eq!("11", fizzbuzz(11));
        assert_eq!("fizz", fizzbuzz(12));
        assert_eq!("13", fizzbuzz(13));
        assert_eq!("14", fizzbuzz(14));
        assert_eq!("fizzbuzz", fizzbuzz(15));
    }
}
