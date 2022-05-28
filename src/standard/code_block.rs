use crate::parser::{
  Parser,
  ParserExt,
  ResultData,
  pair,
};
use crate::standard::utils::Next;

pub enum CodeBlockLevel {
  Inline,
  Block(String),
}

pub struct CodeBlock {
  pub level: CodeBlockLevel,
  pub content: String,
}

pub fn code_block() -> impl Parser<CodeBlock> {
  Next
    .until(|result| result.output == '`')
    .map(|chars| chars.len())
    .strategy(
      |result| {
        match result.output {
          1 => 1,
          3 => 2,
          _ => {
            unreachable!()
          },
        }
      },
      vec![
        // Inline code block
        Next
          .until(|result| result.output != '`')
          .map(|chars| chars.into_iter().collect::<String>())
          .condition(|result| result.next_input.starts_with("*"))
          .operate(|result| {
            Ok(ResultData::new(result.output, result.next_input.chars().skip(1).collect()))
          })
          .map(|content| {
            CodeBlock {
              level: CodeBlockLevel::Inline,
              content,
            }
          }),
        // Code block
        pair(
          Next
            .until_last(|result| result.output != '\n')
            .map(|chars| chars.into_iter().collect::<String>()),
          Next
            .until(|result| result.output != '`')
            .map(|chars| chars.into_iter().collect::<String>())
        )
        .condition(|result| result.next_input.starts_with(&"```".to_string()))
        .operate(|result| {
          Ok(ResultData::new(result.output, result.next_input.chars().skip(3).collect()))
        })
        .map(|(language, content)| {
          CodeBlock {
            level: CodeBlockLevel::Block(language),
            content,
          }
        })
      ]
    )
}
