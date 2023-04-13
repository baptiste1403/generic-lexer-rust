use super::prefix_tree::PrefixTree;
use super::prefix_tree::PrefixTreeNode;

pub struct PrefixTreeCursor<'a> {
    tree: &'a PrefixTree,
    current: &'a PrefixTreeNode,
}

impl<'a> PrefixTreeCursor<'a> {
    pub fn new(t: &'a PrefixTree) -> Self {
        Self {
            tree: t,
            current: &t.get_root(),
        }
    }

    pub fn try_move(&mut self, c: char) -> bool {
        match self.current.get_nodes().get(&c) {
            Some(node) => {
                self.current = node;
                return true
            },
            None => return false,
        }
    }

    pub fn get_token(&self) -> Option<&str> {
        return match self.current.get_token() {
            Some(token) => Some(token),
            None => None,
        }
    }

    pub fn reset(&mut self) {
        self.current = &self.tree.get_root();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_cursor_when_try_move_suceed() {
        let mut tree = PrefixTree::new();
        tree.add("hello", "hello");
        let mut cursor = PrefixTreeCursor::new(&tree);
        assert_eq!(cursor.try_move('h'), true);
        assert_eq!(cursor.try_move('e'), true);
        assert_eq!(cursor.try_move('l'), true);
        assert_eq!(cursor.try_move('l'), true);
        assert_eq!(cursor.try_move('o'), true);
    }

    #[test]
    fn should_not_move_cursor_when_try_move_failed() {
        let mut tree = PrefixTree::new();
        tree.add("hello", "hello");
        let mut cursor = PrefixTreeCursor::new(&tree);
        assert_eq!(cursor.try_move('b'), false);
        //verifies that the cursor is still at the root
        assert_eq!(cursor.try_move('h'), true);
    }

    #[test]
    fn should_return_token_when_get_token_exist() {
        let mut tree = PrefixTree::new();
        tree.add("hello", "tk_hello");
        let mut cursor = PrefixTreeCursor::new(&tree);
        assert_eq!(cursor.try_move('h'), true);
        assert_eq!(cursor.try_move('e'), true);
        assert_eq!(cursor.try_move('l'), true);
        assert_eq!(cursor.try_move('l'), true);
        assert_eq!(cursor.try_move('o'), true);
        assert_eq!(cursor.get_token(), Some("tk_hello"));
    }

    #[test]
    fn should_return_none_when_token_does_not_exist() {
        let mut tree = PrefixTree::new();
        tree.add("hello", "tk_hello");
        let mut cursor = PrefixTreeCursor::new(&tree);
        assert_eq!(cursor.try_move('h'), true);
        assert_eq!(cursor.get_token(), None);
    }

    #[test]
    fn should_set_sursor_to_root_when_reset() {
        let mut tree = PrefixTree::new();
        tree.add("hello", "tk_hello");
        let mut cursor = PrefixTreeCursor::new(&tree);
        assert_eq!(cursor.try_move('h'), true);
        assert_eq!(cursor.try_move('e'), true);
        assert_eq!(cursor.try_move('l'), true);
        assert_eq!(cursor.try_move('l'), true);
        assert_eq!(cursor.try_move('o'), true);
        cursor.reset();
        assert_eq!(cursor.try_move('h'), true);
    }
}