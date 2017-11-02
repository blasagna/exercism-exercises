enum ReplyTo {
    Question,
    Yell,
    Statement,
    Other,
}

pub fn reply(message: &str) -> &str {
    //todo: continue with remaining test failures
    let mut reply: ReplyTo = ReplyTo::Other;
    let msg_processed = message.trim();
    if msg_processed.len() == 0 {
        reply = ReplyTo::Statement;
    } else if msg_processed.ends_with('!') {
        reply = ReplyTo::Yell;
    } else if msg_processed.ends_with('?') {
        reply = ReplyTo::Question;
        if msg_processed == msg_processed.to_uppercase() {
            reply = ReplyTo::Yell;
        }
    } else if msg_processed == msg_processed.to_uppercase() {
        reply = ReplyTo::Yell;
    }


    match reply {
        ReplyTo::Question => "Sure.",
        ReplyTo::Yell => "Whoa, chill out!",
        ReplyTo::Statement => "Fine. Be that way!",
        ReplyTo::Other => "Whatever.",
    }
}
