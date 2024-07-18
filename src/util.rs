#![allow(
  unused_imports,
  dead_code,
  unused_variables,
  unused_assignments,
  unused_mut
)]

use num_traits::{PrimInt, Zero};

pub fn commulative_products<T: PrimInt + std::fmt::Debug>(
  vectors: &Vec<T>,
) -> (Option<T>, Vec<T>, Vec<T>) {
  let mut total: T = T::one();
  let mut after: Vec<T> = vec![];
  let mut before: Vec<T> = vec![];
  let mut bp: usize = 0;

  for (i, vec) in vectors.iter().enumerate().rev() {
    after.push(total);
    if let Some(result) = total.checked_mul(vec) {
      total = result;
    } else {
      bp = i + 1;
      total = T::one();
      break;
    }
  }

  for (i, vec) in vectors[..bp].iter().enumerate().rev() {
    before.push(total);
    if let Some(result) = total.checked_mul(vec) {
      total = result;
    } else {
      total = T::zero();
    }
  }

  after.reverse();
  before.reverse();

  if before != vec![] {
    for _ in 0..(after.len() - 1) {
      before.push(T::zero());
    }
  }

  match total.is_zero() {
    true => (None, before, after),
    false => (Some(total), before, after),
  }
}
