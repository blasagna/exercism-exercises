pub fn is_leap_year(year: i32) -> bool {
    let mut result: bool = false;
    if year % 4 == 0 {
        if year % 100 == 0 && year % 400 == 0 {
            result = true;
        } else if year % 100 != 0 {
            result = true;
        }
    }
    result
}
