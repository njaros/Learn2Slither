use std::collections::BTreeMap;
use playground::Dir;

#[derive(Clone)]
pub struct TrieNode {
    end: bool,
    next: BTreeMap<u8, TrieNode>,
    dir: Dir
}

pub struct Trie {
    root: TrieNode,
    cursor: TrieNode
}

impl Trie {
    pub fn from(words: &Vec<(Vec<u8>, Dir)>) -> Trie {
        let root = words
            .iter()
            .fold(TrieNode {
                end: false,
                next: BTreeMap::<u8, TrieNode>::new(),
                dir: Dir::Down
            }, |mut acc, (word, dir)| {
                let mut current = &mut acc;
                for i in 0..word.len() {
                    if current.next.get(&word[i]).is_none() {
                        current.next.insert(word[i], TrieNode {
                            end: false,
                            next: BTreeMap::<u8, TrieNode>::new(),
                            dir: dir.clone()
                        });
                    }
                    current = current.next.get_mut(&word[i]).unwrap();
                }
                current.end = true;
                acc
            });

        // Cloning because rust....
        Trie {
            root: root.clone(),
            cursor: root
        }
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = self.root.clone();
    }

    pub fn seek(&mut self, char: u8) -> Option<Dir> {
        match self.cursor.next.get(&char) {
            None => {
                self.cursor = self.root.clone();
                match self.cursor.next.get(&char) {
                    None => None,
                    Some(node) => {
                        self.cursor = node.clone();
                        if self.cursor.end {
                            return Some(self.cursor.dir.clone());
                        }
                        None
                    }
                }
            },
            Some(node) => {
                self.cursor = node.clone();
                if self.cursor.end {
                    return Some(self.cursor.dir.clone());
                }
                None
            }
        }
    }
}
