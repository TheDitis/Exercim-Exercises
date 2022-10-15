use Direction::*;
enum Direction { N, S, E, W }
impl Direction {
    fn turn_right(&mut self) { *self = match self { N => E, E => S, S => W, W => N, } }
}

struct Cursor {
    r: isize,
    c: isize,
    dir: Direction,
}

impl Cursor {
    fn get_val(&self, matrix: &[Vec<u32>]) -> Option<u32> {
        if self.r < 0 || self.c < 0 { return None }
        matrix.get(self.r as usize)?.get(self.c as usize).cloned()
    }

    fn set_val(&self, matrix: &mut [Vec<u32>], val: u32) {
        matrix[self.r as usize][self.c as usize] = val
    }

    fn advance(&mut self) {
        match self.dir {
            E => self.c += 1,
            S => self.r += 1,
            W => self.c -= 1,
            N => self.r -= 1,
        }
    }

    fn retreat(&mut self) {
        match self.dir {
            E => self.c -= 1,
            S => self.r -= 1,
            W => self.c += 1,
            N => self.r += 1,
        }
    }

    fn turn_right(&mut self) { self.dir.turn_right() }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = vec![vec![0_u32; size as usize]; size as usize];
    let mut cur = Cursor { r: 0, c: 0, dir: E };
    let mut i = 0;
    while i < size * size {
        match cur.get_val(&matrix) {
            // in bounds, not marked: mark and increment i
            Some(0) => {
                cur.set_val(&mut matrix, i + 1);
                i += 1;
            },
            // out of bounds or already marked
            None | Some(_) =>  {
                cur.retreat();
                cur.turn_right();
            }
        }
        cur.advance();
    }
    matrix
}
