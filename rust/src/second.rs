pub struct Element {
}

pub struct Collection {
    pub elements: Vec<Element>    
}

trait Addable<T> {
    fn add(& mut self, t: T);
}

impl Addable<Element> for Collection {
    fn add(& mut self, t: Element) {
        self.elements.push(t);
    }
}

impl Addable<Collection> for Collection {
    fn add(& mut self, t: Collection) {
        self.elements.extend(t.elements);
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
        c.add(e);
        c.add(c1);
    }
}
