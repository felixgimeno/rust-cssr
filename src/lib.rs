//! Causal-State Splitting Reconstruction (CSSR) algorithm implementation.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use statrs::distribution::{ChiSquared, ContinuousCDF};

/// Represents a history of symbols.
pub type History = Vec<u32>;

/// Represents a causal state in the CSSR algorithm.
///
/// Each causal state contains a set of histories that are considered equivalent
/// for the purpose of prediction. It also stores the probability distribution
/// of the next symbol given the histories in this state.
#[derive(Debug, Clone)]
pub struct CausalState {
    /// The set of histories that belong to this state.
    pub histories: HashSet<History>,
    /// The probability distribution of the next symbol.
    pub next_symbol_dist: HashMap<u32, f32>,
}

impl CausalState {
    /// Creates a new, empty causal state.
    pub fn new() -> Self {
        CausalState {
            histories: HashSet::new(),
            next_symbol_dist: HashMap::new(),
        }
    }
}
impl PartialEq for CausalState {
    fn eq(&self, other: &Self) -> bool {
        self.histories == other.histories
    }
}

impl Eq for CausalState {}

impl Hash for CausalState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut histories: Vec<_> = self.histories.iter().collect();
        histories.sort();
        for history in histories {
            history.hash(state);
        }
    }
}

/// The main CSSR algorithm struct.
///
/// This struct holds the state of the algorithm, including the set of causal
/// states and the alphabet of the input data.
pub struct CSSR {
    /// The set of causal states.
    pub states: HashSet<CausalState>,
    /// The alphabet of the input data.
    pub alphabet: HashSet<u32>,
}

impl CSSR {
    /// Creates a new CSSR instance.
    pub fn new(alphabet: HashSet<u32>) -> Self {
        CSSR {
            states: HashSet::new(),
            alphabet,
        }
    }

    /// Runs the CSSR algorithm on the given data.
    pub fn run(&mut self, data: &[u32], max_history: usize, alpha: f32) {
        // 1. Initialize with a single state containing the null history.
        let mut initial_state = CausalState::new();
        initial_state.histories.insert(vec![]); // The empty history
        initial_state.next_symbol_dist =
            compute_next_symbol_dist(&initial_state.histories, data, &self.alphabet);
        self.states.insert(initial_state);

        // Main loop for increasing history length L
        for l in 0..max_history {
            let mut new_states: HashSet<CausalState> = HashSet::new();

            for state in &self.states {
                // For each symbol in the alphabet, create a potential new state by extending the histories
                let mut potential_new_states: Vec<CausalState> = Vec::new();

                for &symbol in &self.alphabet {
                    let mut new_histories = HashSet::new();
                    for history in &state.histories {
                        // All histories in a state should have the same length `l`
                        if history.len() == l {
                            let mut new_history = history.clone();
                            new_history.push(symbol);
                            new_histories.insert(new_history);
                        }
                    }

                    if !new_histories.is_empty() {
                        let mut new_state = CausalState::new();
                        new_state.histories = new_histories;
                        new_state.next_symbol_dist =
                            compute_next_symbol_dist(&new_state.histories, data, &self.alphabet);

                        // Only add if there is data to support this history
                        if new_state.next_symbol_dist.values().any(|&p| p > 0.0) {
                            potential_new_states.push(new_state);
                        }
                    }
                }

                if potential_new_states.is_empty() {
                    new_states.insert(state.clone());
                    continue;
                }

                let mut merged_states: Vec<CausalState> = Vec::new();

                'outer: while let Some(mut current_state) = potential_new_states.pop() {
                    for merged_state in &mut merged_states {
                        let mut alphabet_vec: Vec<_> = self.alphabet.iter().collect();
                        alphabet_vec.sort();

                        let a_dist: Vec<f64> = alphabet_vec.iter().map(|s| *current_state.next_symbol_dist.get(s).unwrap_or(&0.0) as f64).collect();
                        let b_dist: Vec<f64> = alphabet_vec.iter().map(|s| *merged_state.next_symbol_dist.get(s).unwrap_or(&0.0) as f64).collect();

                        if are_distributions_similar(&a_dist, &b_dist, alpha as f64) {
                            merged_state.histories.extend(current_state.histories.drain());
                            merged_state.next_symbol_dist = compute_next_symbol_dist(&merged_state.histories, data, &self.alphabet);
                            continue 'outer;
                        }
                    }
                    merged_states.push(current_state);
                }

                for s in merged_states {
                    new_states.insert(s);
                }
            }

            self.states = new_states;
        }
    }
}

/// Computes the probability distribution of the next symbol given a set of histories.
fn compute_next_symbol_dist(
    histories: &HashSet<History>,
    data: &[u32],
    alphabet: &HashSet<u32>,
) -> HashMap<u32, f32> {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    let mut total_count = 0;

    if histories.is_empty() {
        return alphabet.iter().map(|&s| (s, 0.0)).collect();
    }

    let history_len = histories.iter().next().unwrap().len();

    if history_len == 0 {
        for symbol in data {
            *counts.entry(*symbol).or_insert(0) += 1;
            total_count += 1;
        }
    } else {
        let history_set: HashSet<&[u32]> = histories.iter().map(|h| h.as_slice()).collect();
        if data.len() > history_len {
            for i in 0..=(data.len() - history_len - 1) {
                let history_slice = &data[i..i + history_len];
                if history_set.contains(history_slice) {
                    let next_symbol = data[i + history_len];
                    *counts.entry(next_symbol).or_insert(0) += 1;
                    total_count += 1;
                }
            }
        }
    }

    let mut dist = HashMap::new();
    if total_count > 0 {
        for (&symbol, &count) in &counts {
            dist.insert(symbol, count as f32 / total_count as f32);
        }
    }

    for &symbol in alphabet {
        dist.entry(symbol).or_insert(0.0);
    }

    dist
}


/// Performs a chi-square statistical test to determine if two distributions are similar.
///
/// This function compares two probability distributions to determine if they are
/// statistically similar based on a given significance level.
///
/// # Arguments
///
/// * `a` - The first probability distribution.
/// * `b` - The second probability distribution.
/// * `alpha` - The significance level.
///
/// # Returns
///
/// A boolean indicating whether the two distributions are statistically similar.
pub fn are_distributions_similar(a: &[f64], b: &[f64], alpha: f64) -> bool {
    if a.len() != b.len() {
        panic!("Input slices must have the same length.");
    }

    let mut chi_square_stat = 0.0;
    let mut degrees_of_freedom = 0.0;

    for (p_a, p_b) in a.iter().zip(b.iter()) {
        if *p_b > 0.0 {
            let diff = p_a - p_b;
            chi_square_stat += (diff * diff) / p_b;
            degrees_of_freedom += 1.0;
        } else if *p_a > 0.0 {
            // p_b is 0, but p_a is not. They are very different.
            // Chi-square statistic would be infinite.
            return false;
        }
    }

    // If there are no categories with non-zero probability in `b`, the distributions are the same.
    if degrees_of_freedom == 0.0 {
        return true;
    }

    // The degrees of freedom for a chi-square test of homogeneity are (rows - 1) * (cols - 1).
    // In our case, rows=2 (the two distributions) and cols is the number of categories.
    // However, since we are comparing two distributions over the same categories, it simplifies.
    // If we have k categories, we have k-1 degrees of freedom.
    let dof = degrees_of_freedom - 1.0;
    if dof <= 0.0 {
        return true;
    }

    let chi_square_dist = ChiSquared::new(dof).unwrap();

    // The p-value is the probability of observing a chi-square statistic as extreme or more extreme
    // than the one calculated, given that the null hypothesis (the distributions are the same) is true.
    let p_value = 1.0 - chi_square_dist.cdf(chi_square_stat);

    // If the p-value is greater than the significance level `alpha`, we fail to reject the null hypothesis,
    // meaning the distributions are considered similar.
    p_value > alpha
}
