#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_mod!(#[allow(clippy::all)] pub kal_grammar);
mod interpreter;

#[cfg(test)]
mod tests;

mod kal_ref;

fn main() {
    let ast = kal_grammar::BlockInnerParser::new()
        .parse(include_str!("../examples/boolean_precedence.kal"))
        .unwrap_or_else(|err| panic!("Failed to parse file, {:?}", err));

    // Never deallocate the AST, because it has to live longer than garbage collected objects.
    let ast = Box::leak(Box::new(ast));

    let result = interpreter::eval(ast);

    println!("{:#?}", result);
}
