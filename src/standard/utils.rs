use crate::parser::{
  Parser,
  ParserResult,
  ResultData,
};

pub struct Next<'a> {
  _marker: std::marker::PhantomData<&'a char>,
}

impl<'a> Parser<'a, char> for Next<'a> {
  fn parse(&self, input: String) -> ParserResult<char> {
    let mut chars = input.chars();
    if let Some(char) = chars.next() {
      Ok(ResultData::new(char, chars.collect()))
    } else {
      Err(input)
    }
  }
}

impl<'a> Next<'a> {
  pub fn new() -> Self {
    Next {
      _marker: std::marker::PhantomData,
    }
  }
}

impl<'a> Default for Next<'a> {
  fn default() -> Self {
    Next {
      _marker: std::marker::PhantomData,
    }
  }
}
