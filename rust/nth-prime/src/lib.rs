fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut i: u64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn nth(n: u64) -> Result<u64, String> {
    if n <= 0 {
        return Err("input must be greater than or equal to 1".to_string());
    }

    let mut count: u64 = 0;
    let mut i: u64 = 1;
    loop {
        if is_prime(i) {
            count += 1;
        }

        if count == n {
            break;
        }
        i += 1;
    }

    Ok(i)
}
