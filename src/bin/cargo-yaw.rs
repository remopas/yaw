use yaw::analysis::Analyser;
use yaw::common::{Grammar, Rule, Symbol};
use yaw::generator::generate;

use std::env;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
enum TerminalSymbols {
    Identifier,
    Separator(&'static str),
    Eof,
}

impl yaw::common::Terminal for TerminalSymbols {
    fn eof() -> Self {
        Self::Eof
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum NonTerminalSymbols {
    Expression,
    FunctionCall,
    FunctionArgs,
    Start,
}

use NonTerminalSymbols::*;
use Symbol::*;
use TerminalSymbols::*;

fn main() {
    let grammar = Grammar::new(
        vec![
            Rule::new(Start, vec![NonTerminal(Expression)]),
            Rule::new(Expression, vec![Terminal(Identifier)]),
            Rule::new(Expression, vec![NonTerminal(FunctionCall)]),
            Rule::new(
                FunctionCall,
                vec![
                    NonTerminal(Expression),
                    Terminal(Separator("(")),
                    NonTerminal(FunctionArgs),
                    Terminal(Separator(")")),
                ],
            ),
            Rule::new(
                FunctionCall,
                vec![
                    NonTerminal(Expression),
                    Terminal(Separator("(")),
                    Terminal(Separator(")")),
                ],
            ),
            Rule::new(
                FunctionArgs,
                vec![
                    NonTerminal(FunctionArgs),
                    Terminal(Separator(",")),
                    NonTerminal(Expression),
                ],
            ),
            Rule::new(FunctionArgs, vec![NonTerminal(Expression)]),
        ],
        0,
    );

    let args = env::args();
    let file_name = args.skip(2).next().expect("No file name specified");
	let table = Analyser::table(&grammar);
	println!("{:?}", table.conflicts());
    generate(&file_name, &table).unwrap();
}
