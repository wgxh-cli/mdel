use crate::parser::{
  Parser,
  ParserExt,
  ResultData,
  pair,
};
use crate::standard::utils::Next;

#[derive(Debug, PartialEq, Eq)]
pub enum StressLevel {
  Italic,
  Bold,
  ItalicAndBold,
}

impl From<u8> for StressLevel {
  fn from(source: u8) -> Self {
    match source {
      1 => StressLevel::Italic,
      2 => StressLevel::Bold,
      3 => StressLevel::ItalicAndBold,
      _ => unreachable!()
    }
  }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Stress {
  pub level: StressLevel,
  pub content: String,
}

pub fn stress<'a>() -> impl Parser<'a, Stress> {
  pair(
    Next::new()
      .until(|result| {
        result.output == '*'
      })
      .map(|chars| chars.len()),
    Next::new()
      .until(|result| result.output != '*')
      .map(|chars| chars.into_iter().collect::<String>())
  )
  .condition(|result| result.next_input.len() == result.output.0)
  .operate(|result| {
    let level = result.output.0;
    Ok(ResultData::new(
      result.output,
      result.next_input.chars().skip(level).collect()
    ))
  })
  .map(|(level, content)| {
    Stress {
      level: StressLevel::from(level as u8),
      content,
    }
  })
}

#[test]
fn test_stress() {
  use crate::parser::{
    ResultData,
    ParserResult,
  };
  let stress_parser = stress();
  let stress_suit_1 = "*Italic*".to_string();
  let stress_suit_2 = "**Bold**".to_string();
  let stress_suit_3 = "***Bold And Italic***".to_string();
  let create_result = |level: usize, content: String| -> ParserResult<Stress> {
    Ok(ResultData::new(
      Stress {
        level: StressLevel::from(level as u8),
        content,
      },
      "".to_string()
    ))
  };
  assert_eq!(
    create_result(1, "Italic".to_string()),
    stress_parser.parse(stress_suit_1),
  );
  assert_eq!(
    create_result(2, "Bold".to_string()),
    stress_parser.parse(stress_suit_2),
  );
  assert_eq!(
    create_result(3, "Bold And Italic".to_string()),
    stress_parser.parse(stress_suit_3),
  );
}
