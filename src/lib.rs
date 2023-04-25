/*!
This crate, provide a generic lexer implementation based on a prefix tree.
It can be used to lex a text and extract tokens from it. Tokens are defined by a name and a representation,
this representation can be a string or a regular expression. After the lexing, the token can be retrieved by iterating over the lexer.
during the iteration, it is possible to pick previous or next tokens. without changing the current position.

The lexer need a list of tuples (token_representation, token_name) for the string representation 
and a list of tuples (token_representation, token_name) for the regular expression representation.

# Examples : lexing a simple text

```rust
// use lexer::lexer::Lexer;
// let mut lexer = Lexer::new();
// let text = "***Ceci est un titre***".to_string();
// let keywords = vec![
//     ("***".to_string(), "tk_title1".to_string()),
//     ("**".to_string(), "tk_title2".to_string())
//     ];
// let patterns = vec![
//     ("[0-9]+".to_string(), "tk_number".to_string()),
//     ("[a-zA-Z]+".to_string(), "tk_text".to_string())
// ];
// 
// lexer.analyse(&text, &keywords, &patterns);
// 
// for token in lexer {
//     println!("token_type: {}, value: {}", token.get_token_type(), token.get_value());
// }
```

should print :

```text
token_type: tk_title1, value: ***
token_type: tk_text, value: Ceci est un titre
token_type: tk_title1, value: ***
```

*/

mod prefix_tree;
mod prefix_tree_cursor;
pub mod token;
pub mod lexer;