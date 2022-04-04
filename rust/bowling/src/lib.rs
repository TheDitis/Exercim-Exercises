
#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    FrameFull,
}

#[derive(Copy, Clone, Debug)]
enum FrameScore {
    Open(u8),
    Strike,
    Spare,
}

impl FrameScore {
    const MAX: u8 = 10;

    fn new(score: u8) -> Result<FrameScore, Error> {
        match score {
            Self::MAX => Ok(FrameScore::Strike),
            0..=Self::MAX => Ok(FrameScore::Open(score)),
            _ => Err(Error::NotEnoughPinsLeft),
        }

    }

    fn add(&mut self, score: u8) -> Result<(), Error> {
        match *self {
            Self::Open(ref mut cur_score) => {
                if *cur_score + score > Self::MAX {
                    return Err(Error::NotEnoughPinsLeft);
                }
                else if *cur_score + score == Self::MAX {
                    *self = FrameScore::Spare
                }
                else {
                    *self = FrameScore::Open(*cur_score + score);
                }
            },
            _ => {
                return Err(Error::NotEnoughPinsLeft)
            }
        }
        Ok(())
    }

    fn num_rolls_multiplier(&self) -> Option<u8> {
        match self {
            Self::Open(_) => None,
            Self::Spare => Some(1),
            Self::Strike => Some(2),
        }
    }

    fn points(&self) -> u8 {
        match &self {
            Self::Open(n) => *n,
            _ => 10
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Frame {
    rolls: (u8, Option<u8>),
    score: FrameScore,
}

impl Frame {
    // const MAX_ROLLS: u8 = 2;
    fn new(score: u8) -> Result<Self, Error> {
        if score > FrameScore::MAX { return Err(Error::NotEnoughPinsLeft) }
        Ok(Frame { rolls: (score, None), score: FrameScore::new(score).unwrap() })
    }

    fn add_score(&mut self, score: u8) -> Result<(), Error> {
        if self.is_complete() { return Err(Error::FrameFull); };
        // if self.rolls.0 + score > FrameScore::MAX { return Err(Error::NotEnoughPinsLeft) }
        self.rolls.1 = Some(score);
        self.score.add(score)
    }

    fn is_complete(&self) -> bool {
        matches!(self.score, FrameScore::Strike | FrameScore::Spare)
            || self.rolls.1.is_some()
    }
}


pub struct BowlingGame {
    frames: Vec<Frame>,
    cur_frame: u8,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new(), cur_frame: 0 }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if !self.is_active() { return Err(Error::GameComplete) }
        return match self.frames.get(self.cur_frame as usize) {
            // if the current frame already has a roll on it:
            Some(ref mut frame) => {
                // Try to add the additional score, and if that works, increment the score
                let _ = frame.add_score(pins)?;
                self.cur_frame += 1;
                Ok(())
            },
            // If if this is the start of a new frame
            None => {
                // Try to start the next frame
                let new_frame = Frame::new(pins)?;
                self.frames.push(new_frame);
                if new_frame.is_complete() {
                    self.cur_frame += 1;
                }
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.cur_frame != 10 || self.frames.len() < 10 { return None; }
        let mut score: u16 = 0;
        let mut multipliers: Vec<u8> = Vec::new();
        for frame in self.frames.iter() {
            let frame_points = frame.score.points() as u16;
            score += frame_points;
            println!("added {} to score", frame_points);
            for m in multipliers.iter_mut() {
                match m {
                    2  => {
                        score += frame_points;
                        *m -= 2;
                    },
                    1 => {
                        score += frame.rolls.0 as u16;
                        *m -= 1;
                    },
                    _ => ()
                }

            }
            multipliers.retain(|&v| v > 0);
            if let Some(m) = frame.score.num_rolls_multiplier() {
                multipliers.push(m);
            }

            println!("multipliers: {:?}", multipliers);
        }
        Some(score as u16)
    }

    pub fn is_active(&self) -> bool {
        self.cur_frame < 10
    }

    // fn add_bonus(&self, pins: u8) {
    //     if self.cur_frame == 0 { return }
    //     match self.frames[(self.cur_frame - 1) as usize].unwrap().score {
    //         FrameScore::Spare
    //     }
    // }

    // pub fn cur_frame_num(&self) -> Option<u8> {
    //     let mut i = 0;
    //     while let Some(_) = self.frames[i] {
    //         i += 1;
    //     }
    //     if i <= 10 { Some(i as u8) } else { None }
    // }

    pub fn print_frames(&self) {
        let unwrapped: Vec<FrameScore> = self.frames.iter().map(|f| f.score).collect();
        println!("{:?}", unwrapped);
    }
}
