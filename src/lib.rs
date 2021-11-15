pub fn fizzbuzz(n: u32) -> String {
    if n % 3 == 0 { return "fizz".to_string(); }
    if n % 5 == 0 { return "buzz".to_string(); }
    n.to_string()
}