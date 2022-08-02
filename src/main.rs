use std::str::FromStr;
use std::io::{self, Read};

#[derive(PartialEq, Clone, Debug)]
enum Token {
    LParen,
    RParen,
    Ident(String),
    Int(i64),
    Float(f64),
}

type BoxIter = Box<dyn Iterator<Item = u8>>;

struct TokenIterator {
    it: BoxIter,
    ungotten: Option<u8>,
}

impl TokenIterator {
    fn new() -> Self {
        io::stdin()
            .bytes()
            .map(Result::unwrap)
            .into()
    }

    fn from_str(s: &'static str) -> Self {
        Self::from(s.bytes())
    }

    fn from_box_iter(it: BoxIter) -> Self {
        Self {
            it,
            ungotten: None,
        }
    }
}

impl<T: Iterator<Item = u8> + 'static> From<T> for TokenIterator {
    fn from(it: T) -> Self {
        Self::from_box_iter(Box::new(it))
    }
}

impl Iterator for TokenIterator {
    type Item = Result<Token, char>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = State::Start;
        let mut partial = String::new();
        while let Some(byte) = self.ungotten.take().or_else(|| self.it.next()) {
            let ch = byte as char;
            match (state, ch) {
                (State::Start, ' ' | '\n' | '\t' | '\r') => (), 
                (State::Start, '(') => return Some(Ok(Token::LParen)),
                (State::Start, ')') => return Some(Ok(Token::RParen)),
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
                _ => {
                    if state != State::Start {
                        self.ungotten = Some(byte);
                    }
                    return match state {
                        State::Start => Some(Err(ch)),
                        State::Ident => Some(Ok(Token::Ident(partial))),
                        State::Int => Some(Ok(Token::Int(i64::from_str(&partial).unwrap()))),
                        State::Float => Some(Ok(Token::Float(f64::from_str(&partial).unwrap()))),
                    }
                },
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
        assert_eq!(it.next(), Some(Err('=')));
        assert_eq!(it.next(), Some(Ok(Token::RParen)));
        assert_eq!(it.next(), None);
        it = TokenIterator::from_str("(xyz()");
        assert_eq!(it.next(), Some(Ok(Token::LParen)));
        assert_eq!(it.next(), Some(Ok(Token::Ident("xyz".to_string()))));
        assert_eq!(it.next(), Some(Ok(Token::LParen)));
        assert_eq!(it.next(), Some(Ok(Token::RParen)));
        assert_eq!(it.next(), None);
    }
}

fn main() {
    for token in TokenIterator::new() {
        println!("{:?}", token);
    }
}
