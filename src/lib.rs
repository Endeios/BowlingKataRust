
pub struct Game {

}

impl Game {
    pub(crate) fn score(&self) -> i32 {
        0
    }
}

impl Game {
    pub(crate) fn roll(&self, number_of_pins: i32) {
    }
}

#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn game_exists() {
        let game: Game = Game{};
    }

    #[test]
    fn game_supports_gutter_game(){
        let game: Game = Game{};
        game.roll(0);
        assert_eq!(game.score(), 0);

    }

}