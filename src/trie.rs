use std::collections::HashMap;
use crate::LetterBox;

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::new(None, false),
        }
    }

    pub fn insert(&mut self, s: String) {
        let mut curr_node = &mut self.root;
        for c in s.chars() {
            curr_node = curr_node
                .children
                .entry(c)
                .or_insert(TrieNode::new(Some(c), false))
        }

        curr_node.is_end = true;
    }

    pub fn contains_word(&self, s: String) -> bool {
        let mut curr_node = &self.root;
        for c in s.chars() {
            if let Some(node) = curr_node.children.get(&c) {
                curr_node = node;
            } else {
                return false;
            }
        }

        return curr_node.is_end;
    }

    pub fn find_valid_words(&self, letterbox: &LetterBox) -> Vec<String> {
        let mut v = Vec::new();

        for i in 0..4 {
            v.append(self.find_valid_words_helper(&self.root, letterbox, String::from(""), i).as_mut());
        }

        v
    }

    fn find_valid_words_helper(&self, node: &TrieNode, letterbox: &LetterBox, curr_word: String, idx: i32) -> Vec<String> {
        let mut words = Vec::new();
        if node.is_end {
            words.push(curr_word.clone());
        }

        for i in 0..4 {
            if i == idx {
                continue;
            }

            let possible_chars = match i {
                0 => &letterbox.top,
                1 => &letterbox.bottom,
                2 => &letterbox.left,
                3 => &letterbox.right,
                _ => panic!("invalid index for letterbox"),
            };

            for c in possible_chars {
                if let Some(new_node) = node.children.get(c) {
                    let mut s = curr_word.clone();
                    s.push(*c);
                    words.append(self.find_valid_words_helper(new_node, letterbox, s, i).as_mut());
                }
            }
        }

        words
    }
}

pub struct TrieNode {
    value: Option<char>,
    is_end: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn new(value: Option<char>, is_end: bool) -> TrieNode {
        TrieNode {
            value,
            is_end,
            children: HashMap::new(),
        }
    }

    pub fn contains_char(&self, c: char) -> bool {
        self.children.contains_key(&c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trie_contains_string() {
        let mut trie = Trie::new();
        trie.insert(String::from("hello"));
        assert_eq!(trie.contains_word(String::from("hello")), true);
    }

    #[test]
    fn trie_does_not_contain_string() {
        let trie = Trie::new();
        assert_eq!(trie.contains_word(String::from("hello")), false);
    }

    #[test]
    fn trie_contains_string_but_not_substring() {
        let mut trie = Trie::new();
        trie.insert(String::from("hello"));
        assert_eq!(trie.contains_word(String::from("hell")), false);
    }
}