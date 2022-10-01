use std::fmt::{Debug};
use crate::Orientation::{Horizontal, Vertical};

const CORNER: char = '+';
const H_WALL: char = '-';
const V_WALL: char = '|';


pub fn count(lines: &[&str]) -> u32 {
    // unwrapping dangerously because prompt says we can assume rectangular inputs
    RectangleCollection::try_from(lines).unwrap().len()
}


///------------------------------------------------------
///  POINT
///------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y:usize) -> Self {
        Point { x, y }
    }
}

///------------------------------------------------------
///  ORIENTATION
///------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
enum Orientation {
    Vertical,
    Horizontal,
}

impl TryFrom<(&Point, &Point)> for Orientation {
    type Error = String;

    fn try_from(pts: (&Point, &Point)) -> Result<Self, Self::Error> {
        let (pt1, pt2) = pts;
        if pt1.y == pt2.y {
            Ok(Horizontal)
        } else if pt1.x == pt2.x {
            Ok(Vertical)
        } else {
            Err("points must be in line on either the x or the y axis".to_string())
        }
    }
}

///------------------------------------------------------
///  EDGE
///------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct Edge {
    start: Point,
    end: Point,
    orientation: Orientation,
}

impl Edge {
    fn new(start: Point, end: Point) -> Result<Self, String> {
        let orientation = Orientation::try_from((&start, &end))?;
        let mut edge = Edge { start, end, orientation };
        edge.orient();
        Ok(edge)
    }

    fn orient(&mut self) {
        let relevant_dims = match self.orientation {
            Vertical => (self.start.y, self.end.y),
            Horizontal => (self.start.x, self.end.x),
        };
        if relevant_dims.0 > relevant_dims.1 {
            std::mem::swap(&mut self.start, &mut self.end);
        }
    }
}

///------------------------------------------------------
///  EDGE COLLECTION
///------------------------------------------------------

#[derive(Debug)]
struct EdgeCollection(Vec<Edge>);

impl EdgeCollection {
    pub fn new() -> Self {
        EdgeCollection(vec![])
    }

    pub fn push(&mut self, edge: Edge) {
        self.0.push(edge);
    }
}

impl TryFrom<&[&str]> for EdgeCollection {
    type Error = String;

    fn try_from(input: &[&str]) -> Result<Self, Self::Error> {
        if !input.is_empty() && !input.iter().all(|row| row.len() == input[0].len()) {
            return Err("NON RECTANGULAR INPUT".to_string())
        }
        let mut edges = EdgeCollection::new();
        for (y, row) in input.iter().enumerate() {
            for (x, val) in row.chars().enumerate() {
                if val == CORNER {
                    for (i, right_val) in row[x+1..].to_string().chars().enumerate() {
                        match right_val {
                            CORNER => edges.push(
                                Edge::new(Point::new(x, y), Point::new(x + i + 1, y))?
                            ),
                            H_WALL => continue,
                            _ => break,
                        };
                    }
                }
            }
        }
        Ok(edges)
    }
}

///------------------------------------------------------
///  RECTANGLE
///------------------------------------------------------

#[derive(Clone, Debug)]
struct Rectangle {
    pub tl: Point,
    pub tr: Point,
    pub bl: Point,
    pub br: Point,
}

enum RectangleFromEdgesError {
    OrientationMismatch,
    Misaligned,
}

impl TryFrom<&[Edge; 2]> for Rectangle {
    type Error = RectangleFromEdgesError;

    fn try_from(edges: &[Edge; 2]) -> Result<Self, Self::Error> {
        fn from_vertical(edges: &[Edge; 2]) -> Result<Rectangle, RectangleFromEdgesError> {
            let compare_func = |e1: &&Edge, e2: &&Edge| e1.start.x.cmp(&e2.start.x);
            let first = edges.iter().min_by(compare_func).unwrap();
            let second = edges.iter().max_by(compare_func).unwrap();
            let ends_align = first.start.y == second.start.y && first.end.y == second.end.y;
            if !ends_align {
                Err(RectangleFromEdgesError::Misaligned)
            } else {
                Ok(Rectangle {
                    tl: first.start,
                    tr: second.start,
                    bl: first.end,
                    br: second.end
                })
            }
        }

        fn from_horizontal(edges: &[Edge; 2]) -> Result<Rectangle, RectangleFromEdgesError> {
            let compare_func = |e1: &&Edge, e2: &&Edge| e1.start.x.cmp(&e2.start.x);
            let first = edges.iter().min_by(compare_func).unwrap();
            let second = edges.iter().max_by(compare_func).unwrap();

            if !(first.start.x == second.start.x && first.end.x == second.end.x) {
                Err(RectangleFromEdgesError::Misaligned)
            } else {
                Ok(Rectangle {
                    tl: first.start,
                    tr: first.end,
                    bl: second.start,
                    br: second.end,
                })
            }
        }

        if edges[0].orientation != edges[1].orientation {
            return Err(RectangleFromEdgesError::OrientationMismatch)
        }
        match edges[0].orientation {
            Vertical => from_vertical(edges),
            Horizontal => from_horizontal(edges),
        }
    }
}

///------------------------------------------------------
///  RECTANGLE COLLECTION
///------------------------------------------------------

#[derive(Clone, Debug)]
struct RectangleCollection(Vec<Rectangle>);

impl RectangleCollection {
    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn push(&mut self, rec: Rectangle) {
        self.0.push(rec)
    }

    pub fn validate_walls(&mut self, grid: &[&str]) {
        let mut remove: Vec<usize> = vec![];
        for (i, rect) in self.0.iter().enumerate() {
            let horizontal = [
                &grid[rect.tl.y][rect.tl.x..=rect.tr.x],
                &grid[rect.bl.y][rect.bl.x..=rect.br.x],
            ];
            let vertical = [
                grid[rect.tl.y..=rect.bl.y].iter().map(|row|
                    row.chars().nth(rect.tl.x).unwrap()
                ).collect::<String>(),
                grid[rect.tr.y..=rect.br.y].iter().map(|row|
                    row.chars().nth(rect.tr.x).unwrap()
                ).collect::<String>(),
            ];
            let horiz_okay = horizontal.iter().all(|s|
                s.chars().all(|c| c == CORNER || c == H_WALL)
            );
            let vert_okay = vertical.iter().all(|s|
                s.chars().all(|c| c == CORNER || c == V_WALL)
            );
            if !(horiz_okay && vert_okay) {
                remove.push(i);
            }
        }
        for i in remove.into_iter().rev() {
            self.0.remove(i);
        }
    }
}

impl From<EdgeCollection> for RectangleCollection {
    fn from(edge_collection: EdgeCollection) -> Self {
        let mut rectangles = RectangleCollection(vec![]);
        let EdgeCollection(edges) = edge_collection;
        for (i, edge1) in edges.iter().enumerate() {
            for edge2 in edges[i+1..].iter() {
                if let Ok(rect) = Rectangle::try_from(&[*edge1, *edge2]) {
                    rectangles.push(rect)
                }
            }
        }
        rectangles
    }
}

impl TryFrom<&[&str]> for RectangleCollection {
    type Error = String;

    fn try_from(input: &[&str]) -> Result<Self, Self::Error> {
        let edges = EdgeCollection::try_from(input)?;
        let mut rectangles = RectangleCollection::from(edges);
        rectangles.validate_walls(input);
        Ok(rectangles)
    }
}
