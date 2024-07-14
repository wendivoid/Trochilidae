use chumsky::prelude::*;
use lsystems::{Condition, Conditional, Module, Operator, Parameters, Rule, State, Value};

use super::Token;

pub fn parse_operator() -> impl Parser<char, Operator, Error = Simple<char>> {
    choice((
        just('+').to(Operator::Add),
        just('-').to(Operator::Sub),
        just('*').to(Operator::Mul),
        just('/').to(Operator::Div),
    ))
}

pub fn parse_conditional() -> impl Parser<char, Conditional, Error = Simple<char>> {
    choice((
        just('=').to(Conditional::EqualTo),
        just('>').to(Conditional::GreaterThan),
        just('<').to(Conditional::LessThan),
    ))
}

pub fn parse_condition() -> impl Parser<char, Condition, Error = Simple<char>> {
    text::whitespace()
        .ignore_then(parse_value())
        .then_ignore(text::whitespace())
        .then(parse_conditional())
        .then_ignore(text::whitespace())
        .then(parse_value())
        .map(|((a, cond), b)| Condition { a, cond, b })
}

pub fn parse_value() -> impl Parser<char, Value, Error = Simple<char>> {
    recursive(|expr| {
        let num = text::int(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .from_str()
            .unwrapped()
            .map(Value::Num);

        let variable = text::ident().from_str::<char>().unwrapped().map(Value::Var);

        let simple = choice((variable.clone(), num.clone()));
        let expr = simple
            .then_ignore(text::whitespace())
            .then(parse_operator())
            .then_ignore(text::whitespace())
            .then(expr)
            .map(|((a, op), b)| Value::Expr(Box::new(a), op, Box::new(b)));

        choice((expr, num, variable))
    })
}

pub fn parse_token() -> impl Parser<char, Token, Error = Simple<char>> {
    choice((
        just('F').to(Token::F),
        just('+').to(Token::Left),
        just('-').to(Token::Right),
        just('&').to(Token::Up),
        just('^').to(Token::Down),
        just('[').to(Token::Push),
        just(']').to(Token::Pop),
        just('$').to(Token::Rotate),
        just('\\').to(Token::CounterRoll),
        just('/').to(Token::Roll),
        text::ident()
            .from_str::<String>()
            .unwrapped()
            .map(|x| Token::External(x.chars().next().unwrap())),
    ))
}

pub fn parse_parameters() -> impl Parser<char, Parameters, Error = Simple<char>> {
    let sep = text::whitespace()
        .or_not()
        .ignore_then(just(','))
        .then_ignore(text::whitespace().or_not());
    parse_value()
        .separated_by(sep)
        .delimited_by(just('('), just(')'))
}

pub fn parse_module() -> impl Parser<char, Module<Token>, Error = Simple<char>> {
    parse_token()
        .then(parse_parameters().or_not())
        .map(|(token, params)| Module {
            token,
            params: params.unwrap_or_default(),
        })
}

pub fn parse_state() -> impl Parser<char, State<Token>, Error = Simple<char>> {
    parse_module()
        .repeated()
        .then(end())
        .map(|(inner, _)| State::new(inner))
}

pub fn parse_rule() -> impl Parser<char, Rule<Token>, Error = Simple<char>> {
    parse_probability()
        .then(parse_prefix())
        .then(parse_module())
        .then(parse_suffix())
        .then(
            text::whitespace()
                .or_not()
                .ignore_then(just(':'))
                .ignore_then(parse_condition())
                .or_not(),
        )
        .then_ignore(text::whitespace())
        .then_ignore(just("->"))
        .then_ignore(text::whitespace())
        .then(parse_state())
        .map(|(((((probability, prefix), m), suffix), cond), s)| {
            Rule::new(m, s)
                .with_condition(cond)
                .with_probability(probability)
                .with_previous(prefix.map(|x| x.token))
                .with_next(suffix.map(|x| x.token))
        })
}

pub fn parse_prefix() -> impl Parser<char, Option<Module<Token>>, Error = Simple<char>> {
    parse_module().then_ignore(just('<')).or_not()
}

pub fn parse_suffix() -> impl Parser<char, Option<Module<Token>>, Error = Simple<char>> {
    just('>').ignore_then(parse_module()).or_not()
}

pub fn parse_probability() -> impl Parser<char, f32, Error = Simple<char>> {
    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .from_str()
        .map(|x| x.unwrap_or(1.0));

    num.then_ignore(just(':'))
        .or_not()
        .map(|x| x.unwrap_or(1.0))
}
