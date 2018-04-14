use num::Float;

pub struct Positive<T: Float> {
    pub value: T
}

impl<T: Float> Positive<T> {
    pub fn new(t: T) -> Positive<T> {
        assert!(t >= T::zero());
        Positive{ value : t }
    }
}

pub fn sqrt<T: Float>(v: Positive<T>) -> T {
    v.value.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        assert!(sqrt(Positive::new(4.0)) == 2.0);        
    }
}
