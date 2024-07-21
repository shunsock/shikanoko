use crate::state::State;
use std::collections::HashMap;

pub(crate) struct MarkovChain {
    states: HashMap<String, State>,
}

impl MarkovChain {
    // 新しいMarkov Chainを作成する関数
    pub(crate) fn new() -> Self {
        MarkovChain {
            states: HashMap::new(),
        }
    }

    // 状態を追加する関数
    pub(crate) fn add_state(&mut self, name: &str, state: State) {
        self.states.insert(name.to_string(), state);
    }

    // テキストを生成する関数
    pub(crate) fn generate_text(&self, start_state: &str) -> String {
        let mut current_state_name = start_state.to_string();
        let mut rng = rand::thread_rng();
        let mut stack: Vec<String> = Vec::new();

        while let Some(state) = self.states.get(&current_state_name) {
            stack.push(state.word.clone());

            if stack.len() == 14 || state.word == "." {
                break;
            }

            current_state_name = state.next_state(&mut rng);
        }

        stack.join("")
    }
}
