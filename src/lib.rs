#[allow(unused_variables)]

pub struct Game {}

impl Game {
    pub(crate) fn score(&self) -> i32 {
        0
    }
}

impl Game {
    pub(crate) fn roll(&self, number_of_pins: i32) {}
}

#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn game_exists() {
        let game: Game = Game {};
    }

    #[test]
    fn game_supports_gutter_game() {
        let game: Game = Game {};
        for _ in 0..20 {
            game.roll(0);
        }
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn game_supports_all_one_game() {
        let game: Game = Game {};
        for _ in 0..20 {
            game.roll(1);
        }
        assert_eq!(game.score(), 20);
    }
}