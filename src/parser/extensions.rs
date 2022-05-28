use super::{
  Parser,
  ParserResult,
  ResultData,
  Operate,
  Map,
  Until,
  UntilLast,
  Condition,
  Strategy,
};

pub trait ParserExt<O>: Parser<O> {
  fn map<'a, M, F>(self, map_fn: F) -> Map<'a, O, M>
  where
    F: Fn(O) -> M + 'a,
    Self: Sized + 'a,
  {
    Map::new(self, map_fn)
  }
  fn until<'a, F>(self, condition: F) -> Until<'a, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'a,
    Self: Sized + 'a,
  {
    Until::new(condition, self)
  }
  fn until_last<'a, F>(self, condition: F) -> UntilLast<'a, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'a,
    Self: Sized + 'a,
  {
    UntilLast::new(condition, self)
  }
  fn condition<'a, F>(self, condition: F) -> Condition<'a, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'a,
    Self: Sized + 'a,
  {
    Condition::new(self, condition)
  }
  fn operate<'a, A, F>(self, operator: F) -> Operate<'a, O, A>
  where
    A: 'a,
    F: Fn(ResultData<O>) -> ParserResult<A> + 'a,
    Self: Sized + 'a,
  {
    Operate::new(self, operator)
  }
  fn strategy<'a, A, P, T>(self, trigger: T, strategies: Vec<P>) -> Strategy<'a, O, A>
  where
    A: 'a,
    P: Parser<A> + 'a,
    T: Fn(ResultData<O>) -> usize + 'a,
    Self: Sized + 'a,
  {
    Strategy::new(self, trigger, strategies)
  }
}

impl<P, O> ParserExt<O> for P
where
  P: Parser<O>
{}
