pub enum MarkupState {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn eval_char(c: char, state: &MarkupState) -> (Option<char>, MarkupState) {
    use MarkupState::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}

fn parse_markup(markup_text: &str) -> String {
    let initial_state = ("".to_owned(), MarkupState::Normal);
    let fold_fn = |(parsed_text, current_state), current_char| {
        let (output, new_state) = eval_char(current_char, &current_state);
        match output {
            Some(c) => (format!("{}{}", parsed_text, c), new_state),
            None => (parsed_text, new_state),
        }
    };
    let (parsed_text, _) = markup_text.chars().fold(initial_state, fold_fn);

    parsed_text
}

#[cfg(test)]
mod test_text_state {
    use super::*;

    #[test]
    fn test_parsed_markup() {
        let markup_text = "This _Is_ some ^input^. #we want this transformed without this comment#";
        let expected = "This is some INPUT. ";
        let result = parse_markup(markup_text);
        assert_eq!(expected, result);
    }
}
