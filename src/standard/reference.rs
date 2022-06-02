use crate::parser::{
  Parser,
  ParserExt,
  right,
};
use crate::standard::utils::Next;

#[derive(Debug, PartialEq, Eq)]
pub struct Reference {
  pub content: String,
}

pub fn reference<'a>() -> impl Parser<'a, Reference> {
  right(
    Next::new()
    .repeat(2)
    .map(|chars| chars.into_iter().collect::<String>())
    .condition(|result| result.output == *"> "),
    Next::new()
    .until_last(|result| result.output != '\n')
    .map(|chars| chars.into_iter().collect::<String>())
  )
  .map(|content| {
    Reference {
      content
    }
  })
}

#[test]
fn test_reference() {
  use crate::parser::ResultData;

  let reference_parser = reference();
  let reference_case = "> DICK\n".to_string();
  assert_eq!(
    Ok(ResultData::new(
      Reference {
        content: "DICK".to_string(),
      },
      "".to_string()
    )),
    reference_parser.parse(reference_case)
  )
}
