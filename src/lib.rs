pub fn fizzbuzz(n: u32) -> String {
    let mut result = String::new();
    result.push_str(&divisible_by_three_is_fizz(n));
    if n % 5 == 0 { result.push_str( "buzz"); }
    if result.len() == 0 { result = n.to_string(); }
    result
}

fn divisible_by_three_is_fizz(n: u32) -> String {
    if n % 3 != 0 { return "".to_string(); }
    "fizz".to_string()
}