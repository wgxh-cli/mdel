use crate::parser::{
  Parser,
  ParserResult,
  ResultData,
  BoxedParser,
};

pub struct UntilLast<'a, A, F>
where
  F: for<'b> Fn(&'b ResultData<A>) -> bool,
{
  parser: BoxedParser<'a, A>,
  condition: Box<F>,
}

impl<'a, A, F> Parser<Vec<A>> for UntilLast<'a, A, F>
where
  F: for<'b> Fn(&'b ResultData<A>) -> bool,
{
  fn parse(&self, input: String) -> ParserResult<Vec<A>> {
    let mut next_input: String = input;
    let mut results: Vec<A> = Vec::new();
    loop {
      if let Ok(result) = self.parser.parse(next_input.clone()) {
        if (self.condition)(&result) {
          results.push(result.output);
          next_input = result.next_input;
        } else {
          break Ok(ResultData::new(results, result.next_input));
        }
      } else {
        return Err(next_input);
      }
    }
  }
}

impl<'a, A, F> UntilLast<'a, A, F>
where
  F: for<'b> Fn(&'b ResultData<A>) -> bool,
{
  pub fn new(condition: F, parser: impl Parser<A> + 'a) -> Self {
    UntilLast {
      condition: Box::new(condition),
      parser: BoxedParser::new(parser),
    }
  }
}
