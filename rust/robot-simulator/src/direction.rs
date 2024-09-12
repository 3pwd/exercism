#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
const DXDY: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
impl Direction {
    pub fn dxdy(&self) -> (i8, i8) {
        DXDY[*self as usize]
    }

    pub fn turn(&self, clockwise: bool) -> Direction {
        let discriminant = *self as i8;
        let new_discriminant = (discriminant + if clockwise { 1 } else { 3 }) % 4;
        // we did arithmetic modulo 4 so it is ok to use unsafe
        unsafe { std::mem::transmute(new_discriminant) }
    }
}
