#[derive(Debug, Clone)]
pub struct CustomSet<T> {
    elements: Vec<T>,
}

impl<T: Copy + PartialEq> PartialEq<CustomSet<T>> for CustomSet<T> {
    fn eq(&self, other: &CustomSet<T>) -> bool {
        for elem in &self.elements {
            if !other.contains(elem) {
                return false;
            }
        }
        self.elements.len() == other.elements.len()
    }
}

impl<T: Copy + PartialEq> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut set = CustomSet { elements: vec![] };
        for elem in _input {
            set.add(*elem);
        }
        set
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.elements.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.elements.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        for elem in &self.elements {
            if !_other.contains(elem) {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        for elem in &self.elements {
            if _other.contains(elem) {
                return false;
            }
        }
        true
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        let mut set = CustomSet { elements: vec![] };
        for elem in &self.elements {
            if _other.contains(elem) {
                set.add(*elem);
                continue;
            }
        }
        set
    }

    pub fn difference(&self, _other: &Self) -> Self {
        let mut set = CustomSet { elements: vec![] };
        for elem in &self.elements {
            if !_other.contains(elem) {
                set.add(*elem);
                continue;
            }
        }
        set
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut set = (*self).clone();
        for elem in &_other.elements {
            set.add(*elem);
        }
        set
    }
}
