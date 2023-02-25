#[allow(unused_variables)]

pub struct Game {
    score: i32,
}

impl Game {
    pub (crate) fn new() -> Game {
        Game {
            score:0
        }
    }
}
impl Game {
    pub(crate) fn score(&self) -> i32 {
        self.score
    }
}

impl Game {
    pub(crate) fn roll(&mut self, number_of_pins: i32) {
       self.score = self.score + number_of_pins;
    }
}

#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn game_exists() {
        let game: Game = Game::new();
    }

    #[test]
    fn game_supports_gutter_game() {
        let mut game: Game = Game::new();
        for _ in 0..20 {
            game.roll(0);
        }
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn game_supports_all_one_game() {
        let mut game: Game = Game::new();
        for _ in 0..20 {
            game.roll(1);
        }
        assert_eq!(game.score(), 20);
    }
}