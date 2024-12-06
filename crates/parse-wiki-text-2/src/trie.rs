// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use crate::case_folding_simple::CASE_FOLDING_SIMPLE;

struct Character<T> {
    character: u8,
    next_state: State<T>,
}

#[derive(Clone, Copy)]
enum State<T> {
    Continue(u32),
    Final(T),
}

pub struct Trie<T> {
    states: Vec<Vec<Character<T>>>,
}

impl<T: Copy> Trie<T> {
    pub fn add_case_sensitive_term(&mut self, term: &str, payload: T) -> bool {
        self.add_term_internal(term, payload, false)
    }

    fn add_folded_characters(&mut self, character: char, initial_state: u32, next_state: State<T>) {
        if let Some(folded_characters) = simple_fold(character) {
            for character in folded_characters {
                let mut last_state = initial_state;
                let mut character_buffer = [0; 4];
                let character_bytes = character.encode_utf8(&mut character_buffer).as_bytes();
                let mut byte_iterator = character_bytes.iter().cloned();
                let mut byte_item = byte_iterator.next();
                'b: while let Some(byte) = byte_item {
                    for item in &self.states[last_state as usize] {
                        if item.character == byte {
                            match item.next_state {
                                State::Continue(next_state) => last_state = next_state,
                                State::Final(_) => unreachable!(),
                            }
                            byte_item = byte_iterator.next();
                            continue 'b;
                        }
                    }
                    byte_item = byte_iterator.next();
                    if byte_item.is_none() {
                        self.states[last_state as usize].push(Character {
                            character: byte,
                            next_state,
                        });
                        break;
                    }
                    let intermediate_state = self.states.len() as _;
                    self.states[last_state as usize].push(Character {
                        character: byte,
                        next_state: State::Continue(intermediate_state),
                    });
                    last_state = intermediate_state;
                    self.states.push(vec![]);
                }
            }
        }
    }

    pub fn add_term(&mut self, term: impl AsRef<str>, payload: T) -> bool {
        self.add_term_internal(term, payload, true)
    }

    fn add_term_internal(&mut self, term: impl AsRef<str>, payload: T, case_folded: bool) -> bool {
        let mut last_state = 0;
        let mut character_iterator = term.as_ref().chars();
        let mut character_item = character_iterator.next();
        while let Some(character) = character_item {
            let mut character_buffer = [0; 4];
            let character_bytes = character.encode_utf8(&mut character_buffer).as_bytes();
            character_item = character_iterator.next();
            let mut byte_iterator = character_bytes.iter().cloned();
            let mut byte_item = byte_iterator.next();
            let state_before_character = last_state;
            'a: while let Some(byte) = byte_item {
                for item in &self.states[last_state as usize] {
                    if item.character == byte {
                        match item.next_state {
                            State::Continue(next_state) => last_state = next_state,
                            State::Final(_) => return false,
                        }
                        byte_item = byte_iterator.next();
                        continue 'a;
                    }
                }
                byte_item = byte_iterator.next();
                if byte_item.is_none() {
                    if character_item.is_none() {
                        self.states[last_state as usize].push(Character {
                            character: byte,
                            next_state: State::Final(payload),
                        });
                        if case_folded {
                            self.add_folded_characters(
                                character,
                                state_before_character,
                                State::Final(payload),
                            );
                        }
                        return true;
                    }
                    let next_state = self.states.len() as _;
                    self.states[last_state as usize].push(Character {
                        character: byte,
                        next_state: State::Continue(next_state),
                    });
                    self.states.push(vec![]);
                    if case_folded {
                        self.add_folded_characters(
                            character,
                            state_before_character,
                            State::Continue(next_state),
                        );
                    }
                    last_state = next_state;
                    break;
                }
                let next_state = self.states.len() as _;
                self.states[last_state as usize].push(Character {
                    character: byte,
                    next_state: State::Continue(next_state),
                });
                last_state = next_state;
                self.states.push(vec![]);
            }
        }
        false
    }

    pub fn find(&self, text: &str) -> Result<(usize, T), usize> {
        let mut state = 0;
        'outer: for (position, character1) in text.as_bytes().iter().cloned().enumerate() {
            for character2 in &self.states[state as usize] {
                if character1 == character2.character {
                    match character2.next_state {
                        State::Continue(next_state) => {
                            state = next_state;
                            continue 'outer;
                        }
                        State::Final(payload) => return Ok((position + 1, payload)),
                    }
                }
            }
            return Err(position);
        }
        Err(0)
    }

    pub fn new() -> Self {
        Trie {
            states: vec![vec![]],
        }
    }
}

fn simple_fold(character: char) -> Option<&'static [char]> {
    match CASE_FOLDING_SIMPLE.binary_search_by_key(&character, |&(character, _)| character) {
        Err(_) => None,
        Ok(index) => Some(CASE_FOLDING_SIMPLE[index].1),
    }
}
