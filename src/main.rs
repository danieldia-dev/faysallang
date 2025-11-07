use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

// Token types for the language
#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Keywords
    Hayde,        // let/var declaration
    Hiyye,        // assignment operator
    OngNoCap,     // true
    Cap,          // false
    Eza,          // if
    Betshil,      // condition check
    Lakan,        // then
    Walla,        // else
    Deal,         // end/close block
    ThreeMol,     // print (3mol -> do/make)
    Highkey,      // emphasize/print keyword
    Lowkey,       // quiet/whisper (for debugging)
    Khalas,       // while loop (khalas -> done/finished when condition met)
    Yalla,        // continue (let's go!)
    Waqif,        // break (stop!)
    
    // Identifiers and literals
    Identifier(String),
    String(String),
    Number(f64),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,      // modulo
    
    // Comparison
    EqualEqual,   // ==
    NotEquals,    // !=
    Greater,      // >
    Less,         // <
    GreaterEq,    // >=
    LessEq,       // <=
    
    // Logical
    And,          // &&
    Or,           // ||
    Not,          // !
    
    // Delimiters
    LeftParen,
    RightParen,
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
    
    fn skip_comment(&mut self) {
        if self.current_char() == Some('/') && self.peek_char(1) == Some('/') {
            while self.current_char().is_some() && self.current_char() != Some('\n') {
                self.advance();
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
            if ch == '\\' && self.peek_char(1) == Some('n') {
                result.push('\n');
                self.advance();
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
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
        self.skip_comment();
        
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
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
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
            Some('%') => {
                self.advance();
                Token::Percent
            }
            Some('=') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Hiyye
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::NotEquals
                } else {
                    Token::Not
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::GreaterEq
                } else {
                    Token::Greater
                }
            }
            Some('<') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::LessEq
                } else {
                    Token::Less
                }
            }
            Some('&') => {
                self.advance();
                if self.current_char() == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    self.next_token()
                }
            }
            Some('|') => {
                self.advance();
                if self.current_char() == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    self.next_token()
                }
            }
            Some('3') if self.peek_char(1) == Some('m') && self.peek_char(2) == Some('o') && self.peek_char(3) == Some('l') => {
                // Special case for 3mol keyword
                self.advance(); // '3'
                self.advance(); // 'm'
                self.advance(); // 'o'
                self.advance(); // 'l'
                Token::ThreeMol
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
                    "walla" => Token::Walla,
                    "deal" => Token::Deal,
                    "highkey" => Token::Highkey,
                    "lowkey" => Token::Lowkey,
                    "khalas" => Token::Khalas,
                    "yalla" => Token::Yalla,
                    "waqif" => Token::Waqif,
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
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    And,
    Or,
}

#[derive(Debug, Clone)]
enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug, Clone)]
enum Statement {
    VarDecl {
        name: String,
        value: Expr,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    Print(Expr),
    Debug(Expr),
    If {
        condition: Expr,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    Break,
    Continue,
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
            } else {
                // If we couldn't parse a statement and didn't advance, break to prevent infinite loop
                if *self.current_token() != Token::Eof {
                    eprintln!("Warning: Could not parse token: {:?}", self.current_token());
                    self.advance();
                }
            }
        }
        
        statements
    }
    
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token() {
            Token::Hayde => self.parse_var_decl(),
            Token::ThreeMol => self.parse_print(),
            Token::Lowkey => self.parse_debug(),
            Token::Eza => self.parse_if(),
            Token::Khalas => self.parse_while(),
            Token::Yalla => {
                self.advance();
                Some(Statement::Continue)
            }
            Token::Waqif => {
                self.advance();
                Some(Statement::Break)
            }
            Token::Identifier(_) => self.parse_assignment(),
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
        
        let value = self.parse_or_expr()?;
        
        Some(Statement::VarDecl { name, value })
    }
    
    fn parse_assignment(&mut self) -> Option<Statement> {
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
        
        let value = self.parse_or_expr()?;
        
        Some(Statement::Assignment { name, value })
    }
    
    fn parse_print(&mut self) -> Option<Statement> {
        self.advance(); // skip '3mol'
        
        if *self.current_token() == Token::Highkey {
            self.advance();
        }
        
        let expr = self.parse_or_expr()?;
        Some(Statement::Print(expr))
    }
    
    fn parse_debug(&mut self) -> Option<Statement> {
        self.advance(); // skip 'lowkey'
        let expr = self.parse_or_expr()?;
        Some(Statement::Debug(expr))
    }
    
    fn parse_if(&mut self) -> Option<Statement> {
        self.advance(); // skip 'eza'
        
        if *self.current_token() == Token::Betshil {
            self.advance();
        }
        
        let condition = self.parse_or_expr()?;
        
        if *self.current_token() == Token::Lakan {
            self.advance();
        }
        
        let mut then_body = Vec::new();
        while *self.current_token() != Token::Deal 
            && *self.current_token() != Token::Walla 
            && *self.current_token() != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                then_body.push(stmt);
            }
        }
        
        let else_body = if *self.current_token() == Token::Walla {
            self.advance(); // skip 'walla'
            let mut else_stmts = Vec::new();
            while *self.current_token() != Token::Deal && *self.current_token() != Token::Eof {
                if let Some(stmt) = self.parse_statement() {
                    else_stmts.push(stmt);
                }
            }
            Some(else_stmts)
        } else {
            None
        };
        
        if *self.current_token() == Token::Deal {
            self.advance(); // skip 'deal'
        }
        
        Some(Statement::If { condition, then_body, else_body })
    }
    
    fn parse_while(&mut self) -> Option<Statement> {
        self.advance(); // skip 'khalas'
        
        if *self.current_token() == Token::Betshil {
            self.advance();
        }
        
        let condition = self.parse_or_expr()?;
        
        if *self.current_token() == Token::Lakan {
            self.advance();
        }
        
        let mut body = Vec::new();
        while *self.current_token() != Token::Deal && *self.current_token() != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            }
        }
        
        if *self.current_token() == Token::Deal {
            self.advance();
        }
        
        Some(Statement::While { condition, body })
    }
    
    // Expression parsing with proper precedence
    fn parse_or_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_and_expr()?;
        
        while *self.current_token() == Token::Or {
            self.advance();
            let right = self.parse_and_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Some(left)
    }
    
    fn parse_and_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_comparison_expr()?;
        
        while *self.current_token() == Token::And {
            self.advance();
            let right = self.parse_comparison_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Some(left)
    }
    
    fn parse_comparison_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_additive_expr()?;
        
        loop {
            let op = match self.current_token() {
                Token::EqualEqual => BinaryOp::Equals,
                Token::NotEquals => BinaryOp::NotEquals,
                Token::Greater => BinaryOp::Greater,
                Token::Less => BinaryOp::Less,
                Token::GreaterEq => BinaryOp::GreaterEq,
                Token::LessEq => BinaryOp::LessEq,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_additive_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Some(left)
    }
    
    fn parse_additive_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_multiplicative_expr()?;
        
        loop {
            let op = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_multiplicative_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Some(left)
    }
    
    fn parse_multiplicative_expr(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary_expr()?;
        
        loop {
            let op = match self.current_token() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_unary_expr()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        
        Some(left)
    }
    
    fn parse_unary_expr(&mut self) -> Option<Expr> {
        match self.current_token() {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary_expr()?;
                Some(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary_expr()?;
                Some(Expr::Unary {
                    op: UnaryOp::Minus,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary_expr(),
        }
    }
    
    fn parse_primary_expr(&mut self) -> Option<Expr> {
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
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_or_expr()?;
                if *self.current_token() == Token::RightParen {
                    self.advance();
                }
                Some(expr)
            }
            _ => None,
        }
    }
}

// Value types for runtime
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", if *b { "ong_no_cap" } else { "cap" }),
        }
    }
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }
    
    fn to_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Bool(true) => 1.0,
            Value::Bool(false) => 0.0,
            Value::String(s) => s.parse().unwrap_or(0.0),
        }
    }
}

// Control flow signals
#[derive(Debug)]
enum FlowControl {
    None,
    Break,
    Continue,
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
            if let FlowControl::Break = self.execute_statement(stmt) {
                break;
            }
        }
    }
    
    fn execute_statement(&mut self, stmt: Statement) -> FlowControl {
        match stmt {
            Statement::VarDecl { name, value } => {
                let val = self.eval_expr(value);
                self.variables.insert(name, val);
                FlowControl::None
            }
            Statement::Assignment { name, value } => {
                let val = self.eval_expr(value);
                self.variables.insert(name, val);
                FlowControl::None
            }
            Statement::Print(expr) => {
                let val = self.eval_expr(expr);
                println!("{}", val);
                FlowControl::None
            }
            Statement::Debug(expr) => {
                let val = self.eval_expr(expr);
                eprintln!("[DEBUG] {}", val);
                FlowControl::None
            }
            Statement::If { condition, then_body, else_body } => {
                let cond_val = self.eval_expr(condition);
                if cond_val.is_truthy() {
                    for stmt in then_body {
                        match self.execute_statement(stmt) {
                            FlowControl::Break => return FlowControl::Break,
                            FlowControl::Continue => return FlowControl::Continue,
                            FlowControl::None => {}
                        }
                    }
                } else if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        match self.execute_statement(stmt) {
                            FlowControl::Break => return FlowControl::Break,
                            FlowControl::Continue => return FlowControl::Continue,
                            FlowControl::None => {}
                        }
                    }
                }
                FlowControl::None
            }
            Statement::While { condition, body } => {
                loop {
                    let cond_val = self.eval_expr(condition.clone());
                    if !cond_val.is_truthy() {
                        break;
                    }
                    
                    let mut should_break = false;
                    for stmt in body.clone() {
                        match self.execute_statement(stmt) {
                            FlowControl::Break => {
                                should_break = true;
                                break;
                            }
                            FlowControl::Continue => break,
                            FlowControl::None => {}
                        }
                    }
                    
                    if should_break {
                        break;
                    }
                }
                FlowControl::None
            }
            Statement::Break => FlowControl::Break,
            Statement::Continue => FlowControl::Continue,
        }
    }
    
    fn eval_expr(&self, expr: Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::String(s) => Value::String(s),
            Expr::Bool(b) => Value::Bool(b),
            Expr::Identifier(name) => {
                self.variables.get(&name).cloned().unwrap_or(Value::Number(0.0))
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.eval_expr(*left);
                let right_val = self.eval_expr(*right);
                self.eval_binary_op(left_val, op, right_val)
            }
            Expr::Unary { op, expr } => {
                let val = self.eval_expr(*expr);
                self.eval_unary_op(op, val)
            }
        }
    }
    
    fn eval_unary_op(&self, op: UnaryOp, val: Value) -> Value {
        match op {
            UnaryOp::Not => Value::Bool(!val.is_truthy()),
            UnaryOp::Minus => Value::Number(-val.to_number()),
        }
    }
    
    fn eval_binary_op(&self, left: Value, op: BinaryOp, right: Value) -> Value {
        match op {
            BinaryOp::Add => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Value::Number(l + r),
                    (Value::String(l), Value::String(r)) => Value::String(format!("{}{}", l, r)),
                    (Value::String(l), r) => Value::String(format!("{}{}", l, r)),
                    (l, Value::String(r)) => Value::String(format!("{}{}", l, r)),
                    _ => Value::Number(0.0),
                }
            }
            BinaryOp::Sub => Value::Number(left.to_number() - right.to_number()),
            BinaryOp::Mul => Value::Number(left.to_number() * right.to_number()),
            BinaryOp::Div => {
                let r = right.to_number();
                if r != 0.0 {
                    Value::Number(left.to_number() / r)
                } else {
                    Value::Number(f64::INFINITY)
                }
            }
            BinaryOp::Mod => Value::Number(left.to_number() % right.to_number()),
            BinaryOp::Equals => Value::Bool(left == right),
            BinaryOp::NotEquals => Value::Bool(left != right),
            BinaryOp::Greater => Value::Bool(left.to_number() > right.to_number()),
            BinaryOp::Less => Value::Bool(left.to_number() < right.to_number()),
            BinaryOp::GreaterEq => Value::Bool(left.to_number() >= right.to_number()),
            BinaryOp::LessEq => Value::Bool(left.to_number() <= right.to_number()),
            BinaryOp::And => Value::Bool(left.is_truthy() && right.is_truthy()),
            BinaryOp::Or => Value::Bool(left.is_truthy() || right.is_truthy()),
        }
    }
}

fn run_code(code: &str, debug: bool) {
    if debug {
        println!("Running Faysal Lang...\n");
        println!("Code:\n{}\n", code);
    }
    
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    
    if debug {
        println!("Tokens: {:?}\n", tokens);
    }
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    
    if debug {
        println!("AST ({} statements): {:#?}\n", ast.len(), ast);
        println!("Output:");
        println!("-------");
    } else if ast.is_empty() {
        eprintln!("Error: No statements were parsed from the code");
        return;
    }
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        let filename = &args[1];
        let debug = args.len() > 2 && args[2] == "--debug";
        
        let code = match std::fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                std::process::exit(1);
            }
        };
        
        if code.trim().is_empty() {
            eprintln!("Warning: File '{}' is empty", filename);
            std::process::exit(1);
        }
        
        run_code(&code, debug);
    } else {
        // REPL mode
        println!("Faysal Lang REPL v0.1.0");
        println!("Type 'exit' to quit\n");
        
        let mut interpreter = Interpreter::new();
        
        loop {
            print!("faysal> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            let input = input.trim();
            if input == "exit" {
                break;
            }
            
            if input.is_empty() {
                continue;
            }
            
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();
            
            let mut parser = Parser::new(tokens);
            let statements = parser.parse();
            
            for stmt in statements {
                interpreter.execute_statement(stmt);
            }
        }
    }
}