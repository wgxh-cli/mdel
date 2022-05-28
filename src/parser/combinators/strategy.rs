use crate::parser::{
  Parser,
  ParserResult,
  ResultData,
  BoxedParser,
};

pub struct Strategy<'a, A, B> {
  parser: BoxedParser<'a, A>,
  strategies: Vec<BoxedParser<'a, B>>,
  trigger: Box<dyn Fn(ResultData<A>) -> usize + 'a>,
}

impl<'a, A, B> Parser<B> for Strategy<'a, A, B> {
  fn parse(&self, input: String) -> ParserResult<B> {
    self.parser.parse(input).and_then(|result| {
      let next_input = result.next_input.clone();
      let index = (self.trigger)(result);
      if let Some(strategy) = self.strategies.into_iter().nth(index) {
        strategy.parse(next_input)
      } else {
        Err(next_input)
      }
    })
  }
}

impl<'a, A, B> Strategy<'a, A, B> {
  pub fn new<P, T, S>(parser: P, trigger: T, strategies: Vec<S>) -> Self
  where
    P: Parser<A> + 'a,
    T: Fn(ResultData<A>) -> usize + 'a,
    S: Parser<B> + 'a,
  {
    Strategy {
      parser: BoxedParser::new(parser),
      trigger: Box::new(trigger),
      strategies: strategies.into_iter().map(|strategy| BoxedParser::new(strategy)).collect(),
    }
  }
}
