pub trait Index {
    fn get_index(&self) -> u8;
    fn get_bit(&self) -> usize;
}

pub struct IndexSet {
    set: usize,
}

impl IndexSet {
    pub fn add<T: Index>(&mut self, val: &T) {
        self.set = self.set | (val.get_bit());
    }
    
    pub fn exists<T: Index>(&self, val: &T) -> bool {
        (self.set & val.get_bit()) == val.get_bit()
    }
}

impl<T: Index> FromIterator<T> for IndexSet {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut set = IndexSet {
            set: 0x0,
        };
        for item in iter {
            set.add(&item);
        }
        set
    }
}

pub struct IndexableChar {
    char: char,
}

impl IndexableChar {
    pub fn from_char(char: char) -> Self {
        IndexableChar {
            char
        }
    }
}

impl Index for IndexableChar {
    fn get_index(&self) -> u8 {
        if self.char.is_uppercase() { return (self.char as u8) - ('A' as u8) + 27; }
        (self.char as u8) - ('a' as u8) + 1
    }
    
    fn get_bit(&self) -> usize {
        0x1 << self.get_index() as usize 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_indexable_char_get_index() {
        let char = IndexableChar::from_char('a');
        assert_eq!(char.get_index(), 1);
        let char = IndexableChar::from_char('Z');
        assert_eq!(char.get_index(), 52);
    }
}
