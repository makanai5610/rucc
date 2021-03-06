use crate::token_kind::*;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) raw_string: String,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, raw_string: String) -> Token {
        Token { kind, raw_string }
    }

    pub(crate) fn consume(&self, op: &str) -> bool {
        use TokenKind::*;
        match &self.kind {
            Reserved(kind) if kind == op => true,
            _ => false,
        }
    }

    pub(crate) fn consume_ident(&self) -> bool {
        use TokenKind::*;
        match &self.kind {
            Ident(_) => true,
            _ => false,
        }
    }

    pub(crate) fn consume_return(&self) -> bool {
        use TokenKind::*;
        match &self.kind {
            Return => true,
            _ => false,
        }
    }

    pub(crate) fn expect(&self, op: &str) {
        use TokenKind::*;
        match &self.kind {
            Reserved(kind) if kind == op => {}
            _ => panic!("expect {}. got={}", op, self.kind),
        }
    }

    pub(crate) fn expect_number(&self) -> u32 {
        use TokenKind::*;
        match &self.kind {
            Num(n) => *n,
            _ => panic!("not a number."),
        }
    }

    pub(crate) fn is_eof(&self) -> bool {
        use TokenKind::*;
        match &self.kind {
            EOF => true,
            _ => false,
        }
    }
}
