pub mod nodes;

use std::{rc::Rc, vec};

use nodes::{expr::{Expr, Type, BinaryExpr, UnaryExpr, LiteralExpr, GroupingExpr, CallExpr, GetExpr}, stmt::{Stmt, Var, Fn}, Param};

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
        } else if self.match_token(vec![TType::FN]) {
            return self.fn_decl();
        } else if self.match_token(vec![TType::RETURN]) {
            return self.return_stmt();
        } else if self.match_token(vec![TType::IMPORT]) {
            return self.import_stmt();
        }

        self.expr_stmt()
    }

    fn import_stmt(&mut self) -> Result<Stmt, Error> {
        if let TType::String(path) = self.peek().ttype {
            self.advance();
            self.consume(TType::SEMICOLON, "Expect ';' after import path.".to_string())?;
            Ok(Stmt::FileImport(path))
        } else if let TType::Identifier(name) = self.peek().ttype {
            let mut paths = vec![name];
            self.advance();

            while self.match_token(vec![TType::DOT]) {
                if let TType::Identifier(name) = self.peek().ttype {
                    paths.push(name);
                    self.advance();
                } else {
                    let tok = self.peek();
                    return Err(Error::new_with_notes(
                        LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                        "Expect import path.".to_string(),
                        vec![],
                    ));
                }
            }

            println!("{:?}", self.peek());


            self.consume(TType::SEMICOLON, "Expect ';' after import path.".to_string())?;

            Ok(Stmt::LangImport(paths))
        } else {
            let tok = self.peek();
            return Err(Error::new_with_notes(
                LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                "Expect import path.".to_string(),
                vec![],
            ));
        }
    }

    fn fn_decl(&mut self) -> Result<Stmt, Error> {
        if let TType::Identifier(name) = self.peek().ttype {
            self.advance();
            self.consume(TType::LPAREN, "Expect '(' after function name.".to_string())?;
            let mut params: Vec<Param> = Vec::new();
            if !self.check(TType::RPAREN) {
                loop {
                    if params.len() >= 255 {
                        let tok = self.peek();
                        return Err(Error::new_with_notes(
                            LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                            "Cannot have more than 255 parameters.".to_string(),
                            vec![],
                        ));
                    }

                    if let TType::Identifier(name) = self.peek().ttype {
                        self.advance();
                        self.consume(TType::COLON, "Expect type after param".to_string())?;
                        let tok = self.advance();


                        let param = Param::new(name, tok);
                        params.push(param);
                    } else {
                        let tok = self.peek();
                        return Err(Error::new_with_notes(
                            LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                            "Expect parameter name.".to_string(),
                            vec![],
                        ));
                    }

                    if !self.match_token(vec![TType::COMMA]) {
                        break;
                    }
                }
            }
            self.consume(TType::RPAREN, "Expect ')' after parameters.".to_string())?;

            let return_value = if self.match_token(vec![TType::ARROW]) {
                Some(self.advance())
            } else {
                None
            };

            self.consume(TType::LBRACE, "Expect '{' before function body.".to_string())?;

            let body = self.block()?;
            Ok(Stmt::FnDecl(
                Fn::new(name, params, body, return_value)
            ))
        } else {
            let tok = self.peek();
            return Err(Error::new_with_notes(
                LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                "Expect function name.".to_string(),
                vec![],
            ));
        }
    }

    fn return_stmt(&mut self) -> Result<Stmt, Error> {
        let _keyword = self.previous();
        let value = if !self.check(TType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TType::SEMICOLON, "Expect ';' after return value.".to_string())?;

        Ok(Stmt::Return(value))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts = Vec::new();

        while !self.check(TType::RBRACE) && !self.is_at_end() {
            stmts.push(self.stmt()?);
        }

        self.consume(TType::RBRACE, "Expect '}' after block.".to_string())?;

        Ok(stmts)
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
            expr = Expr::Binary(BinaryExpr::new(Rc::new(expr), operator, Rc::new(right)));
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
                BinaryExpr::new(Rc::new(expr), operator, Rc::new(right)),
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
                BinaryExpr::new(Rc::new(expr), operator, Rc::new(right)),
            ));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TType::BANG, TType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary(
                UnaryExpr::new(operator, Rc::new(right)),
            ));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![TType::LPAREN]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(vec![TType::DOT]) {
                println!("{:?}", self.peek());
                let name = self.consume_any_identifier("Expect property name after '.'.".to_string())?;

                expr = Expr::Get(
                    GetExpr::new(Rc::new(expr), name),
                );
            }
            else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, expr: Expr) -> Result<Expr, Error> {
        let mut args = Vec::new();

        if !self.check(TType::RPAREN) {
            loop {
                if args.len() >= 255 {
                    let tok = self.peek();
                    return Err(Error::new_with_notes(
                        LineInfo::new(tok.lineinfo.line, tok.lineinfo.start, tok.lineinfo.end),
                        "Cannot have more than 255 arguments.".to_string(),
                        vec![],
                    ));
                }

                args.push(Rc::new(self.expression()?));

                if !self.match_token(vec![TType::COMMA]) {
                    break;
                }
            }
        }

        let paren = self.consume(TType::RPAREN, "Expect ')' after arguments.".to_string())?;

        Ok(Expr::Call(
            CallExpr::new(Rc::new(expr), paren, args),
        ))
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TType::FALSE]) {
            return Ok(Expr::Literal(
                LiteralExpr::new(Type::Bool(false)),
            ))
        } else if self.match_token(vec![TType::TRUE]) {
            return Ok(Expr::Literal(
                LiteralExpr::new(Type::Bool(true)),
            ))
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
                return Ok(Expr::Grouping(
                    GroupingExpr::new(Rc::new(expr)),
                ));
            }
        }

        match self.peek().ttype {
            TType::Number(num) => {
                self.advance();
                Ok(Expr::Literal(
                    LiteralExpr::new(Type::Float(num)),

                ))
            }
            TType::String(string) => {
                self.advance();
                Ok(Expr::Literal(
                    LiteralExpr::new(Type::String(string)),
                ))
            }
            TType::Identifier(name) => {
                self.advance();
                Ok(Expr::Literal(
                    LiteralExpr::new(Type::Iden(name)),
                ))
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

    fn consume_any_identifier(&mut self, message: String) -> Result<Token, Error> {
        if let TType::Identifier(_) = self.peek().ttype {
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
