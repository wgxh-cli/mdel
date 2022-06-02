use crate::parser::{
  Parser,
  ParserExt,
  ResultData,
  BoxedParser,
  pair,
};
use crate::standard::utils::Next;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeBlockLevel {
  Inline,
  Block(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeBlock {
  pub level: CodeBlockLevel,
  pub content: String,
}

pub fn code_block<'a>() -> impl Parser<'a, CodeBlock> {
  Next::new()
    .until(|result| result.output == '`')
    .map(|chars| chars.into_iter().collect::<String>())
    .strategy(
      |result| {
        match result.output.len() {
          1 => 0,
          3 => 1,
          _ => unreachable!(),
        }
      },
      Vec::from([
        BoxedParser::new(
          Next::new()
          .until_last(|result| result.output != '`')
          .map(|content| {
            CodeBlock {
              level: CodeBlockLevel::Inline,
              content: content.into_iter().collect(),
            }
          })
        ),
        BoxedParser::new(
          pair(
            Next::default()
            .until_last(|result| result.output != '\n')
            .map(|chars| chars.into_iter().collect::<String>()),
            Next::default()
            .until(|result| result.output != '`')
            .map(|chars| chars.into_iter().collect::<String>())
          )
          .condition(|result| result.next_input.starts_with(&"```"))
          .operate(|result| {
            Ok(ResultData::new(result.output, result.next_input.chars().skip(3).collect()))
          })
          .map(|(language, content)| {
            CodeBlock {
              level: CodeBlockLevel::Block(language),
              content,
            }
          })
        )
      ])
    )
}

#[test]
fn test_code_block() {
  let code_block_parser = code_block();
  let inline_case = "`DICK`".to_string();
  let block_case = "```rust\nDICK\n```".to_string();
  assert_eq!(
    Ok(ResultData::new(
      CodeBlock {
        level: CodeBlockLevel::Inline,
        content: "DICK".to_string(),
      },
      "".to_string()
    )),
    code_block_parser.parse(inline_case)
  );
  assert_eq!(
    Ok(ResultData::new(
      CodeBlock {
        level: CodeBlockLevel::Block("rust".to_string()),
        content: "DICK\n".to_string(),
      },
      "".to_string()
    )),
    code_block_parser.parse(block_case)
  )
}
