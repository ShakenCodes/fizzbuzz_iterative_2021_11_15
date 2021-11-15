pub fn fizzbuzz(n: u32) -> String {
    let text = div_by_three_is_fizz(n) + &div_by_five_is_buzz(n);
    not_fizz_nor_buzz_is_number(text, n)
}

fn div_by_three_is_fizz(n: u32) -> String {
    if n % 3 != 0 { return "".to_string(); }
    "fizz".to_string()
}

fn div_by_five_is_buzz(n: u32) -> String {
    if n % 5 != 0 { return "".to_string(); }
    "buzz".to_string()
}

fn not_fizz_nor_buzz_is_number(s: String, n: u32) -> String {
    if s.len() > 0 { return s; }
    n.to_string()
}