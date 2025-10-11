use std::io::Write;

use strum::EnumString;

use crate::{Interpreter, Literal, Parser, Resolver, Scanner, RUNTIME_ERROR, SYNTAX_ERROR};

#[derive(EnumString, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    Tokenize,
    Parse,
    Evaluate,
    Run,
}

fn write_err(err: &mut dyn Write, msg: &str) {
    let _ = writeln!(err, "{}", msg);
}

pub fn run(mode: Mode, input: &str, out: &mut dyn Write, err: &mut dyn Write) -> u8 {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();

    for e in scanner.errors() {
        let _ = writeln!(err, "{}", e);
    }

    if let Mode::Tokenize = mode {
        for token in tokens {
            let _ = writeln!(out, "{}", token);
        }
        return if scanner.errors().is_empty() {
            0
        } else {
            SYNTAX_ERROR
        };
    }

    match mode {
        Mode::Parse => {
            let mut parser = Parser::new(&tokens);
            match parser.expression() {
                Ok(expr) => {
                    let _ = writeln!(out, "{}", expr);
                    0
                }
                Err(e) => {
                    write_err(err, &e.to_string());
                    SYNTAX_ERROR
                }
            }
        }
        Mode::Evaluate => {
            let mut parser = Parser::new(&tokens);
            let expr = match parser.expression() {
                Ok(expr) => expr,
                Err(e) => {
                    write_err(err, &e.to_string());
                    return SYNTAX_ERROR;
                }
            };
            let mut interpreter = Interpreter::new_with_buffer();
            match interpreter.evaluate(&expr) {
                Ok(value) => {
                    let _ = out.write_all(&interpreter.drain_output());
                    match value {
                        Literal::Number(n) => {
                            let _ = writeln!(out, "{}", n);
                        }
                        v => {
                            let _ = writeln!(out, "{}", v);
                        }
                    }
                    0
                }
                Err(e) => {
                    write_err(err, &e.to_string());
                    RUNTIME_ERROR
                }
            }
        }
        Mode::Run => {
            let mut parser = Parser::new(&tokens);
            let statements = match parser.parse() {
                Ok(stmts) => stmts,
                Err(e) => {
                    write_err(err, &e.to_string());
                    return SYNTAX_ERROR;
                }
            };
            let mut interpreter = Interpreter::new_with_buffer();
            let mut resolver = Resolver::new(&mut interpreter);

            if let Err(e) = resolver.resolve(&statements) {
                write_err(err, &e.to_string());
                return SYNTAX_ERROR;
            }

            let result = interpreter.interpret(&statements);
            let _ = out.write_all(&interpreter.drain_output());
            match result {
                Ok(_) => 0,
                Err(e) => {
                    write_err(err, &e.to_string());
                    RUNTIME_ERROR
                }
            }
        }
        Mode::Tokenize => unreachable!(), // Not needed due to early return
    }
}
