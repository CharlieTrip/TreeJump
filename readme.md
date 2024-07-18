# TreeJump

*tl;dr:* wrapper for `IndexTree` that uses constraints for clever jumping.

`TreeJump` is a wrapper of `IndexTree` that allows the tree's navigation using conditional jumps
	defined using a list of constraints.

**AAA**: the code doesn't provide checks if the constraints are computable or not.

```
use tree_jump::Constrain;
use tree_jump::TreeJump;

type K = u8;
fn phi(candidate: Vec<K>) -> bool {
  candidate[0] % 2 == 0
}
fn psi(candidate: Vec<K>) -> bool {
  candidate[0] + candidate[1] == 6
}
fn chi(candidate: Vec<K>) -> bool {
  candidate[1] % 2 == 0
}

let candidate1 = vec![1, 2, 3, 4];
let candidate2 = vec![1, 5, 4, 2, 3, 6];
let space = vec![candidate1, candidate2];
let constrains = vec![
  Constrain {
    index: 0,
    constrain: phi,
  },
  Constrain {
    index: 1,
    constrain: psi,
  },
  Constrain {
    index: 1,
    constrain: chi,
  },
];

let mut tree_jump = TreeJump::new(space.clone(), constrains.clone(),None);
let solved = tree_jump.search();
println!("Solved: {:?}", solved);
```


# TODO

- [ ] Consistency with skip index
- [ ] Sanitize the inputs
- [ ] Add ProgressBar
- [ ] Clean up the code
- [ ] Benchmark against standard loops
- [ ] Optimize the execution time

