#[cfg(test)]
mod tests {
    use fizzbuzz::fizzbuzz;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!("1", fizzbuzz(1));
    }
}
