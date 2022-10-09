use std::cmp::Ordering::{Greater, Less, Equal};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    BonusOnOpenFrame,
    ExtraBonusOnStrikeFrame,
}

#[derive(Debug)]
enum Frame {
    Open {
        rolls: (u8, Option<u8>),
        score: u8,
    },
    Spare {
        score: u8,
        bonus: Option<u8>,
    },
    Strike {
        bonus: (Option<u8>, Option<u8>),
    },
}


impl Frame {
    pub fn new(pins: u8) -> Result<Self, Error> {
        match pins.cmp(&10) {
            Equal => Ok(Self::Strike { bonus: (None, None) }),
            Less => Ok(Self::Open { rolls: (pins, None), score: pins }),
            Greater => Err(Error::NotEnoughPinsLeft)
        }
    }

    pub fn is_complete(&self) -> bool {
        !matches!(self, Frame::Open { rolls: (_, None), score: _ })
    }

    fn add_roll(&mut self, pins: u8) -> Result<(), Error> {
        match self {
            // Open frame with only one roll so far:
            Frame::Open { rolls, score } if rolls.1.is_none() => {
                match (*score + pins).cmp(&10) {
                    Greater => Err(Error::NotEnoughPinsLeft),
                    // needs to upgrade to a Spare
                    Equal => {
                        *self = Frame::Spare {
                            score: *score + pins,
                            bonus: None,
                        };
                        Ok(())
                    },
                    Less => {
                        *score += pins;
                        rolls.1 = Some(pins);
                        Ok(())
                    }
                }
            },
            _ => Err(Error::NotEnoughPinsLeft)
        }
    }

    fn awaiting_bonus(&self) -> bool {
        matches!(
            self,
            Self::Spare { bonus: None, .. }
            | Self::Strike { bonus: (_, None), .. }
        )
    }

    fn add_bonus(&mut self, bonus_pts: u8) -> Result<(), Error> {
        match self {
            Frame::Spare { bonus, .. } => *bonus = Some(bonus_pts),
            Frame::Strike { bonus, .. } => {
                match bonus {
                    (None, None) => bonus.0 = Some(bonus_pts),
                    (Some(_), None) => bonus.1 = Some(bonus_pts),
                    _ => return Err(Error::ExtraBonusOnStrikeFrame),
                }
            },
            Frame::Open { .. } => return Err(Error::BonusOnOpenFrame),
        }
        Ok(())
    }

    fn score(&self) -> u8 {
        match self {
            Self::Open { score, .. } => *score,
            Self::Spare { score, bonus, .. } => score + bonus.unwrap_or(0),
            Self::Strike { bonus, .. } => 10 + bonus.0.unwrap_or(0) + bonus.1.unwrap_or(0)
        }
    }

    fn final_frame_check(&self, pins: u8) -> Result<(), Error> {
        match self {
            Self::Strike { bonus, .. }
            if (
                bonus.1.is_some()
                    || (bonus.0.is_some() && (10 + pins + bonus.0.unwrap() > 30 || bonus.0.unwrap() != 10 && bonus.0.unwrap() + pins > 10))
            ) => {
                Err(Error::NotEnoughPinsLeft)
            },
            _ => Ok(())
        }
    }
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    cur_frame: u8,
}

impl BowlingGame {
    pub fn new() -> Self { Self::default() }

    pub fn roll(&mut self, pins: u8) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft) }
        if self.is_complete() { return Err(Error::GameComplete) }

        //// handle final last throw case (last throw(s) of 10th frame):
        if self.cur_frame == 10 {
            self.latest_frame().final_frame_check(pins)?;
            self.add_bonuses(pins);
            return Ok(())
        }

        //// every roll but the last bonus roll:
        // add any bonuses necessary to previous frames
        self.add_bonuses(pins);
        // if this is not the first roll:
        if let Some(frame) = self.frames.last_mut() {
            // if the most recent frame is complete, add a new one
            if frame.is_complete() { self.frames.push(Frame::new(pins).unwrap()); }
            // if the most recent frame is not complete, add the extra score to it
            else { frame.add_roll(pins)?; }
        }
        // if this is the first roll, add a new frame:
        else { self.frames.push(Frame::new(pins).unwrap()); }

        // if the frame we just modified is complete, increment frame_num
        self.advance_frame_if_needed();

        Ok(())

    }

    fn is_complete(&self) -> bool {
        self.frames.len() >= 10
            && self.frames.iter().all(|f| f.is_complete() && !f.awaiting_bonus())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None
        }
        Some(self.frames.iter().map(|f| f.score() as u16).sum())
    }

    fn latest_frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn add_bonuses(&mut self, pins: u8) {
        self.frames[..self.cur_frame as usize].iter_mut().rev()
            .take_while(|f| f.awaiting_bonus() && f.is_complete())
            .for_each(|f| f.add_bonus(pins).unwrap())
    }

    fn advance_frame_if_needed(&mut self) {
        let last_frame = self.frames.last().unwrap();
        let standard_frame_complete = self.cur_frame < 10 && last_frame.is_complete();
        let last_frame_complete =
            self.cur_frame == 10 && last_frame.is_complete() && !last_frame.awaiting_bonus();
        if standard_frame_complete || last_frame_complete {
            self.cur_frame += 1
        }
    }
}