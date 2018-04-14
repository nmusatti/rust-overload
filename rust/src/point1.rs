pub enum Coordinates {
    Cartesian(f64, f64),
    Polar(f64, f64)
}

#[derive(PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    
    pub fn new(c: Coordinates) -> Point {
        use self::Coordinates::*;
        match c {
            Cartesian(x, y) => Point{x,y},
            Polar(r, t) => Point{ x: r * t.cos(), y: r * t.sin() }
        }
    }
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test() {
        let c = Point::new(Coordinates::Cartesian(2.,0.));
        let p = Point::new(Coordinates::Polar(2.,0.));
        assert!(c == p);
    }
    
}
