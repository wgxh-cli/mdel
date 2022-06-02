pub mod combinators;
pub mod extensions;

pub use extensions::*;
pub use combinators::operate::*;
pub use combinators::map::*;
pub use combinators::pair::*;
pub use combinators::until::*;
pub use combinators::until_last::*;
pub use combinators::condition::*;
pub use combinators::selector::*;
pub use combinators::strategy::*;
pub use combinators::repeat::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResultData<O> {
  pub output: O,
  pub next_input: String,
}

impl<O> ResultData<O> {
  pub fn new(output: O, next_input: String) -> Self {
    ResultData {
      output,
      next_input,
    }
  }
}

pub type ParserResult<O> = Result<ResultData<O>, String>;

pub trait Parser<'a, O>
where
  O: 'a,
{
  fn parse(&self, input: String) -> ParserResult<O>;
}

pub struct BoxedParser<'a, O> {
  parser: Box<dyn Parser<'a, O> + 'a>,
}

impl<'a, O> BoxedParser<'a, O> {
  pub fn new<P: Parser<'a, O> + 'a>(parser: P) -> Self {
    BoxedParser {
      parser: Box::new(parser),
    }
  }
}

impl<'a, O> Parser<'a, O> for BoxedParser<'a, O> {
  fn parse(&self, input: String) -> ParserResult<O> {
    self.parser.parse(input)
  }
}

impl<'a, O> From<Box<dyn Parser<'a, O>>> for BoxedParser<'a, O>
{
  fn from(parser: Box<dyn Parser<'a, O>>) -> Self {
    BoxedParser {
      parser,
    }
  }
}
