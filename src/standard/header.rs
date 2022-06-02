use crate::parser::{
  Parser,
  ParserExt,
  pair,
};
use crate::standard::utils::Next;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
  pub content: String,
  pub level: u8,
}

pub fn header<'a>() -> impl Parser<'a, Header> {
  pair(
    Next::new()
      .until_last(|result| result.output != ' ' && result.output == '#')
      .map(|chars| chars.len() as u8),
    Next::new()
      .until_last(|result| result.output != '\n')
      .map(|chars| chars.into_iter().collect::<String>())
  )
  .map(|(level, content)| {
    Header {
      level,
      content,
    }
  })
}

#[test]
fn test_header() {
  use crate::parser::ResultData;
  let header_parser = header();
  let header_suit = "#### Header\n".to_string();
  assert_eq!(
    Ok(ResultData::new(
      Header {
        level: 4,
        content: "Header".to_string(),
      },
      "".to_string()
    )),
    header_parser.parse(header_suit)
  )
}
