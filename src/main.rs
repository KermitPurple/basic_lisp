use std::str::FromStr;
use std::iter::{Peekable, Map};
use std::io::{self, Read};

#[derive(PartialEq, Clone, Debug)]
enum Token {
    LParen,
    RParen,
    Ident(String),
    Int(i64),
    Float(f64),
}

struct TokenIterator<T: Iterator<Item = u8>> {
    it: Peekable<T>,
}

impl TokenIterator<Map<io::Bytes<io::Stdin>, fn(io::Result<u8>) -> u8>> {
    fn new() -> Self {
        Self {
            it: io::stdin()
                .bytes()
                .map(Result::unwrap as fn(_) -> _)
                .peekable()
        }
    }
}

impl TokenIterator<std::str::Bytes<'static>> {
    fn from_str(s: &'static str) -> Self {
        Self {
            it: s.bytes()
                .peekable()
        }
        
    }
}

impl<T: Iterator<Item = u8>> Iterator for TokenIterator<T> {
    type Item = Result<Token, u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = State::Start;
        let mut partial = String::new();
        let mut result = None;
        while let Some(&byte) = self.it.peek() {
            let ch = byte as char;
            let mut used_ch = true;
            match (state, ch) {
                (_, ' ' | '\n' | '\t' | '\r') => {
                    used_ch = state == State::Start;
                    match state {
                        State::Start => (),
                        State::Ident => result = Some(Ok(Token::Ident(partial.clone()))),
                        State::Int => result = Some(Ok(Token::Int(i64::from_str(&partial).unwrap()))),
                        State::Float => result = Some(Ok(Token::Float(f64::from_str(&partial).unwrap()))),
                    }
                },
                (State::Start, '(') => result = Some(Ok(Token::LParen)),
                (State::Start, ')') => result = Some(Ok(Token::RParen)),
                (State::Start, 'a'..='z' | 'A'..='Z' | '_') => {
                    state = State::Ident;
                    partial.push(ch);
                }
                (State::Start | State::Int | State::Float, '0'..='9') => {
                    if state == State::Start {
                        state = State::Int;
                    }
                    partial.push(ch);
                }
                (State::Start | State::Int, '.') => {
                    state = State::Float;
                    partial.push(ch);
                }
                (State::Ident, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') => partial.push(ch),
                _ => result = Some(Err(byte)),
            }
            if used_ch {
                self.it.next();
            }
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

#[derive(PartialEq, Copy, Clone)]
enum State {
    Start,
    Ident,
    Int,
    Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_iterator_new_test() {
        let it = TokenIterator::new();
    }

    #[test]
    fn token_iterator_test() {
        let mut it = TokenIterator::from_str("(abc 123 1.3 =)");
        assert_eq!(it.next(), Some(Ok(Token::LParen)));
        assert_eq!(it.next(), Some(Ok(Token::Ident("abc".to_string()))));
        assert_eq!(it.next(), Some(Ok(Token::Int(123))));
        assert_eq!(it.next(), Some(Ok(Token::Float(1.3))));
        assert_eq!(it.next(), Some(Err(b'=')));
        assert_eq!(it.next(), Some(Ok(Token::RParen)));
        assert_eq!(it.next(), None);
    }
}

fn main() {
}
