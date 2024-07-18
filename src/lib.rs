// #![allow(unused_imports, unused_variables)]
#![doc = include_str!("../readme.md")]

pub mod util;

use crate::util::commulative_products;
use index_tree::IndexTree;
use indicatif::ProgressBar;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Candidate<K: std::clone::Clone> {
  pub candidate: Vec<K>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constrain<K: std::clone::Clone> {
  pub index: usize,
  pub constrain: fn(Vec<K>) -> bool,
}

/// Search Space with extractor given an index
#[derive(Debug, Clone, PartialEq)]
pub struct SearchSpace<K: std::clone::Clone> {
  pub search_space: Vec<Candidate<K>>,
}

impl<K: std::clone::Clone> SearchSpace<K> {
  pub fn get(&self, index: &Vec<usize>) -> Vec<K> {
    index
      .iter()
      .enumerate()
      .map(|(i, j)| self.search_space[i].candidate[*j].clone())
      .collect()
  }
}

#[derive(Debug, Clone)]
pub struct TreeJump<K: std::clone::Clone> {
  pub search_space: SearchSpace<K>,
  indextree: IndexTree,
  pub constrains: Vec<Constrain<K>>,
  pub solved: Vec<Vec<K>>,
  verbatim: Option<ProgressBar>,
  timing: Option<Duration>,
}

impl<K: std::clone::Clone + std::fmt::Debug> TreeJump<K> {
  pub fn new(
    space: Vec<Vec<K>>,
    mut phis: Vec<Constrain<K>>,
    verbatim: Option<ProgressBar>,
  ) -> Self {
    // Create SearchSpace
    let search: Vec<Candidate<K>> = space
      .iter()
      .map(|cand| Candidate {
        candidate: (*cand.clone()).to_vec(),
      })
      .collect();

    // Create IndexTree
    let dims: Vec<usize> = space.iter().map(|v| v.len()).collect();
    let it: IndexTree = IndexTree::new(&dims, &vec![]);

    // Sort conditions to be index sorted
    phis.sort_by_key(|c| c.index);

    Self {
      search_space: SearchSpace {
        search_space: search,
      },
      indextree: it,
      constrains: phis,
      solved: vec![],
      verbatim: verbatim,
      timing: None,
    }
  }

  pub fn constrain_indices(&self) -> Vec<usize> {
    self.constrains.iter().map(|cons| cons.index).collect()
  }

  pub fn jump_indices(indices: &Vec<usize>) -> Vec<usize> {
    let mut jumps: Vec<usize> = vec![];
    let mut pos: usize = 0;
    let mut ind: usize = 0;
    for (j, p) in indices.into_iter().enumerate() {
      if ind != *p {
        jumps.push(j);
        ind = *p;
        pos = j;
      } else {
        jumps.push(pos);
      }
    }
    jumps
  }

  pub fn bad_indices(indices: &Vec<usize>) -> Vec<usize> {
    let mut bad: Vec<usize> = vec![];
    let mut pos: usize = 0;
    for (j, p) in indices.into_iter().enumerate() {
      if pos <= *p {
        while bad.len() <= *p {
          bad.push(j);
        }
        pos = *p;
      }
    }
    bad
  }

  pub fn search(&mut self) -> Vec<Vec<K>> {
    let length = self.constrains.len();
    let cons = self.constrain_indices();
    let bad = Self::bad_indices(&cons);
    let jump = Self::jump_indices(&cons);

    let mut pos: usize = 0;

    let (verb, pb) = self.progressbar();

    while self.indextree.check() {
      let i = self.indextree.get();
      let c = self.search_space.get(i);

      if pos >= length {
        self.solved.push(c);
        let res = self.indextree.inc();

        if verb {
          match pb {
            Some((_, ref bar)) => bar.inc(1),
            None => (),
          }
        };

        match res {
          // TODO: remove +1 when IndexTree is consistent
          Ok((_, false)) => pos = jump[pos - 1],
          Ok((j, true)) => pos = bad[j - 1],
          Err(_) => (),
        }
      } else {
        if (self.constrains[pos].constrain)(c) {
          pos += 1;
        } else {
          // TODO: remove +2 when IndexTree is consistent
          let res = self.indextree.inc_skip(self.constrains[pos].index + 2);
          if verb {
            match pb {
              Some((ref jumps, ref bar)) => bar.inc(jumps[self.constrains[pos].index]),
              None => (),
            }
          };

          match res {
            // TODO: remove +1 when IndexTree is consistent
            Ok((_, false)) => pos = jump[pos],
            Ok((j, true)) => pos = bad[j - 1],
            Err(_) => (),
          }
        }
      }
    }

    if verb {
      match pb {
        Some((_, bar)) => {
          bar.finish_and_clear();
          self.timing = Some(bar.elapsed());
        }
        None => (),
      }
    };

    self.solved.clone()
  }

  pub fn timing(&self) -> Option<Duration> {
    self.timing
  }

  fn progressbar(&self) -> (bool, Option<(Vec<u64>, ProgressBar)>) {
    match &self.verbatim {
      Some(pb) => {
        let dims: Vec<u64> = self
          .indextree
          .dimensions()
          .iter()
          .map(|x| *x as u64)
          .collect();

        let (tot, pre, after) = commulative_products(&dims);

        if let Some(n) = tot {
          pb.set_length(n);
          pb.set_message("Running:");
          let v: Vec<u64> = if pre == vec![] { after } else { pre };
          (true, Some((v, pb.clone())))
        } else {
          println!("Too big for a ProgressBar.");
          (false, None)
        }
      }
      None => (false, None),
    }
  }
}