pub struct Element {
}

pub struct Collection {
    pub elements: Vec<Element>    
}

impl Collection {
    pub fn add_element(& mut self, e: Element) {
        self.elements.push(e);
    }
    
    pub fn add_collection(& mut self, c: Collection) {
        self.elements.extend(c.elements);
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
        c.add_element(e);
        c.add_collection(c1);
    }
}
