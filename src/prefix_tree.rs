use std::collections::HashMap;

pub struct PrefixTree
{
    root: PrefixTreeNode,
}

impl PrefixTree {
    pub fn new() -> Self {
        Self {
            root: PrefixTreeNode::new(),
        }
    }

    pub fn add(&mut self, word: &str, token: &str) {
        let mut current = &mut self.root;
        if word.len() == 0 {
            return;
        }
        for i in 0..word.len() {
            let c = word.chars().nth(i).unwrap(); // unwrap is safe because we already checked the length of the string
            if !current.nodes.contains_key(&c) {
                let node = PrefixTreeNode::new();
                current.nodes.insert(c, node);
            }
            current = match current.nodes.get_mut(&c) {
                Some(node) => node,
                None => panic!("Error, this should never happen!"),
            };
            if i == word.len() - 1 {
                current.token = Some(token.to_string());
            }
        }
    }

    pub fn get_root(&self) -> &PrefixTreeNode {
        return &self.root;
    }
    
}

pub struct PrefixTreeNode
{
    nodes: HashMap<char, PrefixTreeNode>,
    token: Option<String>,
}

impl PrefixTreeNode {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            token: None,
        }
    }

    pub fn get_nodes(&self) -> &HashMap<char, PrefixTreeNode> {
        return &self.nodes;
    }

    pub fn get_token(&self) -> Option<&str> {
        return match self.token {
            Some(ref token) => Some(token.as_str()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_store_token_on_last_letter_when_add_keyword() {
        let mut tree = PrefixTree::new();
        tree.add("hello", "tk_hello");
        let mut current = &tree.root;
        for c in "hello".chars() {
            assert!(current.nodes.contains_key(&c), "The key {c} should exist");
            current = current.nodes.get(&c).unwrap(); // unwrap is safe because we just checked that the key exists
        }
        assert_eq!(current.token, Some("tk_hello".to_string()), "The token should be set on the last letter");
    }
}