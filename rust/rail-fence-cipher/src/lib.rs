

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
        eprintln!("rails.join() = {:?}", rails.join(""));
        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let output = String::new();
        let mut rails: Vec<String> = (0..self.rails).map(|_| String::new()).collect();
        let mut inds = (0..self.rails).chain((1..self.rails-1).rev()).cycle();

        // let centers =
        // while output.len() < cipher.len() {
        //
        // }
        output
    }
}

