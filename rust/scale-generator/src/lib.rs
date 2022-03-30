use crate::ScaleInterval::{Augmented, HalfStep, WholeStep};
use crate::ScaleModifier::{Flat, Sharp};

///-------------------------------------------------------------------------------------------------
///  ScaleInterval
///-------------------------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug)]
pub enum ScaleInterval {
    HalfStep = 1,
    WholeStep = 2,
    Augmented = 3,
}

impl ScaleInterval {
    /// Given a string of interval representations ('m' = Halfstep, 'M' = WholeStep, 'A' = Augmented)
    /// return a result containing a new Vec of ScaleIntervals representing those same steps
    fn intervals_from_str(arr: &str) -> Result<Vec<Self>, Error> {
        let mut output: Vec<Self> = Vec::new();
        for char in arr.chars() {
            match ScaleInterval::from(char) {
                Ok(interval) => { output.push(interval); },
                Err(e) => { return Err(e) }
            }
        }
        Ok(output)
    }

    /// Given a character ('m', 'M', or 'A') get the corresponding ScaleInterval
    fn from(interval_string: char) ->  Result<Self, Error> {
        match interval_string {
            'm' => Ok(HalfStep),
            'M' => Ok(WholeStep),
            'A' => Ok(Augmented),
            _  => Err(Error::InvalidInterval),
        }
    }
}

///-------------------------------------------------------------------------------------------------
///  ScaleModifier
///-------------------------------------------------------------------------------------------------
/// For a given scale, should we use the Sharp or Flat variant of modified notes
#[derive(Debug, Clone, Copy)]
pub enum ScaleModifier {
    Sharp = 0,
    Flat = 1,
}


///-------------------------------------------------------------------------------------------------
///  Note
///-------------------------------------------------------------------------------------------------
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Note {
    White(&'static str),
    Black((&'static str, &'static str)),
}

impl Note {
    /// Chromatic scale starting from C as const
    pub const ALL: [Note; 12] = [
        Note::White("C"),
        Note::Black(("C#", "Db")),
        Note::White("D"),
        Note::Black(("D#", "Eb")),
        Note::White("E"),
        Note::White("F"),
        Note::Black(("F#", "Gb")),
        Note::White("G"),
        Note::Black(("G#", "Ab")),
        Note::White("A"),
        Note::Black(("A#", "Bb")),
        Note::White("B")
    ];

    /// Get the note that ist 'interval' up from 'start_note'
    pub fn interval_from(start_note: &Note, interval: &ScaleInterval) -> Note {
        let start_num = start_note.note_number() as usize;
        let interval = *interval as usize ;
        Self::ALL[(start_num + interval) % (Self::ALL.len() as usize)].clone()
    }

    /// String representation of the note, given the desired modifier (Sharp of Flat)
    pub fn to_str(&self, modifier: &ScaleModifier) -> &str {
        match self {
            Note::White(note) => note,
            Note::Black((sharp, flat)) => {
                match modifier {
                    Sharp => sharp,
                    Flat => flat,
                }
            }
        }
    }

    /// Create a new note from a string representing that note, or None if 'note_str' is invalid
    pub fn from(note_str: &str) -> Result<Self, Error> {
        let note_str = Note::fix_case(note_str);
        for note in Self::ALL {
            match note {
                Note::White(n) => {
                    if n == note_str { return Ok(note) }
                },
                Note::Black((sharp, flat)) => {
                    if note_str == sharp || note_str == flat { return Ok(note) }
                }
            }
        }
        Err(Error::InvalidNote)
    }

    /// This note's number relative from C (0-11)
    pub fn note_number(&self) -> u8 {
        for (i, &n) in Self::ALL.iter().enumerate() {
            if n == *self {
                return i as u8;
            }
        }
        0
    }

    /// Given a string representing a note, get a propperly capitalized version
    pub fn fix_case(note_str: &str)-> String {
        let mut c = note_str.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

///-------------------------------------------------------------------------------------------------
///  Error
///-------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum Error {
    InvalidNote,
    InvalidInterval,
}


///-------------------------------------------------------------------------------------------------
///  Scale
///-------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Scale {
    modifier: ScaleModifier, // use Sharps or Flats?
    notes: Vec<Note>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let root_res = Note::from(tonic);
        let intervals_res = ScaleInterval::intervals_from_str(intervals);
        match (root_res, intervals_res) {
            (Ok(root), Ok(intervals)) => {
                let modifier = Scale::determine_modifier(tonic);
                let notes = Scale::create_note_arr(&root, &intervals);
                Ok(Scale { modifier: modifier.clone(), notes: notes.to_owned() })
            },
            (Ok(_), Err(e)) => Err(e),
            _ => Err(Error::InvalidNote),
        }
    }

    /// Get the full octave chromatic scale starting from 'tonic'
    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    /// Get all notes in this scale as a vec of their string representations
    pub fn enumerate(&self) -> Vec<String> {
        self.notes.iter().map(|n| n.to_str(&self.modifier).to_owned()).collect()
    }

    /// Given the root note string, determine whether the scale should use Sharp or Flat notes
    fn determine_modifier(root: &str) -> ScaleModifier {
        match root {
            "C" | "G" | "D" | "A" | "E" | "B" | "F#" | "a" | "e"| "b"| "f#"| "c#"| "g#"| "d#" => Sharp,
            _ => Flat,
        }
    }

    /// Given the root and a vec of intervals, create a corresponding vec of Note objects
    fn create_note_arr(root: &Note, intervals: &Vec<ScaleInterval>) -> Vec<Note> {
        let mut output: Vec<Note> = vec![(*root).clone()];
        for interval in intervals {
            output.push(Note::interval_from(&output.last().unwrap(), &interval))
        }
        output
    }
}