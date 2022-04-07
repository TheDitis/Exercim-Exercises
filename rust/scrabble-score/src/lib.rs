
/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars().map(letter_score).sum()
}

// Each group of letters (repeated for equal length) with their score value
const LETTER_SCORES: [([char; 10], u64); 7] = [
    (['a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't'     ], 1),
    (['d', 'g',      'd', 'g', 'd', 'g', 'd', 'g', 'd', 'g'], 2),
    (['b', 'c', 'm', 'p',      'b', 'c', 'm', 'p', 'b', 'c'], 3),
    (['f', 'h', 'v', 'w', 'y',      'f', 'h', 'v', 'w', 'y'], 4),
    (['k',      'k', 'k', 'k', 'k', 'k', 'k', 'k', 'k', 'k'], 5),
    (['j', 'x',      'j', 'x', 'j', 'x', 'j', 'x', 'j', 'x'], 8),
    (['q', 'z',      'q', 'z', 'q', 'z', 'q', 'z', 'q', 'z'], 10),
];

pub fn letter_score(c: char) -> u64 {
    for (letters, score) in LETTER_SCORES {
        if letters.contains(&c.to_lowercase().next().unwrap()) { return score }
    }
    0
}


