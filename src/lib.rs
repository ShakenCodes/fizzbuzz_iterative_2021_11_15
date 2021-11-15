pub fn fizzbuzz(n: u32) -> String {
    if n % 15 != 0 { 
        if n % 3 == 0 { return "fizz".to_string(); }
        if n % 5 == 0 { return "buzz".to_string(); }
        return n.to_string();
    }
    "fizzbuzz".to_string()
}
