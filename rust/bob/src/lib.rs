pub fn reply(to: &str) -> &str {
    match to {
        x if x.is_empty() =>  "Fine. Be that way!",
        x if x.ends_with("?") =>  "Sure.",
        x if x.to_uppercase() == to.to_string() =>  "Whoa, chill out!",
        _ =>  "Whatever."
    }
}