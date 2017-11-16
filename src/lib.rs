/*!

This crate makes it fast and simple to implement Levenshtein Automata.



# Example

```rust
extern crate levenshtein_automaton;

use levenshtein_automaton::{LevenshteinAutomatonBuilder, Distance};

fn main() {

    let lev_automaton_builder = LevenshteinAutomatonBuilder::new(2, true);

    // We can now build an entire dfa.
    let dfa = lev_automaton_builder.build_dfa("saucisson sec");

    let mut state = dfa.initial_state();
    for &b in "saucissonsec".as_bytes() {
        state = dfa.transition(state, b);
    }
    assert_eq!(dfa.distance(state), Distance::Exact(1));
}
```

The implementation is based on the following paper
**Fast String Correction with Levenshtein-Automata (2002)** by by Klaus Schulz and Stoyan Mihov.
I also tried to explain it in the following [blog post](https://fulmicoton.com/posts/levenshtein/).


# Bench


The time taken by the construction a Levenshtein DFA
strongly depends on the max distance it can measure and the length of the input string.

Here is the time spent to build a Levenshtein DFA for the string "Levenshtein"


```ignore
test bench::bench_build_dfa_distance1_no_transpose   ... bench:     115,580 ns/iter (+/- 38,866)
test bench::bench_build_dfa_distance1_with_transpose ... bench:     112,540 ns/iter (+/- 29,627)
test bench::bench_build_dfa_distance2_no_transpose   ... bench:     308,064 ns/iter (+/- 141,326)
test bench::bench_build_dfa_distance2_with_transpose ... bench:     301,771 ns/iter (+/- 117,123)
test bench::bench_build_dfa_distance3_no_transpose   ... bench:   1,459,171 ns/iter (+/- 267,558)
test bench::bench_build_dfa_distance3_with_transpose ... bench:   2,123,324 ns/iter (+/- 1,559,343)
test bench::bench_build_dfa_distance4_no_transpose   ... bench:   6,114,126 ns/iter (+/- 654,160)
test bench::bench_build_dfa_distance4_with_transpose ... bench:  10,313,151 ns/iter (+/- 1,283,931)
```

!*/

#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod bench;


mod parametric_dfa;
mod alphabet;
mod levenshtein_nfa;
mod dfa;
mod index;

use self::index::Index;
use self::levenshtein_nfa::LevenshteinNFA;
use self::parametric_dfa::ParametricDFA;
pub use self::dfa::DFA;
pub use self::levenshtein_nfa::Distance;

/// Builder for Levenshtein Automata.
///
/// It wraps a precomputed datastructure that allows to
/// produce small (but not minimal) DFA.
pub struct LevenshteinAutomatonBuilder {
    parametric_dfa: ParametricDFA
}

impl LevenshteinAutomatonBuilder {

    /// Creates a Levenshtein automaton builder.
    /// The builder
    ///
    /// * `max_distance` - maximum distance considered by the automaton.
    /// * `transposition_cost_one` - assign a distance of 1 for transposition
    ///
    /// Building this automaton builder is computationally intensive.
    /// While it takes only a few milliseconds for `d=2`, it grows exponentially with
    /// `d`. It is only reasonable to `d <= 5`.
    pub fn new(max_distance: u8, transposition_cost_one: bool) -> LevenshteinAutomatonBuilder {
        let levenshtein_nfa = LevenshteinNFA::levenshtein(max_distance, transposition_cost_one);
        let parametric_dfa = ParametricDFA::from_nfa(&levenshtein_nfa);
        LevenshteinAutomatonBuilder {
            parametric_dfa: parametric_dfa
        }
    }

    /// Builds a Finite Determinstic Automaton to compute
    /// that computes the levenshtein distance to a given `query`.
    ///
    /// There is no guarantee that the resulting DFA is minimal
    /// but its number of states is guaranteed to be smaller
    /// than `C * (query.len() + 1)` in which C is a constant that depends
    /// on the distance as well as whether transposition are supported
    /// or not.
    ///
    /// For instance for `d=2` and with transposition, `C=68`.
    pub fn build_dfa(&self, query: &str) -> DFA {
        self.parametric_dfa.build_dfa(query)
    }
}
