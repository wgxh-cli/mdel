use crate::parser::{
  Parser,
  ParserResult,
  ResultData,
};
use super::pair::Pair;

pub struct Left<'a, A, B> {
  pair: Pair<'a, A, B>,
}

impl<'a, A, B> Parser<'a, A> for Left<'a, A, B> {
  fn parse(&self, input: String) -> ParserResult<A> {
    self.pair.parse(input).map(|result| ResultData::new(result.output.0, result.next_input))
  }
}

impl<'a, A, B> Left<'a, A, B> {
  pub fn new(first: impl Parser<'a, A> + 'a, second: impl Parser<'a, B> + 'a) -> Self {
    Left {
      pair: Pair::new(first, second),
    }
  }
}

pub struct Right<'a, A, B> {
  pair: Pair<'a, A, B>,
}

impl<'a, A, B> Parser<'a, B> for Right<'a, A, B> {
  fn parse(&self, input: String) -> ParserResult<B> {
    self.pair.parse(input).map(|result| ResultData::new(result.output.1, result.next_input))
  }
}

impl<'a, A, B> Right<'a, A, B> {
  pub fn new(first: impl Parser<'a, A> + 'a, second: impl Parser<'a, B> + 'a) -> Self {
    Right {
      pair: Pair::new(first, second),
    }
  }
}

pub fn left<'a, A, B, F, S>(first: F, second: S) -> Left<'a, A, B>
where
  F: Parser<'a, A> + 'a,
  S: Parser<'a, B> + 'a,
{
  Left::new(first, second)
}

pub fn right<'a, A, B, F, S>(first: F, second: S) -> Right<'a, A, B>
where
  F: Parser<'a, A> + 'a,
  S: Parser<'a, B> + 'a,
{
  Right::new(first, second)
}
