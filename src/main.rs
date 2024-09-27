use swc_common::BytePos;
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax, TsSyntax};

fn parse_js(code: &str) {
    // create lexer
    let lexer = Lexer::new(
        Syntax::Es(EsSyntax {
            jsx: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::new(code, BytePos(0), BytePos(code.len() as u32)),
        None,
    );

    // create parser
    let mut parser = Parser::new_from(lexer);

    // parse
    let ast = parser.parse_script().expect("failed to parse JavaScript");
    println!("{:#?}", ast);
}

fn parse_ts(code: &str) {
    // create lexer
    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::new(code, BytePos(0), BytePos(code.len() as u32)),
        None,
    );

    // create parser
    let mut parser = Parser::new_from(lexer);

    // parse
    let ast = parser.parse_script().expect("failed to parse TypeScript");
    println!("{:#?}", ast);
}

fn main() {
    let js_code = r#"
    function greet(name) {
        console.log(`Hello, ${name}`);
    }
    "#;

    let ts_code = r#"
    interface Person {
        name: string;
        age: number;
    }
    "#;

    println!("JavaScript AST:");
    parse_js(js_code);

    println!("\nTypeScript AST:");
    parse_ts(ts_code);
}
