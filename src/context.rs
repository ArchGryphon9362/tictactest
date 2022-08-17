use std::vec::Vec;
use std::option::Option;
use crate::player::Player;

pub struct Context {
    frame_count: i32,
    players: Vec<Option<Player>>
}

impl Context {
    pub fn new() -> Self {
        let players = vec![Option::None; 2];
        Context {
            frame_count: 0,
            players
        }
    }

    pub fn frame(&mut self) {
        self.frame_count += 1;
    }

    #[allow(dead_code)]
    pub fn get_frame(&self) -> i32 {
        self.frame_count
    }

    pub fn set_player(&mut self, player: usize, name: String) {
        let new_player = Some(Player::new(name));
        self.players[player] = new_player;
    }

    pub fn get_player(&self, player: usize) -> &Option<Player> {
        match &self.players.get(player) {
            Some(player_obj) => {
                player_obj
            }
            None => {
                &Option::None
            }
        }
    }
}
