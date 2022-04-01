use crate::MessageKind::{BlankStare, Question, Statement};
use crate::Tone::{Plain, Yell};

enum Tone {
    Yell,
    Plain,
}

enum MessageKind {
    Question,
    Statement,
    BlankStare,
}

struct Message {
    tone: Tone,
    kind: MessageKind,
}

impl Message {
    /// Returns true if all letters are capitalized
    fn is_yelling(message: &str) -> bool {
        // filter out any non-alphabetic chars
        let letters_only: String = message.chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        // is_yelling only if there are letters and they are all capital
        !letters_only.is_empty() && letters_only.chars().all(|c| c.is_ascii_uppercase())
    }

    /// Determines whether the message is a Statement, a Question, or a BlankStare
    fn detect_message_kind(message: &str) -> MessageKind {
        if message.is_empty() {
            return BlankStare
        }
        if message.ends_with('?') {
            return Question
        }
        Statement
    }

    /// Determines whether message tone is a Yell or just Plain
    fn detect_message_tone(message: &str) -> Tone {
        if Message::is_yelling(message) { Yell } else { Plain }
    }
}


impl From<&str> for Message {
    fn from(message: &str) -> Self {
        Message {
            kind: Message::detect_message_kind(message),
            tone: Message::detect_message_tone(message),
        }
    }
}

pub fn reply(message: &str) -> &str {
    let message = Message::from(message.trim());
    match message {
        Message {kind: MessageKind::Question, tone} => {
            match tone {
                Tone::Plain => "Sure.",
                Tone::Yell => "Calm down, I know what I'm doing!"
            }
        },
        Message {kind: MessageKind::Statement, tone} => {
            match tone {
                Tone::Plain => "Whatever.",
                Tone::Yell => "Whoa, chill out!",
            }
        },
        Message {kind: MessageKind::BlankStare, tone: _} => {
            "Fine. Be that way!"
        }
    }
}

