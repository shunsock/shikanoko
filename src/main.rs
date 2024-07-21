mod markov_chain;
mod state;

use markov_chain::MarkovChain;
use state::State;
use std::time::Instant;

fn main() {
    let markov_chain = shikanoko_chain();

    let start = Instant::now();

    loop {
        let text: String = markov_chain.generate_text("s1");
        println!("{}", text);
        if text == "しかのこのこのここしたんたん" {
            break;
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed in generate_text() is: {:?}", duration);
}

fn shikanoko_chain() -> MarkovChain {
    let mut markov_chain = MarkovChain::new();

    markov_chain.add_state(
        "s1",
        State::new(
            "し",
            [("s2".to_string(), 0.5), ("s5".to_string(), 0.5)]
                .iter()
                .cloned()
                .collect(),
        ),
    );
    markov_chain.add_state(
        "s2",
        State::new("か", [("s3".to_string(), 1.0)].iter().cloned().collect()),
    );
    markov_chain.add_state(
        "s3",
        State::new("の", [("s4".to_string(), 1.0)].iter().cloned().collect()),
    );
    markov_chain.add_state(
        "s4",
        State::new(
            "こ",
            [
                ("s3".to_string(), 0.5),
                ("s4".to_string(), 0.25),
                ("s1".to_string(), 0.25),
            ]
            .iter()
            .cloned()
            .collect(),
        ),
    );
    markov_chain.add_state(
        "s5",
        State::new("た", [("s6".to_string(), 1.0)].iter().cloned().collect()),
    );
    markov_chain.add_state(
        "s6",
        State::new(
            "ん",
            [("s7".to_string(), 0.5), ("s6".to_string(), 0.5)]
                .iter()
                .cloned()
                .collect(),
        ),
    );
    markov_chain.add_state(
        "s7",
        State::new(
            ".",
            [("s1".to_string(), 0.5), ("s7".to_string(), 0.5)]
                .iter()
                .cloned()
                .collect(),
        ),
    );

    markov_chain
}
