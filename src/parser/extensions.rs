use super::{
  Parser,
  ParserResult,
  ResultData,
  BoxedParser,
  Operate,
  Map,
  Until,
  UntilLast,
  Condition,
  Strategy,
  Repeat,
};

pub trait ParserExt<'c, O>: Parser<'c, O>
where
  O: 'c
{
  fn map<M, F>(self, map_fn: F) -> Map<'c, O, M>
  where
    F: Fn(O) -> M + 'c,
    Self: Sized + 'c,
  {
    Map::new(self, map_fn)
  }
  fn until<F>(self, condition: F) -> Until<'c, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'c,
    Self: Sized + 'c,
  {
    Until::new(condition, self)
  }
  fn until_last<F>(self, condition: F) -> UntilLast<'c, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'c,
    Self: Sized + 'c,
  {
    UntilLast::new(condition, self)
  }
  fn condition<F>(self, condition: F) -> Condition<'c, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'c,
    Self: Sized + 'c,
  {
    Condition::new(self, condition)
  }
  fn operate<A, F>(self, operator: F) -> Operate<'c, O, A>
  where
    A: 'c,
    F: Fn(ResultData<O>) -> ParserResult<A> + 'c,
    Self: Sized + 'c,
  {
    Operate::new(self, operator)
  }
  fn strategy<A>(self, trigger: impl Fn(ResultData<O>) -> usize + 'c, strategies: Vec<BoxedParser<'c, A>>) -> Strategy<'c, O, A>
  where
    A: 'c,
    Self: Sized + 'c,
  {
    Strategy::new(self, trigger, strategies)
  }
  fn repeat(self, count: usize) -> Repeat<'c, O>
  where
    Self: Sized + 'c
  {
    Repeat::new(self, count)
  }
}

impl<'a, P, O> ParserExt<'a, O> for P
where
  P: Parser<'a, O>,
  O: 'a
{}
