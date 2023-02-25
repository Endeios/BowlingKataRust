#[allow(unused_variables)]
pub struct Game {
    rolls: [i32; 20],
    current_roll: usize,
}

impl Game {
    pub(crate) fn new() -> Game {
        Game {
            rolls: [0; 20],
            current_roll: 0,
        }
    }
}

impl Game {
    pub(crate) fn score(&self) -> i32 {
        let mut score = 0;
        for roll_index  in 0..20 {
            if self.rolls[roll_index] + self.rolls[roll_index+1]==10 {
                score += 10 + self.rolls[roll_index+2];
            } else {
                score += self.rolls[roll_index];
            }
        }
        score
    }
}

impl Game {
    pub(crate) fn roll(&mut self, number_of_pins: i32) {
        self.rolls[self.current_roll] = number_of_pins;
        self.current_roll += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn game_exists() {
        let _: Game = Game::new();
    }

    #[test]
    fn game_supports_gutter_game() {
        let mut game: Game = Game::new();
        roll_many(&mut game, 0, 20);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn game_supports_all_one_game() {
        let mut game: Game = Game::new();
        roll_many(&mut game, 1, 20);
        assert_eq!(game.score(), 20);
    }

    fn roll_many(game: &mut Game, number_of_pins: i32, times: i32) {
        for _ in 0..times {
            game.roll(number_of_pins);
        }
    }

    #[test]
    fn game_supports_spare() {
        let mut game: Game = Game::new();
        game.roll(5);
        game.roll(5);
        game.roll(3);
        game.roll(3);
        roll_many(&mut game, 0, 16);
        assert_eq!(game.score(), 19);
    }
}