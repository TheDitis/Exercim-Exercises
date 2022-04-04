#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Active,
    Complete,
}

pub enum ScoreKind {
    Open(u32),
    Strike,
    Spare,
}

impl ScoreKind {
    fn from_frame(frame: (u32, u32)) -> Self {
        match frame {
            (10, 0) => ScoreKind::Strike,
            (a, b) => {
                match a + b {
                    10 => ScoreKind::Spare,
                    sum => ScoreKind::Open(sum),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    cur_frame: (Option<u32>, Option<u32>),
    roll_num: u8,
    status: GameStatus,
    pins: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            // roll_history: Vec::new(),
            cur_frame: (None, None),
            roll_num: 0,
            status: GameStatus::Active,
            pins: 10,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if !self.is_active() { return Err(Error::GameComplete) }
        if pins > self.pins { return Err(Error::NotEnoughPinsLeft) }

        // TODO: HERE
        OK(())
    }

}
