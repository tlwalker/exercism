extern crate bob;

#[test]
fn test_statement() {
    assert_eq!("Whatever.", bob::reply("Tom-ay-to, tom-aaaah-to."));
}

#[test]
fn test_shouting() {
    assert_eq!("Whoa, chill out!", bob::reply("WATCH OUT!"));
}

#[test]
fn test_exclaiming() {
    assert_eq!("Whatever.", bob::reply("Let's go make out behind the gym!"));
}

#[test]
fn test_asking() {
    assert_eq!("Sure.", bob::reply("Does this cryogenic chamber make me look fat?"));
}

#[test]
fn test_shout_numbers() {
    assert_eq!("Whoa, chill out!", bob::reply("1, 2, 3 GO!"));
}

#[test]
fn test_shout_weird_characters() {
    assert_eq!("Whoa, chill out!", bob::reply("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"));
}

#[test]
fn test_shout_without_punctuation() {
    assert_eq!("Whoa, chill out!", bob::reply("I HATE YOU"));
}

#[test]
fn test_non_question_with_question_mark() {
    assert_eq!("Whatever.", bob::reply("Ending with ? means a question."));
}

#[test]
fn test_silent_treatment() {
    assert_eq!("Fine. Be that way!", bob::reply(""));
}

