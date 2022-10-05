use std::collections::HashMap;
use std::str::Chars;

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

