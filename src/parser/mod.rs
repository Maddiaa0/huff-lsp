use huff_lexer::*;
use huff_parser::*;
use huff_utils::{
    prelude::{CompilerError, Contract, FullFileSource, ParserError},
    token::Token,
};

// TODO: add returning semantic tokens
pub fn parse(text_source: String, file_path: String) -> Result<Contract, CompilerError<'static>> {
    let file_source = FullFileSource {
        source: &text_source,
        file: None,
        spans: vec![],
    };
    let lexer = Lexer::new(file_source);

    // Grab the tokens from the lexer
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<Token>>();

    // TODO: tracing

    // Parser incantation
    let mut parser = Parser::new(tokens, Some(file_path.clone()));

    // Parse into an AST
    let parse_res = parser.parse().map_err(CompilerError::ParserError);

    // separate out the generated ast and the parsing errors
    parse_res
}
