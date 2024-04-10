#![no_std]
#![no_main]

use nexus_rt::{println, Write};
use sqlparser::{dialect::GenericDialect, parser::Parser};

#[nexus_rt::main]
fn main() {
    let sql = "SELECT a, b FROM table_1";

    let dialect = GenericDialect {};

    // Debugged in it, and found that it's failed to parse SQL tokens.
    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {:?}", ast);
}
