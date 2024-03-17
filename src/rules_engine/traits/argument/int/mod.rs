use anyhow::Error as AnyError;
use core::ops::Range;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait IntArgument {
  fn value(&self) -> Result<i64, AnyError>;
}

#[typetag::serde(name = "Int")]
impl IntArgument for i64 {
  fn value(&self) -> Result<i64, anyhow::Error> {
    Ok(*self)
  }
}

impl PartialEq for dyn IntArgument {
  fn eq(&self, other: &Self) -> bool {
    self.value().unwrap() == other.value().unwrap()
  }
}

define_list_argument_trait_and_impl!(IntListArgument, IntArgument, "IntList");
define_map_argument_trait_and_impl!(IntMapArgument, IntArgument, "IntMap");

#[typetag::serde(tag = "type")]
pub trait IntRangeArgument {
  fn value(&self) -> Result<&Range<i64>, AnyError>;
}

#[typetag::serde(name = "IntRange")]
impl IntRangeArgument for Range<i64> {
  fn value(&self) -> Result<&Range<i64>, AnyError> {
    Ok(self)
  }
}

define_list_argument_trait_and_impl!(IntRangeListArgument, IntRangeArgument, "IntRangeList");
define_map_argument_trait_and_impl!(IntRangeMapArgument, IntRangeArgument, "IntRangeMap");
