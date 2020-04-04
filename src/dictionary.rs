use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Dictionary {
    words: Vec<Vec<u8>>,
    tree: WordsTree
}

impl Index<usize> for Dictionary {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.words[index];
    }
}

impl IndexMut<usize> for Dictionary {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.words[index];
    }
}

impl Dictionary {
    const MAX_SIZE:usize = 4194304;
    pub fn new() -> Self {
        let mut res = Self {
            words: Vec::new(),
            tree: WordsTree::new()
        };
        for i in 0..=255 {
            res.add(vec![i]);
        }
        res
    }

    pub fn vec_eq<A>(a: &Vec<A>, b: &Vec<A>) -> bool
        where A: PartialEq
    {
        if a.len() != b.len() {
            return false;
        }
        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }

    pub fn word_position(&self, seq: &Vec<u8>) -> Option<usize> {
        self.tree.get(seq.as_slice())
        /*for i in 0..self.words.len() {
            if Self::vec_eq(&seq, &self.words[i]) {
                return Some(i);
            }
        }
        None*/
    }

    pub fn add(&mut self, seq: Vec<u8>) -> Option<usize> {
        match self.word_position(&seq) {
            None => {
                return if self.words.len() < Self::MAX_SIZE {
                    self.tree.add(seq.as_slice(), self.words.len());
                    //eprintln!("{} {:?}", self.words.len(), seq);
                    self.words.push(seq);
                    Some(self.words.len())
                } else {
                    None
                };
            }
            Some(v) => Some(v)
        }
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }
}

#[derive(Debug, Clone)]
pub enum WordsTree{
    Node(usize, Box<Vec<WordsTree>>),
    Leaf
}

impl WordsTree{
    fn new() -> Self{
        WordsTree::Node(0, Box::new(vec![WordsTree::Leaf; 256]))
    }

    fn add(&mut self, path: &[u8], index: usize){
        match self {
            WordsTree::Node(_, rest) => {
                let x = path[0];
                rest[x as usize].add(&path[1..], index);
            }
            WordsTree::Leaf => {
                *self = WordsTree::Node(index, Box::new(vec![WordsTree::Leaf; 256]));
            }
        }
    }

    fn get(&self, path: &[u8]) -> Option<usize>{
        return match self {
            WordsTree::Node(index, rest) => {
                if path.is_empty() {
                    return Some(*index);
                }
                let x = path[0];
                return rest[x as usize].get(&path[1..])
            }
            WordsTree::Leaf => {
                None
            }
        }
    }
}

#[cfg(test)]
mod dict_test {
    use crate::dictionary::WordsTree;

    #[test]
    fn tree_test() {
        let mut tree = WordsTree::new();
        tree.add(&[0], 0);
        tree.add(&[1], 1);
        tree.add(&[0, 1], 2);
        assert_eq!(tree.get(&[0,1]), Some(2))
    }
}