//! Core compiler code for the Percival language.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::HashMap;

use chumsky::prelude::*;

/// A program translation unit in the Percival language.
#[derive(Clone, Debug)]
pub struct Program {
    /// Rules that make up the program.
    pub rules: Vec<Rule>,
}

/// Represents a single Horn clause.
#[derive(Clone, Debug)]
pub struct Rule {
    /// Head or implicand of the Horn clause.
    pub head: Fact,
    /// Tail or conditional assumptions of the Horn clause.
    pub clauses: Vec<Fact>,
}

/// Literal part of a Horn clause, written in terms of relations.
#[derive(Clone, Debug)]
pub struct Fact {
    /// Name of the relation being referenced.
    pub name: String,
    /// Named properties of the relation.
    pub props: HashMap<String, Value>,
}

/// A bound or unbound value assigned to part of a relation.
#[derive(Clone, Debug)]
pub enum Value {
    /// A simple identifier, which can be either bound or unbound.
    Id(String),
    // TODO: Expr(Expr),
}

/// Constructs a parser combinator for the Percival language.
fn parser() -> impl Parser<char, Program, Error = Simple<char>> {
    let id = text::ident().labelled("ident");

    let value = id.map(Value::Id).labelled("value");

    let prop = id
        .then(just(':').padded().ignore_then(value).or_not())
        .map(|(id, value)| (id.clone(), value.unwrap_or_else(|| Value::Id(id))))
        .labelled("prop");

    let fact = text::ident()
        .then(prop.padded().separated_by(just(',')).delimited_by('(', ')'))
        .map(|(name, props)| Fact {
            name,
            props: props.into_iter().collect(),
        })
        .labelled("fact");

    let rule = fact
        .then_ignore(seq(":-".chars()).padded())
        .then(fact.padded().separated_by(just(',')))
        .then_ignore(just('.'))
        .map(|(head, clauses)| Rule { head, clauses })
        .labelled("rule");

    rule.padded()
        .repeated()
        .map(|rules| Program { rules })
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
