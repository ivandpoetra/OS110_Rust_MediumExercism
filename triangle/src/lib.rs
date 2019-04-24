pub struct Triangle {
    a: u64, 
    b: u64, 
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if (sides[0], sides[1], sides[2]) == (0, 0, 0) || sides[2] >= sides[1] + sides[0] || sides[1] >= sides[2] + sides [0] || sides [0] >= sides[1] + sides[2] {
            return None
        }
        else {
        let tr = Triangle {a: sides[0], b: sides[1], c: sides[2]};
        Some(tr)
        }

    }

    pub fn is_equilateral(&self) -> bool {
        return self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        return self.a != self.b && self.b != self.c && self.a != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        return (self.a == self.b && self.a != self.c && self.b != self.c) || (self.b == self.c && self.b != self.a && self.c != self.a) || (self.c == self.a && self.c != self.b && self.a != self.b)
    }
}
