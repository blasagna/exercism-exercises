enum ReplyTo {
    Question,
    Yell,
    Statement,
    Other,
}

pub fn reply(message: &str) -> &str {
    let mut reply: ReplyTo = ReplyTo::Other;
    let msg_processed = message.trim();
    if msg_processed.len() == 0 {
        reply = ReplyTo::Statement;
    } else {
        // determine if any letters present
        let mut any_letters = false;
        for char in msg_processed.chars() {
            if char.is_alphabetic() {
                any_letters = true;
            }
        }

        if any_letters {
            if msg_processed.ends_with('!') && msg_processed == msg_processed.to_uppercase() {
                reply = ReplyTo::Yell;
            } else if msg_processed.ends_with('?') {
                reply = ReplyTo::Question;
                if msg_processed == msg_processed.to_uppercase() {
                    reply = ReplyTo::Yell;
                }
            } else if msg_processed == msg_processed.to_uppercase() {
                reply = ReplyTo::Yell;
            }
        } else {
            if msg_processed.ends_with('?') {
               reply = ReplyTo::Question;
            } else if msg_processed.ends_with('!') {
                reply = ReplyTo::Yell;
            }
        }
    }


    match reply {
        ReplyTo::Question => "Sure.",
        ReplyTo::Yell => "Whoa, chill out!",
        ReplyTo::Statement => "Fine. Be that way!",
        ReplyTo::Other => "Whatever.",
    }
}
