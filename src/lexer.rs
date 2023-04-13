use std::vec;
use super::prefix_tree::PrefixTree;
use super::prefix_tree_cursor::PrefixTreeCursor;

use super::token::Token;

const DEFAULT_TOKEN_TYPE: &str = "DEFAULT_TOKEN";

pub struct Lexer {
    tokens: Vec<Token>,
    curent_token: usize,
}

impl Lexer {
    fn new() -> Self {
        Self {
            tokens: Vec::new(),
            curent_token: 0,
        }
    }

    pub fn analyse(&mut self, text: &String, keywords: &Vec<(String, String)>) {
        
        let mut main_buffer = String::new();
        let mut keyword_buffer = String::new();
        let mut prefix_tree = PrefixTree::new();

        for keyword in keywords {
            prefix_tree.add(
                keyword.0.as_str(), 
                keyword.1.as_str());
        }
        
        let mut cursor = PrefixTreeCursor::new(&prefix_tree);

        for c in text.chars() { /* ***titre***  */
            let cursor_move = cursor.try_move(c);
            if cursor_move {
                keyword_buffer.push(c);
            } else {
                match cursor.get_token() {
                    Some(token) => {
                        if main_buffer.len() > 0 {
                            self.tokens.push(Token::new(DEFAULT_TOKEN_TYPE.to_string(), main_buffer.clone()));
                            main_buffer.clear();
                        }
                        self.tokens.push(Token::new(token.to_string(), keyword_buffer.clone()));
                        keyword_buffer.clear();
                        cursor.reset();
                        if cursor.try_move(c)  {
                            keyword_buffer.push(c);
                        } else {
                            main_buffer.push(c);
                        }
                    },
                    None => {
                        main_buffer.push_str(&keyword_buffer);
                        keyword_buffer.clear();
                        cursor.reset();
                        main_buffer.push(c);
                    },
                }
            }
        }
        match cursor.get_token() {
            Some(token) => {
                if main_buffer.len() > 0 {
                    self.tokens.push(Token::new(DEFAULT_TOKEN_TYPE.to_string(), main_buffer.clone()));
                    main_buffer.clear();
                }
                self.tokens.push(Token::new(token.to_string(), keyword_buffer.clone()));
                keyword_buffer.clear();
                cursor.reset();
            },
            None => {
                main_buffer.push_str(&keyword_buffer);
                keyword_buffer.clear();
                cursor.reset();
                self.tokens.push(Token::new(DEFAULT_TOKEN_TYPE.to_string(), main_buffer.clone()));
            },
        }
    }

    pub fn pick_previous(&mut self, backward_index: usize) -> Option<Token> {
        if self.curent_token > backward_index {
            let token = self.tokens.get(self.curent_token - backward_index);
            return Some(token.unwrap().clone()); // unwrap is safe because we check if the index is valid
        }
        return None;
    }

    pub fn pick_next(&mut self, forward_index: usize) -> Option<Token> {
        if self.curent_token + forward_index < self.tokens.len() {
            let token = self.tokens.get(self.curent_token + forward_index);
            return Some(token.unwrap().clone()); // unwrap is safe because we check if the index is valid
        }
        return None;
    }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curent_token < self.tokens.len() {
            let token = self.tokens.get(self.curent_token);
            self.curent_token += 1;
            return Some(token.unwrap().clone());
        }
        return None;
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.tokens.len() - self.curent_token;
        return (len, Some(len));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut lexer = Lexer::new();
        let text = "***Ceci est un titre*****".to_string();
        let keywords = vec![
            ("***".to_string(), "tk_title1".to_string()),
            ("**".to_string(), "tk_title2".to_string())
        ];

        lexer.analyse(&text, &keywords);

        let mut iter = lexer.into_iter();
        let mut token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "tk_title1");
        assert_eq!(token.get_value(), "***");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "DEFAULT_TOKEN");
        assert_eq!(token.get_value(), "Ceci est un titre");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "tk_title1");
        assert_eq!(token.get_value(), "***");

    }
}