
// const NOTES_SHARPS: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
// const NOTES_FLATS:  [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

use std::io::stdout;
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
pub enum Note<'a> {
    White(&'a str),
    Black((&'a str, &'a str)),
}

impl Note<'_> {
    pub const ALL: [Note<'static>; 12] = [
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

    pub fn interval_from(start_note: &Note, interval: &ScaleInterval) -> Note<'static> {
        let start_num = start_note.note_number() as usize;
        let interval = *interval as usize ;
        Self::ALL[(start_num + interval) % (Self::ALL.len() as usize)].clone()
    }

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

    pub fn note_number(&self) -> u8 {
        for (i, &n) in Self::ALL.iter().enumerate() {
            if n == *self {
                return i as u8;
            }
        }
        0
    }

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
pub struct Scale<'a> {
    root: Note<'a>,
    modifier: ScaleModifier, // use Sharps or Flats?
    intervals: Vec<ScaleInterval>,
    notes: Vec<Note<'a>>,
}

impl Scale<'_> {
    pub fn new<'a>(tonic: &'a str, intervals: &str) -> Result<Scale<'a>, Error> {
        let root_res = Note::from(tonic);
        let intervals_res = ScaleInterval::intervals_from_str(intervals);
        match (root_res, intervals_res) {
            (Ok(root), Ok(intervals)) => {
                let modifier = Scale::determine_modifier(tonic).unwrap();
                let notes = Scale::create_note_arr(&root, &intervals);
                Ok(Scale { root: root.clone(), modifier: modifier.clone(), intervals, notes: notes.to_owned() })
            },
            (Ok(_), Err(e)) => Err(e),
            _ => Err(Error::InvalidNote),
        }
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.iter().map(|n| n.to_str(&self.modifier).to_owned()).collect()
    }

    fn determine_modifier(root: &str) -> Result<ScaleModifier, Error> {
        match Note::from(root) {
            Ok(root_note) => {
                Ok(
                    match root_note {
                        Note::White(k) => {
                            match k {
                                "E" | "A" | "D" | "G" | "C" => ScaleModifier::Sharp,
                                _ => ScaleModifier::Flat,
                            }
                        },
                        Note::Black((sharp, flat)) => {
                            if root.chars().nth(1).unwrap() == '#' { Sharp } else { Flat }
                        }
                    }
                )
            }
            Err(e) => Err(e)

        }
    }

    pub fn as_str_arr(&self) -> Vec<&str> {
        let mut output: Vec<&str> = Vec::new();
        for note in &self.notes {
            output.push(note.to_str(&self.modifier));
        }
        output
    }

    fn create_note_arr<'a>(root: &'a Note, intervals: &Vec<ScaleInterval>) -> Vec<Note<'a>> {
        let mut output: Vec<Note> = vec![(*root).clone()];
        for interval in intervals {
            output.push(Note::interval_from(&output.last().unwrap(), &interval))
        }
        output
    }
}
