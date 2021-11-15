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
        assert_eq!("16", fizzbuzz(16));
        assert_eq!("17", fizzbuzz(17));
        assert_eq!("fizz", fizzbuzz(18));
        assert_eq!("19", fizzbuzz(19));
        assert_eq!("buzz", fizzbuzz(20));
        assert_eq!("fizz", fizzbuzz(21));
        assert_eq!("22", fizzbuzz(22));
        assert_eq!("23", fizzbuzz(23));
        assert_eq!("fizz", fizzbuzz(24));
        assert_eq!("buzz", fizzbuzz(25));
        assert_eq!("26", fizzbuzz(26));
        assert_eq!("fizz", fizzbuzz(27));
        assert_eq!("28", fizzbuzz(28));
        assert_eq!("29", fizzbuzz(29));
        assert_eq!("fizzbuzz", fizzbuzz(30));
    }
}
