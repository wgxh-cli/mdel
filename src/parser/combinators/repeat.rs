use crate::parser::{
  Parser,
  ParserResult,
  BoxedParser,
  ResultData,
};

pub struct Repeat<'a, A> {
  parser: BoxedParser<'a, A>,
  count: usize,
}

impl<'a, A> Parser<'a, Vec<A>> for Repeat<'a, A> {
  fn parse(&self, input: String) -> ParserResult<Vec<A>> {
    let mut next_input = input;
    let mut results: Vec<A> = Vec::new();
    for _ in 0..self.count {
      if let Ok(result) = self.parser.parse(next_input.clone()) {
        results.push(result.output);
        next_input = result.next_input.clone();
      } else {
        return Err(next_input);
      }
    }
    Ok(ResultData::new(results, next_input))
  }
}

impl<'a, A> Repeat<'a, A> {
  pub fn new(parser: impl Parser<'a, A> + 'a, count: usize) -> Self {
    Repeat {
      parser: BoxedParser::new(parser),
      count,
    }
  }
}
