pub fn reply(message: &str) -> &str {
    let msg_clean = message.trim();

    if msg_clean.is_empty() {
        "Fine. Be that way!"
    } else if msg_clean.find(char::is_uppercase).is_some() && msg_clean.find(char::is_lowercase).is_none() {
        "Whoa, chill out!"
    } else if msg_clean.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
