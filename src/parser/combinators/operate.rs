use crate::parser::{
  Parser,
  ParserResult,
  BoxedParser,
  ResultData,
};

pub struct Operate<'a, A, B>
where
  A: 'a,
  B: 'a,
{
  parser: BoxedParser<'a, A>,
  operator: Box<dyn Fn(ResultData<A>) -> ParserResult<B> + 'a>,
}

impl<'a, A, B> Parser<'a, B> for Operate<'a, A, B>
where
  A: 'a,
  B: 'a,
{
  fn parse(&self, input: String) -> ParserResult<B> {
    self.parser.parse(input).and_then(|result| {
      (self.operator)(result)
    })
  }
}

impl<'a, A, B> Operate<'a, A, B>
where
  A: 'a,
  B: 'a,
{
  pub fn new<P, F>(parser: P, operator: F) -> Self
  where
    P: Parser<'a, A> + 'a,
    F: Fn(ResultData<A>) -> ParserResult<B> + 'a,
  {
    Operate {
      parser: BoxedParser::new(parser),
      operator: Box::new(operator),
    }
  }
}
