extern crate pest;
#[macro_use]
extern crate pest_derive;
mod instructions;

#[derive(Parser)]
#[grammar = "riscv_grammar.pest"] // relative to src
struct GrammarParser;

fn main() {
    println!("Hello, world!");
}
