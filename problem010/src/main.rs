pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {

        use std::collections::{HashMap, VecDeque};

        let parantheses = HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
        ]);

        let mut stack = VecDeque::new();

        for c in s.chars() {

            match stack.back() {

                Some(last_bracket) => {
                    match parantheses.get(last_bracket) {

                        Some(current_bracket) => {
                            if &c == current_bracket {
                                stack.pop_back();
                            }
                            else {
                                stack.push_back(c);
                            }
                        },

                        None => {},

                    }
                },

                None => {
                    stack.push_back(c);
                },

            }

        }

        stack.is_empty()

    }
}

fn main() {

    assert_eq!(
        Solution::is_valid(String::from("(()[]]{})")),
        true
    );
}
