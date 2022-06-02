use crate::parser::{
  Parser,
  ParserExt,
  ResultData,
  right,
};
use crate::standard::utils::Next;

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph {
  pub content: String,
}

pub fn paragraph<'a>() -> impl Parser<'a, Paragraph> {
  right(
    Next::new()
    .condition(|result| result.output == '\n'),
    Next::new()
    .until(|result| result.output != '\n')
    .map(|chars| {
      chars.into_iter().collect::<String>()
    })
  )
  .operate(|result| {
    Ok(ResultData::new(result.output, result.next_input.chars().skip(1).collect()))
  })
  .map(|content| {
    Paragraph {
        content
    }
  })
}

#[test]
fn test_paragraph() {
  let paragraph_parser = paragraph();
  let paragraph_case = "\nThis is Paragraph\n".to_string();
  assert_eq!(
    Ok(ResultData::new(
      Paragraph {
        content: "This is Paragraph".to_string()
      },
      "".to_string()
    )),
    paragraph_parser.parse(paragraph_case)
  )
}
