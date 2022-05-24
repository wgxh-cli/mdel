use crate::parser::{
  Parser,
  ParserExt,
  pair,
};
use super::utils::Next;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
  pub content: String,
  pub level: u8,
}

pub fn header() -> impl Parser<Header> {
  pair(
    Next
      .until_last(|result| result.output != ' ' && result.output == '#')
      .map(|chars| chars.len() as u8),
    Next
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
