#[derive(Clone)]
pub struct Player {
    name: String,
    rounds_won: i32
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            rounds_won: 0
        }
    }

    pub fn round_won(&mut self) {
        self.rounds_won += 1;
    }

    pub fn get_rounds_won(&self) -> i32 {
        self.rounds_won
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
