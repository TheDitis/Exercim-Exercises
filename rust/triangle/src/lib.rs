pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        match Triangle::are_valid_sides(sides) {
            true => Some(Triangle { sides }),
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|v| v == &self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        (0..=1).all(|i| {
            self.sides[i+1..].iter()
                .all(|side2| { &self.sides[i] != side2 })
        })
    }

    pub fn is_isosceles(&self) -> bool {
        (0..=1).any(|i| {
            self.sides[i+1..].iter()
                .any(|side2| { &self.sides[i] == side2 })
        })
        // !self.is_scalene() && !self.is_equilateral() <-- this works too but that's no fun!
    }

    fn are_valid_sides(sides: [u64; 3]) -> bool {
        sides.iter().enumerate().all(|(i, &v)|
            v > 0 && v < sides[(i + 1) % 3] + sides[(i + 2) % 3]
        )
    }
}
