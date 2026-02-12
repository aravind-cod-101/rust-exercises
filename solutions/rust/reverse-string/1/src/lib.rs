pub fn reverse(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut reversed = String::new();

    for i in (0..chars.len()).rev() {
        reversed.push(chars[i]);
    }
    reversed
}
