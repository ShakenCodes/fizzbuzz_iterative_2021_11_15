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
    }
}
