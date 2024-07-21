use rand::Rng;
use std::collections::HashMap;

// 各状態を表す構造体
#[derive(Debug, Clone)]
pub(crate) struct State {
    pub(crate) word: String,
    probs: HashMap<String, f64>,
}

impl State {
    // 新しい状態を作成する関数
    pub(crate) fn new(word: &str, probs: HashMap<String, f64>) -> Self {
        let total_prob: f64 = probs.values().sum();
        if (total_prob - 1.0).abs() > f64::EPSILON {
            panic!(
                "The sum of probabilities must be 1.0, but it is {}",
                total_prob
            );
        }
        State {
            word: word.to_string(),
            probs,
        }
    }

    // 確率に基づいて次の状態を選択する関数
    pub(crate) fn next_state<R: Rng>(&self, rng: &mut R) -> String {
        let rand_value = rng.gen::<f64>();
        let mut cumulative_prob = 0.0;

        for (state, &prob) in &self.probs {
            cumulative_prob += prob;
            if rand_value < cumulative_prob {
                return state.clone();
            }
        }
        panic!(
            "Failed to select the next state. This should never happen if probabilities sum to 1."
        );
    }
}
