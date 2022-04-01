/// Represents any bracket, differentiating by brace-type, not orientation
enum BraceKind {
    Curly,
    Parenthesis,
    Square,
}

impl BraceKind {
    //! Checks if another BraceKind is of the same kind
    fn matches(&self, other: &BraceKind) -> bool {
        matches!((self, other),
            (BraceKind::Curly, BraceKind::Curly) |
            (BraceKind::Parenthesis, BraceKind::Parenthesis) |
            (BraceKind::Square, BraceKind::Square)
        )
    }
}

/// Represents any bracket, differentiating by orientation
enum Bracket {
    Opening(BraceKind),
    Closing(BraceKind),
}

impl Bracket {
    /// Creates a new Bracket if the character is one
    fn from_char(char: char) -> Option<Bracket> {
        match char {
            '{' => Some(Bracket::Opening(BraceKind::Curly)),
            '(' => Some(Bracket::Opening(BraceKind::Parenthesis)),
            '[' => Some(Bracket::Opening(BraceKind::Square)),
            ']' => Some(Bracket::Closing(BraceKind::Square)),
            ')' => Some(Bracket::Closing(BraceKind::Parenthesis)),
            '}' => Some(Bracket::Closing(BraceKind::Curly)),
            _  => None,
        }
    }

    /// Checks if the passed bracket closes this one
    fn closed_by(&self, closing: &Bracket) -> bool {
        match (self, closing) {
            (Bracket::Opening(k_opening), Bracket::Closing(k_closing)) => {
                k_opening.matches(k_closing)
            }
            _ => false
        }
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack: Vec<Bracket> = vec![];
    for char in string.chars() {
        let bracket = match Bracket::from_char(char) {
            Some(bracket) => bracket,
            None => continue,
        };

        match &bracket {
            Bracket::Opening(_) => {
                bracket_stack.push(bracket);
            },
            Bracket::Closing(_) => {
                match bracket_stack.pop() {
                    Some(last_open) => {
                        if !last_open.closed_by(&bracket) {
                            return false;
                        }
                    },
                    None => return false,
                }
            }
        };
    };
    bracket_stack.is_empty()
}

