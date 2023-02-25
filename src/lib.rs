#[allow(unused_variables)]
#[derive()]
pub struct Game {
    rolls: [i32; 20],
    current_roll: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            rolls: [0; 20],
            current_roll: 0,
        }
    }
}

impl Game {
    pub fn score(&self) -> i32 {
        let mut score = 0;
        let mut i: usize = 0;
        for _frame in 0..10 {
            if self.rolls[i] == 10 {
                score += self.current_frame_score(i) + self.rolls[i + 2];
                i += 1;
            } else if self.frame_is_spare(i) {
                score += self.current_frame_score(i) + self.rolls[i + 2];
                i += 2;
            } else {
                score += self.current_frame_score(i);
                i += 2;
            }
        }
        score
    }

    fn current_frame_score(&self, i: usize) -> i32 {
        self.rolls[i] + self.rolls[i + 1]
    }

    fn frame_is_spare(&self, i: usize) -> bool {
        self.rolls[i] + self.rolls[i + 1] == 10
    }
}

impl Game {
    pub fn roll(&mut self, number_of_pins: i32) {
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

    #[test]
    fn game_supports_strike(){
        let mut game: Game = Game::new();
        game.roll(10);
        game.roll(3);
        game.roll(4);
        roll_many(&mut game, 0, 16);
        assert_eq!(game.score(), 24);
    }

    #[test]
    fn game_supports_perfect_game(){
        let mut game: Game = Game::new();
        roll_many(&mut game, 10, 12);
        assert_eq!(game.score(), 300);
    }
}