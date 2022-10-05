use std::collections::HashMap;
use std::str::Chars;

/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
/// W  .  A  .  E  .  I  .  C  .  V  .  R  .  D  .  L  .  E  .  T  .  N  .  E
/// .  E  .  R  .  D  .  S  .  O  .  E  .  E  .  F  .  E  .  A  .  O  .  C  .
///
/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
/// W  .  .  .  E  .  .  .  C  .  .  .  R  .  .  .  L  .  .  .  T  .  .  .  E
/// .  E  .  R  .  D  .  S  .  O  .  E  .  E  .  F  .  E  .  A  .  O  .  C  .
/// .  .  A  .  .  .  I  .  .  .  V  .  .  .  D  .  .  .  E  .  .  .  N  .  .
/// "WECRLTE ERDSOEEFE AOCAIVDEN"
///
/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
/// W  .  .  .  .  .  I  .  .  .  .  .  R  .  .  .  .  .  E  .  .  .  .  .  E
/// .  E  .  .  .  D  .  S  .  .  .  E  .  E  .  .  .  E  .  A  .  .  .  C  .
/// .  .  A  .  E  .  .  .  C  .  V  .  .  .  D  .  L  .  .  .  T  .  N  .  .
/// .  .  .  R  .  .  .  .  .  O  .  .  .  .  .  F  .  .  .  .  .  O  .  .  .
///
/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
/// W  .  .  .  .  .  .  .  C  .  .  .  .  .  .  .  L  .  .  .  .  .  .  .  E
/// .  E  .  .  .  .  .  S  .  O  .  .  .  .  .  F  .  E  .  .  .  .  .  C  .
/// .  .  A  .  .  .  I  .  .  .  V  .  .  .  D  .  .  .  E  .  .  .  N  .  .
/// .  .  .  R  .  D  .  .  .  .  .  E  .  E  .  .  .  .  .  A  .  O  .  .  .
/// .  .  .  .  E  .  .  .  .  .  .  .  R  .  .  .  .  .  .  .  T  .  .  .  .
///
/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
/// W  .  .  .  .  .  .  .  .  .  V  .  .  .  .  .  .  .  .  .  T  .  .  .  .
/// .  E  .  .  .  .  .  .  .  O  .  E  .  .  .  .  .  .  .  A  .  O  .  .  .
/// .  .  A  .  .  .  .  .  C  .  .  .  R  .  .  .  .  .  E  .  .  .  N  .  .
/// .  .  .  R  .  .  .  S  .  .  .  .  .  E  .  .  .  E  .  .  .  .  .  C  .
/// .  .  .  .  E  .  I  .  .  .  .  .  .  .  D  .  L  .  .  .  .  .  .  .  E
/// .  .  .  .  .  D  .  .  .  .  .  .  .  .  .  F  .  .  .  .  .  .  .  .  .
///
/// 0  3  8                 9  4  1  5  10                11 6  2  7  12
/// W  E  A  R  E  D  I  S  C  O  V  E  R  E  D  F  L  E  E  A  T  O  N  C  E
/// WVT EOEAO ACREN RSEEC EIDLE DF
///
///
///
///
/// T  H  E  D  E  V  I  L  I  S  I  N  T  H  E  D  E  T  A  I  L  S
/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21
/// T  .  .  .  E  .  .  .  I  .  .  .  T  .  .  .  E  .  .  .  L  .
/// .  H  .  D  .  V  .  L  .  S  .  N  .  H  .  D  .  T  .  I  .  S
/// .  .  E  .  .  .  I  .  .  .  I  .  .  .  E  .  .  .  A  .  .  .
/// TEITEL HDVLSNHDTIS EIIEA
///
///
/// E  X  E  R  C  I  S  M  I  S  A  W  E  S  O  M  E
/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16
/// E  .  .  .  .  .  .  .  I  .  .  .  .  .  .  .  E
/// .  X  .  .  .  .  .  M  .  S  .  .  .  .  .  M  .
/// .  .  E  .  .  .  S  .  .  .  A  .  .  .  O  .  .
/// .  .  .  R  .  I  .  .  .  .  .  W  .  S  .  .  .
/// .  .  .  .  C  .  .  .  .  .  .  .  E  .  .  .  .

/// 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59
/// 1  .  .  .  .  .  .  .  .  .  3  .  .  .  .  .  .  .  .  .  3  .  .  .  .  .  .  .  .  .  7  .  .  .  .  .  .  .  .  .  1  .  .  .  .  .  .  .  .  .  4  .  .  .  .  .  .  .  .  .
/// .  1  .  .  .  .  .  .  .  1  .  4  .  .  .  .  .  .  .  2  .  3  .  .  .  .  .  .  .  8  .  1  .  .  .  .  .  .  .  4  .  8  .  .  .  .  .  .  .  9  .  6  .  .  .  .  .  .  .  6
/// .  .  2  .  .  .  .  .  2  .  .  .  5  .  .  .  .  .  4  .  .  .  3  .  .  .  .  .  9  .  .  .  5  .  .  .  .  .  4  .  .  .  1  .  .  .  .  .  0  .  .  .  1  .  .  .  .  .  8  .
/// .  .  .  3  .  .  .  3  .  .  .  .  .  5  .  .  .  4  .  .  .  .  .  7  .  .  .  0  .  .  .  .  .  9  .  .  .  8  .  .  .  .  .  6  .  .  .  1  .  .  .  .  .  7  .  .  .  2  .  .
/// .  .  .  .  5  .  1  .  .  .  .  .  .  .  8  .  1  .  .  .  .  .  .  .  7  .  1  .  .  .  .  .  .  .  7  .  5  .  .  .  .  .  .  .  7  .  5  .  .  .  .  .  .  .  7  .  1  .  .  .
/// .  .  .  .  .  8  .  .  .  .  .  .  .  .  .  9  .  .  .  .  .  .  .  .  .  6  .  .  .  .  .  .  .  .  .  2  .  .  .  .  .  .  .  .  .  6  .  .  .  .  .  .  .  .  .  1  .  .  .  .
/// 1  1  2  3  5  8  1  3  2  1  3  4  5  5  8  9  1  4  4  2  3  3  3  7  7  6  1  0  9  8  7  1  5  9  7  2  5  8  4  4  1  8  1  6  7  6  5  1  0  9  4  6  1  7  7  1  1  2  8  6

pub struct RailFence {
    rails: usize
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails: Vec<String> = (0..self.rails).map(|_| String::new()).collect();
        let mut inds = (0..self.rails).chain((1..self.rails-1).rev()).cycle();
        for c in text.chars() {
            rails[inds.next().unwrap()].push(c);
        }
        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rails: Vec<String> = (0..self.rails).map(|_| String::new()).collect();
        let mut cipher_chars = cipher.chars();

        // all the inds of the output that should touch the top rail, up to one above the cipher len
        let centers: Vec<usize> = (0..cipher.len())
            .map(|i| i * ((self.rails + (self.rails - 1)) - 1))
            .cycle()
            .take_while(|i| *i < cipher.len() + self.rails + (self.rails - 1) - 1).collect();
        // a map with centers as keys, and the ind of the next adjacent inds (below, above)
        let mut center_map: HashMap<usize, (usize, usize)> = HashMap::new();
        eprintln!("centers = {:?}", centers);

        // for each rail, iterate through each center
        for rail_num in 0..self.rails {
            for center in &centers {
                // if the current center hasn't yet been hit (first iteration):
                if !center_map.contains_key(center) {
                    // if the current center is within bounds, pop off the next char to the current rail
                    if center < &cipher.len() {
                        rails[rail_num].push(cipher_chars.next().unwrap());
                    }
                    // add the entry to the centers map and continue
                    center_map.insert(*center, (center.checked_sub(1).unwrap_or(0), *center + 1));
                    continue;
                }
                // For each next adjacent index of each center:
                let (min_i, max_i) = center_map.get(center).unwrap();
                for adjacent in [min_i, max_i] {
                    // if adjacent doesn't overlap with any center, it's within bounds, and there are chars left:
                    if !center_map.contains_key(adjacent) && adjacent < &cipher.len() {
                        if let Some(next_char) = cipher_chars.next() {
                            // Push the next char to the current rail
                            rails[rail_num].push(next_char);
                        }
                    }
                }
                center_map.insert(*center, (min_i.saturating_sub(1), max_i + 1));
            }
        }

        // collect chars in the rails into the output string
        let mut output = String::new();
        let mut rails_chars: Vec<Chars> = rails.iter().map(|s| s.chars()).collect();
        let mut inds = (0..self.rails).chain((1..self.rails-1).rev()).cycle();
        while output.len() < cipher.len() {
            if let Some(c) = rails_chars[inds.next().unwrap()].next() {
                output.push(c)
            }
        }
        output
    }
}

