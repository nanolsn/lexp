
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ParseResult<T> {
    Ok(T, usize),
    UnexpectedAt(usize),
}

pub trait Parse<'t> {
    type Token;
    fn parse(&self, text: &'t str) -> Option<(Self::Token, usize)>;

    fn tokenize<'p>(&'p self, text: &'t str) -> ParseIterator<'p, 't, Self> where
        Self: Sized {
        ParseIterator::new(self, text)
    }
}

pub struct ParseIterator<'p, 't, P> {
    parser: &'p P,
    rest: &'t str,
    parsed_len: usize,
    end: bool,
}

impl<'p, 't, P> ParseIterator<'p, 't, P> where
    P: Parse<'t> {
    pub fn new(parser: &'p P, text: &'t str) -> Self {
        ParseIterator {
            parser,
            rest: text,
            parsed_len: 0,
            end: false,
        }
    }
}

impl<'p, 't, P> Iterator for ParseIterator<'p, 't, P> where
    P: Parse<'t> {
    type Item = ParseResult<P::Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        match self.parser.parse(self.rest) {
            Some((tok, len)) => {
                self.rest = &self.rest[len..];
                let pos = self.parsed_len;
                self.parsed_len += len;
                Some(ParseResult::Ok(tok, pos))
            },
            None if self.rest.is_empty() => None,
            None => {
                self.end = true;
                Some(ParseResult::UnexpectedAt(self.parsed_len))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::read_token::ReadToken;

    #[derive(Copy, Clone, PartialEq, Debug, ReadToken)]
    enum Token {
        Eq,
        Plus,
        Star,
        One,
        X,
        LeftBracket,
        RightBracket,
        Semicolon,
    }

    #[test]
    fn tokenize() {
        let lx =
              lex('=', Token::Eq)
            | lex('+', Token::Plus)
            | lex('*', Token::Star)
            | lex('1', Token::One)
            | lex('x', Token::X)
            | lex('(', Token::LeftBracket)
            | lex(')', Token::RightBracket)
            | lex(';', Token::Semicolon);

        let code = "x=(1+1)*x;";
        let tokens: Vec<Token> = lx.tokenize(code).map(|r| match r {
            ParseResult::Ok(tok, _) => tok,
            ParseResult::UnexpectedAt(_) => unreachable!(),
        }).collect();

        assert_eq!(tokens, [
            Token::X,
            Token::Eq,
            Token::LeftBracket,
            Token::One,
            Token::Plus,
            Token::One,
            Token::RightBracket,
            Token::Star,
            Token::X,
            Token::Semicolon,
        ]);
    }
}
