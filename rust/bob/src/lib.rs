enum ReplyTo {
    Question,
    Yell,
    Statement,
    Other,
}

pub fn reply(message: &str) -> &str {
    let mut reply: ReplyTo = ReplyTo::Other;
    let mut msg_processed = message.trim().to_string();
    if msg_processed.len() == 0 {
        reply = ReplyTo::Statement;
    } else {
        // TODO: better classify yell (all words in all caps, ? or ! or no punc)
        // TODO: better classify question (all words not in all caps, ?)
        let last = msg_processed.pop().unwrap();
        let words = msg_processed.split_whitespace();


        if last == '?' {
            let next_last = msg_processed.pop().unwrap();
            if next_last.is_uppercase() {
                reply = ReplyTo::Yell;
            } else {
                reply = ReplyTo::Question;
            }
        } else if last == '!' {
            let next_last = msg_processed.pop().unwrap();
            if next_last.is_uppercase() {
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
