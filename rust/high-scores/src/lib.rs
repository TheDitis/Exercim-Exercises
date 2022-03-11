
#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores { scores }
    }

    /// Returns all past scores
    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    /// Returns most recent score
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    /// Returns maximum score
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter()
            .reduce(|acc, v| { if v > acc { v } else { acc } })
            .copied()
    }

    /// Returns vec of top 3 scores in descending order
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three: Vec<u32> = Vec::new();
        for val in self.scores {
            if top_three.len() == 0 {
                top_three.push(*val);
            }
            else if top_three.len() < 3 || val > top_three.last().unwrap() {
                HighScores::insort(&mut top_three, val);
            }
            if top_three.len() > 3 { top_three.pop(); }
        }
        top_three
    }

    /// Insert an item into a vec in sorted position (input array must be sorted)
    fn insort<T: PartialOrd + Copy>(vec: &mut Vec<T>, new_val: &T) {
        for (i, val) in vec.iter().enumerate() {
            if new_val >= val {
                vec.insert(i, *new_val);
                return;
            }
        }
        vec.insert(vec.len(), *new_val);
    }
}
