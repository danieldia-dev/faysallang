use std::collections::HashMap;
use std::fmt;

// Token types for the language
#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Keywords
    Hayde,    // let/var declaration
    Hiyye,    // assignment operator
    OngNoCap, // true
    Cap,      // false
    Eza,      // if
    Betshil,  // condition check
    Lakan,    // then
    Deal,     // end/close block
    ThreeMol, // print (3mol -> do/make)
    Highkey,  // emphasize/print keyword

    // Identifiers and literals
    Identifier(String),
    String(String),
    Number(f64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,

    // Comparison
    Equals,
    NotEquals,
    Greater,
    Less,

    // Delimiters
    Newline,
    Eof,
}

// Lexer to tokenize input
struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        let pos = self.position + offset;
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> String {
        self.advance(); // skip opening quote
        let mut result = String::new();

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance(); // skip closing quote
                break;
            }
            result.push(ch);
            self.advance();
        }

        result
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_numeric() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse().unwrap_or(0.0)
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            None => Token::Eof,
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('"') => {
                let s = self.read_string();
                Token::String(s)
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                Token::Star
            }
            Some('/') => {
                self.advance();
                Token::Slash
            }
            Some(ch) if ch.is_numeric() => {
                let num = self.read_number();
                Token::Number(num)
            }
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "hayde" => Token::Hayde,
                    "hiyye" => Token::Hiyye,
                    "ong_no_cap" => Token::OngNoCap,
                    "cap" => Token::Cap,
                    "eza" => Token::Eza,
                    "betshil" => Token::Betshil,
                    "lakan" => Token::Lakan,
                    "deal" => Token::Deal,
                    "3mol" => Token::ThreeMol,
                    "highkey" => Token::Highkey,
                    _ => Token::Identifier(ident),
                }
            }
            Some(_) => {
                self.advance();
                self.next_token()
            }
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            if token != Token::Newline {
                tokens.push(token);
            }
        }

        tokens
    }
}

// AST Node types
#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    NotEquals,
    Greater,
    Less,
}

#[derive(Debug, Clone)]
enum Statement {
    VarDecl {
        name: String,
        value: Expr,
    },
    Print(Expr),
    If {
        condition: Expr,
        body: Vec<Statement>,
    },
}

// Parser
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        if self.position < self.tokens.len() {
            &self.tokens[self.position]
        } else {
            &Token::Eof
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while *self.current_token() != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }

        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token() {
            Token::Hayde => self.parse_var_decl(),
            Token::ThreeMol => self.parse_print(),
            Token::Eza => self.parse_if(),
            _ => {
                self.advance();
                None
            }
        }
    }

    fn parse_var_decl(&mut self) -> Option<Statement> {
        self.advance(); // skip 'hayde'

        let name = if let Token::Identifier(n) = self.current_token() {
            let name = n.clone();
            self.advance();
            name
        } else {
            return None;
        };

        if *self.current_token() != Token::Hiyye {
            return None;
        }
        self.advance(); // skip 'hiyye'

        let value = self.parse_expr()?;

        Some(Statement::VarDecl { name, value })
    }

    fn parse_print(&mut self) -> Option<Statement> {
        self.advance(); // skip '3mol'

        if *self.current_token() == Token::Highkey {
            self.advance();
        }

        let expr = self.parse_expr()?;
        Some(Statement::Print(expr))
    }

    fn parse_if(&mut self) -> Option<Statement> {
        self.advance(); // skip 'eza'

        if *self.current_token() != Token::Betshil {
            return None;
        }
        self.advance(); // skip 'betshil'

        let condition = self.parse_expr()?;

        if *self.current_token() != Token::Lakan {
            return None;
        }
        self.advance(); // skip 'lakan'

        let mut body = Vec::new();
        while *self.current_token() != Token::Deal && *self.current_token() != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            }
        }

        if *self.current_token() == Token::Deal {
            self.advance(); // skip 'deal'
        }

        Some(Statement::If { condition, body })
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        match self.current_token().clone() {
            Token::Number(n) => {
                self.advance();
                Some(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Some(Expr::String(s))
            }
            Token::OngNoCap => {
                self.advance();
                Some(Expr::Bool(true))
            }
            Token::Cap => {
                self.advance();
                Some(Expr::Bool(false))
            }
            Token::Identifier(name) => {
                self.advance();
                Some(Expr::Identifier(name))
            }
            _ => None,
        }
    }
}

// Value types for runtime
#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", if *b { "ong_no_cap" } else { "cap" }),
        }
    }
}

// Interpreter
struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    fn execute(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.execute_statement(stmt);
        }
    }

    fn execute_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::VarDecl { name, value } => {
                let val = self.eval_expr(value);
                self.variables.insert(name, val);
            }
            Statement::Print(expr) => {
                let val = self.eval_expr(expr);
                println!("{}", val);
            }
            Statement::If { condition, body } => {
                let cond_val = self.eval_expr(condition);
                if let Value::Bool(true) = cond_val {
                    for stmt in body {
                        self.execute_statement(stmt);
                    }
                }
            }
        }
    }

    fn eval_expr(&self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::String(s) => Value::String(s),
            Expr::Bool(b) => Value::Bool(b),
            Expr::Identifier(name) => self
                .variables
                .get(&name)
                .cloned()
                .unwrap_or(Value::Bool(false)),
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(*left);
                let right_val = self.eval_expr(*right);
                self.eval_binary_op(left_val, op, right_val)
            }
        }
    }

    fn eval_binary_op(&self, _left: Value, _op: BinaryOp, _right: Value) -> Value {
        // Simplified for now
        Value::Bool(false)
    }
}

fn main() {
    let code = r#"
hayde x hiyye ong_no_cap
eza betshil x lakan
   3mol highkey "x is fr ong no cap"
deal
"#;

    println!("Faysal Lang Interpreter");
    println!("=======================\n");

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    println!("Tokens: {:?}\n", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("AST: {:#?}\n", ast);

    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);
}
