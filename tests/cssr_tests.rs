use rust_cssr::{are_distributions_similar, CSSR};
use std::collections::HashSet;

#[test]
fn test_are_distributions_similar() {
    let a = vec![0.5, 0.5];
    let b = vec![0.5, 0.5];
    assert!(are_distributions_similar(&a, &b, 0.05));

    let a = vec![0.1, 0.9];
    let b = vec![0.9, 0.1];
    assert!(!are_distributions_similar(&a, &b, 0.05));
}

#[test]
fn test_cssr_simple_sequence() {
    // A simple sequence with a clear pattern: 0, 1, 0, 1, ...
    let data = vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    let alphabet: HashSet<u32> = data.iter().cloned().collect();
    let mut cssr = CSSR::new(alphabet);

    // Run CSSR with a max history of 1 and a high alpha to force splits.
    cssr.run(&data, 1, 0.01);

    // There should be two causal states:
    // 1. After a 0, the next symbol is always 1.
    // 2. After a 1, the next symbol is always 0.
    assert_eq!(cssr.states.len(), 2);

    let mut state_defs = Vec::new();
    for state in &cssr.states {
        let mut histories: Vec<_> = state.histories.iter().collect();
        histories.sort();
        let mut dist: Vec<_> = state.next_symbol_dist.iter().collect();
        dist.sort_by(|a, b| a.0.cmp(b.0));
        state_defs.push((histories, dist));
    }
    state_defs.sort_by(|a, b| a.0.cmp(&b.0));

    // After a 0, we expect a 1.
    assert_eq!(state_defs[0].0, vec![&vec![0]]);
    assert_eq!(*state_defs[0].1[0].0, 0);
    assert_eq!(*state_defs[0].1[0].1, 0.0);
    assert_eq!(*state_defs[0].1[1].0, 1);
    assert_eq!(*state_defs[0].1[1].1, 1.0);

    // After a 1, we expect a 0.
    assert_eq!(state_defs[1].0, vec![&vec![1]]);
    assert_eq!(*state_defs[1].1[0].0, 0);
    assert_eq!(*state_defs[1].1[0].1, 1.0);
    assert_eq!(*state_defs[1].1[1].0, 1);
    assert_eq!(*state_defs[1].1[1].1, 0.0);
}
