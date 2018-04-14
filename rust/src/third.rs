pub struct Element {
}

pub struct Collection {
    pub elements: Vec<Element>    
}

pub enum Args {
    Elem(Element),
    Coll(Collection)
}
    
impl Collection {
    pub fn add(&mut self, args: Args) {
        match args {
            Args::Elem(e) => self.elements.push(e),
            Args::Coll(c) => self.elements.extend(c.elements)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let e = Element{};
        let mut c = Collection{ elements: Vec::new() };
        let c1 = Collection{ elements: Vec::new() };
        c.add(Args::Elem(e));
        c.add(Args::Coll(c1));
    }
    
}
