// #![allow(dead_code, unused_variables, unused_imports)]
#[cfg(test)]
extern crate tree_jump;

#[cfg(test)]
mod tests {
  use super::*;
  use rand::Rng;

  use tree_jump::util::commulative_products;
  use tree_jump::Candidate;
  use tree_jump::Constrain;
  use tree_jump::SearchSpace;
  use tree_jump::TreeJump;

  #[test]
  fn init_structs() {
    let mut rng = rand::thread_rng();
    let limit: usize = 10;
    let limitbis: usize = 10;
    let random_values: Vec<Vec<u8>> = (0..limit)
      .into_iter()
      .map(|_| (0..limitbis).into_iter().map(|_| rng.gen()).collect())
      .collect();

    let mut cand: Vec<Candidate<u8>> = vec![];
    for i in 0..limit {
      cand.push(Candidate::<u8> {
        candidate: random_values[i].clone(),
      });
      assert_eq!(cand[i].candidate, random_values[i]);
    }

    let space = SearchSpace {
      search_space: cand.clone(),
    };

    for i in 0..limitbis {
      let value = space.get(&vec![i]);

      for j in value.iter() {
        assert_eq!(*j, random_values[0][i]);
      }
    }

    let phis: Vec<Constrain<K>> = (0..3)
      .into_iter()
      .map(|i| Constrain::<u8> {
        index: 2 - i,
        constrain: phi,
      })
      .collect();

    let tree = TreeJump::<K>::new(random_values, phis, None);

    for i in 0..3 {
      assert_eq!(tree.constrains[i].index, i)
    }
  }

  #[test]
  fn jump_vector() {
    let jump = TreeJump::<u8>::jump_indices(&vec![1, 2, 2, 4, 5, 5]);
    assert_eq!(jump, vec![0, 1, 1, 3, 4, 4]);
    let jump = TreeJump::<u8>::jump_indices(&vec![0, 0, 4, 4, 5, 5]);
    assert_eq!(jump, vec![0, 0, 2, 2, 4, 4]);
    let jump = TreeJump::<u8>::jump_indices(&vec![3, 3, 4, 5, 6, 7]);
    assert_eq!(jump, vec![0, 0, 2, 3, 4, 5]);
  }

  #[test]
  fn bad_vector() {
    let bad = TreeJump::<u8>::bad_indices(&vec![1, 2, 2, 4, 5, 5]);
    assert_eq!(bad, vec![0, 0, 1, 3, 3, 4]);
    let bad = TreeJump::<u8>::bad_indices(&vec![0, 0, 4, 4, 5, 5]);
    assert_eq!(bad, vec![0, 2, 2, 2, 2, 4]);
    let bad = TreeJump::<u8>::bad_indices(&vec![3, 3, 4, 5, 6, 7]);
    assert_eq!(bad, vec![0, 0, 0, 0, 2, 3, 4, 5]);
  }

  #[test]
  fn search() {
    let candidate1 = vec![1, 2, 6, 3, 4];
    let candidate2 = vec![4, 5, 2, 3, 6];
    let space = vec![candidate1, candidate2];
    let constrains = vec![
      Constrain {
        index: 0,
        constrain: phiev,
      },
      Constrain {
        index: 1,
        constrain: phieq,
      },
    ];

    let mut tree_jump = TreeJump::new(space.clone(), constrains.clone(), None);
    let solved = tree_jump.search();

    assert_eq!(solved, vec![vec![2, 2], vec![6, 6], vec![4, 4]]);

    let constrains = vec![
      Constrain {
        index: 0,
        constrain: phiev,
      },
      Constrain {
        index: 1,
        constrain: phieq,
      },
      Constrain {
        index: 1,
        constrain: phisix,
      },
    ];

    let mut tree_jump = TreeJump::new(space.clone(), constrains.clone(), None);
    let solved = tree_jump.search();

    assert_eq!(solved, vec![vec![6, 6]]);
  }

  #[test]
  fn vectors() {
    let vectors: Vec<u8> = vec![2, 3, 3, 8];
    let (a, _, _) = commulative_products(&vectors);
    assert_eq!(a, Some(144));
    let vectors: Vec<u8> = vec![2, 8, 8, 8];
    let (a, _, _) = commulative_products(&vectors);
    assert_eq!(a, Some(16));
  }

  type K = u8;
  type X = u8;
  type C = u8;

  fn f(x: X, k: Vec<K>, c: C) -> bool {
    x ^ k[0] ^ k[1] ^ c == 0
  }

  fn phieq(x: Vec<u8>) -> bool {
    x[0] ^ x[1] == 0
  }

  fn phiev(x: Vec<u8>) -> bool {
    x[0] % 2 == 0
  }
  fn phisix(x: Vec<u8>) -> bool {
    x[1] % 3 == 0
  }

  fn phi(k: Vec<K>) -> bool {
    let x: X = 5;
    let c: C = 3;
    f(x, k, c)
  }
}
