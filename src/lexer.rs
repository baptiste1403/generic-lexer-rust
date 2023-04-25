use super::prefix_tree::PrefixTree;
use super::prefix_tree_cursor::PrefixTreeCursor;

use super::token::Token;

use regex::Regex;

const DEFAULT_TOKEN_TYPE: &str = "DEFAULT_TOKEN";

pub struct Lexer {
    tokens: Vec<Token>,
    curent_token: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            curent_token: 0,
        }
    }

    pub fn analyse(&mut self, text: &String, keywords: &Vec<(String, String)>, patterns: &Vec<(String, String)>) {
        
        let mut main_buffer = String::new();
        let mut keyword_buffer = String::new();
        let mut prefix_tree = PrefixTree::new();

        for keyword in keywords {
            prefix_tree.add(
                keyword.0.as_str(), 
                keyword.1.as_str());
        }
        
        let mut cursor = PrefixTreeCursor::new(&prefix_tree);

        for c in text.chars() {
            if cursor.try_move(c) {
                keyword_buffer.push(c);
            } else {
                if cursor.get_token().is_some() {
                    if main_buffer.len() > 0 {
                        self.tokens.push(Token::new(self.match_pattern_token(&main_buffer, patterns), main_buffer.clone()));
                    }
                    self.tokens.push(Token::new(cursor.get_token().unwrap().to_string(), keyword_buffer.clone()));
                    main_buffer.clear();
                    keyword_buffer.clear();
                } else {
                    main_buffer.push_str(&keyword_buffer);
                    keyword_buffer.clear();
                }
                cursor.reset();
                if cursor.try_move(c) {
                    keyword_buffer.push(c);
                } else {
                    main_buffer.push(c);
                }
            }
        }

        if cursor.get_token().is_some() {
            if main_buffer.len() > 0 {
                self.tokens.push(Token::new(self.match_pattern_token(&main_buffer, patterns), main_buffer.clone()));
            }
            self.tokens.push(Token::new(cursor.get_token().unwrap().to_string(), keyword_buffer.clone()));
        } else {
            main_buffer.push_str(&keyword_buffer);
            self.tokens.push(Token::new(self.match_pattern_token(&main_buffer, patterns), main_buffer.clone()));
        }
    }

    fn match_pattern_token(&self, text: &String, patterns: &Vec<(String, String)>) -> String {
        if patterns.len() == 0 { // no patterns, return default token type
            return DEFAULT_TOKEN_TYPE.to_string();
        }
        for pattern in patterns {
            let re = match Regex::new(pattern.0.as_str()) {
                Ok(re) => re,
                Err(_) => panic!("Invalid regex pattern: {}", pattern.0.as_str()),
            };
            if re.is_match(text) {
                return pattern.1.to_string();
            }
        }

        return DEFAULT_TOKEN_TYPE.to_string();
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
    fn test_parse1() {
        let mut lexer = Lexer::new();
        let text = "***Ceci est un titre*****".to_string();
        let keywords = vec![
            ("***".to_string(), "tk_title1".to_string()),
            ("**".to_string(), "tk_title2".to_string())
        ];
        let patterns = vec![
            ("[0-9]+".to_string(), "tk_number".to_string()),
            ("[a-zA-Z]+".to_string(), "tk_text".to_string())
        ];

        lexer.analyse(&text, &keywords, &patterns);

        let mut iter = lexer.into_iter();
        let mut token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "tk_title1");
        assert_eq!(token.get_value(), "***");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "tk_text");
        assert_eq!(token.get_value(), "Ceci est un titre");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "tk_title1");
        assert_eq!(token.get_value(), "***");

    }

    #[test]
    fn test_parse2() {
        let mut lexer = Lexer::new();
        let text = "##3310##".to_string();
        let keywords = vec![
            ("#".to_string(), "diese".to_string()),
        ];
        let patterns = vec![
            ("[0-9]+".to_string(), "tk_number".to_string()),
            ("[a-zA-Z]+".to_string(), "tk_text".to_string())
        ];

        lexer.analyse(&text, &keywords, &patterns);

        let mut iter = lexer.into_iter();
        let mut token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "diese");
        assert_eq!(token.get_value(), "#");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "diese");
        assert_eq!(token.get_value(), "#");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "tk_number");
        assert_eq!(token.get_value(), "3310");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "diese");
        assert_eq!(token.get_value(), "#");

        token = iter.next().unwrap();
        assert_eq!(token.get_token_type(), "diese");
        assert_eq!(token.get_value(), "#");

    }

}