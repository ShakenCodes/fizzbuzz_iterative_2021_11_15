pub fn fizzbuzz(n: u32) -> String {
    let mut result = divisible_by_three_is_fizz(n);
    result.push_str( &divisible_by_five_is_buzz(n));
    if result.len() == 0 { result = n.to_string(); }
    result
}

fn divisible_by_three_is_fizz(n: u32) -> String {
    if n % 3 != 0 { return "".to_string(); }
    "fizz".to_string()
}

fn divisible_by_five_is_buzz(n: u32) -> String {
    if n % 5 != 0 { return "".to_string(); }
    "buzz".to_string()
}