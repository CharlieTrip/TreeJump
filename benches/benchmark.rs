use criterion::{criterion_group, criterion_main, Criterion};

use tree_jump::Constrain;
use tree_jump::TreeJump;

use std::hint::black_box;

const OBJ: [u8; 10] = [10, 15, 80, 72, 1, 0, 81, 72, 30, 8];
const DIM: [usize; 6] = [10, 10, 10, 10, 10, 10];
// const DIM: [usize; 6] = [1, 1, 1, 1, 1, 1_000_000];
// const DIM: [usize; 6] = [1_000_000, 1, 1, 1, 1, 1];

pub fn get(v: Vec<usize>) -> Vec<u8> {
  v.iter().map(|&i| OBJ[i % 10]).collect()
}

pub fn f3(input: Vec<u8>, c: &Option<u8>) -> bool {
  match c {
    Some(x) => input[0] + input[1] + input[2] % 2 == *x,
    None => input[0] + input[1] + input[2] % 2 == 0,
  };
  input[2] == 0
}

pub fn f4(input: Vec<u8>, c: &Option<u8>) -> bool {
  match c {
    Some(x) => input[0] + input[2] + input[3] % 2 == *x,
    None => input[0] + input[2] + input[3] % 2 == 0,
  };
  input[3] == 0
}

pub fn f6(input: Vec<u8>, c: &Option<u8>) -> bool {
  match c {
    Some(x) => input[2] + input[4] + input[5] % 2 == *x,
    None => input[2] + input[4] + input[5] % 2 == 0,
  };
  input[5] == 8
}

pub fn cycle_for() {
  let input = vec![Some(0), Some(1), Some(0)];
  let mut sol = vec![];
  for i1 in 0..DIM[0] {
    for i2 in 0..DIM[1] {
      for i3 in 0..DIM[2] {
        let x = get(vec![i1, i2, i3, 0, 0, 0]);
        if !f3(x, &input[0]) {
          continue;
        }
        for i4 in 0..DIM[3] {
          let x = get(vec![i1, i2, i3, i4, 0, 0]);
          if !f4(x, &input[1]) {
            continue;
          }
          for i5 in 0..DIM[4] {
            for i6 in 0..DIM[5] {
              let x = get(vec![i1, i2, i3, i4, i5, i6]);
              if !f6(x.clone(), &input[2]) {
                continue;
              }
              sol.push(x);
            }
          }
        }
      }
    }
  }
}

pub fn cycle_gen_tree() {
  let space = vec![OBJ.to_vec(); 6];
  let phis = vec![
    Constrain {
      index: 2,
      constrain: f3,
    },
    Constrain {
      index: 3,
      constrain: f4,
    },
    Constrain {
      index: 5,
      constrain: f6,
    },
  ];
  let input = Some(vec![Some(0), Some(1), Some(0)]);
  let _tree = TreeJump::new(input, space, phis, None);
}

pub fn cycle_jump_tree() {
  let space = vec![OBJ.to_vec(); 6];
  let phis = vec![
    Constrain {
      index: 2,
      constrain: f3,
    },
    Constrain {
      index: 3,
      constrain: f4,
    },
    Constrain {
      index: 6,
      constrain: f6,
    },
  ];
  let input = Some(vec![Some(0), Some(1), Some(0)]);
  let mut tree = TreeJump::new(input, space, phis, None);

  let _s = tree.search();
}

fn bench_cycle_for(c: &mut Criterion) {
  c.bench_function("bench_cycle_for", |b| {
    b.iter(|| {
      black_box(cycle_for());
    });
  });
}

fn bench_gen_tree(c: &mut Criterion) {
  c.bench_function("bench_gen_tree", |b| {
    b.iter(|| {
      black_box(cycle_gen_tree());
    });
  });
}

fn bench_jump_tree(c: &mut Criterion) {
  c.bench_function("bench_jump_tree", |b| {
    b.iter(|| {
      black_box(cycle_jump_tree());
    });
  });
}

// criterion_group!(benches, bench_cycle_for, bench_gen_tree, bench_jump_tree,);
criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets =  bench_cycle_for, bench_gen_tree, bench_jump_tree
}
criterion_main!(benches);
