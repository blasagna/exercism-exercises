pub fn raindrops(n: usize) -> String {
    // result contains at most PlingPlangPlong, 15 bytes
    let mut result: String = String::with_capacity(15);
    if n % 3 == 0 {
        result.push_str("Pling");
    }
    if n % 5 == 0 {
        result.push_str("Plang");
    }
    if n % 7 == 0 {
        result.push_str("Plong");
    }
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return n.to_string();
    }
    result
}
