pub mod nodes;

use std::rc::Rc;

use nodes::{expr::{Expr, Type}, stmt::{Stmt, Var}};

use gecko_lexer::{token::Token, ttype::TType};
use gecko_error::{Error, LineInfo};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,

}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.stmt()?);
        }

        Ok(stmts)

    }

    fn stmt(&mut self) -> Result<Stmt, Error> {
        if self.match_token(vec![TType::LET]) {
            return self.var_decl();
        }

        self.expr_stmt()
    }

    fn var_decl(&mut self) -> Result<Stmt, Error> {
        if let TType::Identifier(name) = self.peek().ttype {
            self.advance();
            let value = if self.match_token(vec![TType::EQ]) {
                Some(self.expression()?)
            } else {
                None
            };

            self.consume(TType::SEMICOLON, "Expect ';' after variable declaration.".to_string())?;

            return Ok(Stmt::VarDecl(Var::new(name, value)));


        } else {
            let tok = self.peek();
            return Err(Error::new_with_notes(
                LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                "Expect variable name.".to_string(),
                vec![],
            ));
        }

    }

    fn expr_stmt(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;

        if !self.match_token(vec![TType::SEMICOLON]) {
            let tok = self.previous();
            return Err(Error::new_with_notes(
                LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                "Expect ';' after expression.".to_string(),
                vec![],
            ));
        }

        Ok(Stmt::ExprStmt(expr))
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TType::EQEQ, TType::BANGEQ]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Rc::new(expr), operator, Rc::new(right));
        }

        Ok(expr)
    }

    #[allow(unused_mut)]
    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        //while self.match_token(vec![TType::G, TType::GREATEREQ, TType::LESS, TType::LESSEQ]) {
        //    let operator = self.previous();
        //    let right = self.term()?;
        //    expr = Expr::Binary(Rc::new(expr), operator, Rc::new(right));
        //}

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let expr = self.factor()?;
        while self.match_token(vec![TType::MINUS, TType::PLUS]) {
            let operator = self.previous();
            let right = self.term()?;

            return Ok(Expr::Binary(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let expr = self.unary()?;
        while self.match_token(vec![TType::SLASH, TType::STAR]) {
            let operator = self.previous();
            let right = self.factor()?;

            return Ok(Expr::Binary(
                Rc::new(expr),
                operator,
                Rc::new(right),
            ));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TType::BANG, TType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary(operator, Rc::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TType::FALSE]) {
            return Ok(Expr::Literal(Type::Bool(false)))
        } else if self.match_token(vec![TType::TRUE]) {
            return Ok(Expr::Literal(Type::Bool(true)))
        } else if self.match_token(vec![TType::LPAREN]) {
            let expr = self.expression()?;

            if !self.match_token(vec![TType::RPAREN]) {
                let tok = self.previous();
                return Err(Error::new_with_notes(
                    LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                    "Expect ')' after expression.".to_string(),
                    vec![],
                ));
            } else {
                return Ok(Expr::Grouping(Rc::new(expr)));
            }
        }

        match self.peek().ttype {
            TType::Number(num) => {
                self.advance();
                Ok(Expr::Literal(Type::Float(num)))
            }
            TType::String(string) => {
                self.advance();
                Ok(Expr::Literal(Type::String(string)))
            }
            TType::Identifier(_) => {
                self.advance();
                Ok(Expr::Variable(self.previous()))
            }
            _ => {
                let tok = self.peek();
                Err(Error::new_with_notes(
                    LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                    "Expect expression.".to_string(),
                    vec![],
                ))
            }
        }
    }

    fn consume(&mut self, ttype: TType, message: String) -> Result<Token, Error> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            let tok = self.peek();
            Err(Error::new_with_notes(
                LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                message,
                vec![],
            ))
        }
    }

    fn match_token(&mut self, ttypes: Vec<TType>) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: TType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ttype == ttype
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }


    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}