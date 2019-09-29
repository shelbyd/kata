#[derive(Debug)]
struct Game {
    rolls: Vec<usize>,
}

impl Game {
    fn new() -> Self {
        Game { rolls: Vec::new() }
    }

    fn roll(&mut self, pins: usize) {
        self.rolls.push(pins);
    }

    fn score(&self) -> usize {
        self.frames().map(|f| f.score()).sum()
    }

    fn frames(&self) -> impl Iterator<Item = Frame> {
        let mut count = 0;
        std::iter::successors(Some(Frame::new(self)), move |current| {
            count += 1;
            if count < 10 {
                Some(current.next())
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
struct Frame<'g> {
    roll_index: usize,
    game: &'g Game,
}

impl<'g> Frame<'g> {
    fn new(game: &'g Game) -> Self {
        Frame {
            roll_index: 0,
            game,
        }
    }

    fn frame_type(&self) -> FrameType {
        match self.roll(0) {
            10 => Strike,
            x if x + self.roll(1) == 10 => Spare,
            _ => Points,
        }
    }

    fn roll(&self, increment: usize) -> usize {
        self.game.rolls[self.roll_index + increment]
    }

    fn score(&self) -> usize {
        match self.frame_type() {
            Strike => 10 + self.roll(1) + self.roll(2),
            Spare => 10 + self.roll(2),
            Points => self.roll(0) + self.roll(1),
        }
    }

    fn next(&self) -> Frame<'g> {
        let increment = match self.frame_type() {
            Strike => 1,
            Spare | Points => 2,
        };

        Frame {
            roll_index: self.roll_index + increment,
            game: self.game,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FrameType {
    Strike,
    Spare,
    Points,
}
use FrameType::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn roll_many(g: &mut Game, n: usize, pins: usize) {
        for _ in 0..n {
            g.roll(pins);
        }
    }

    fn roll_spare(g: &mut Game) {
        g.roll(5);
        g.roll(5);
    }

    fn roll_strike(g: &mut Game) {
        g.roll(10);
    }

    #[test]
    fn test_gutter_game() {
        let mut g = Game::new();
        roll_many(&mut g, 20, 0);
        assert_eq!(g.score(), 0);
    }

    #[test]
    fn test_all_ones() {
        let mut g = Game::new();
        roll_many(&mut g, 20, 1);
        assert_eq!(g.score(), 20);
    }

    #[test]
    fn test_one_spare() {
        let mut g = Game::new();

        roll_spare(&mut g);

        g.roll(3);

        roll_many(&mut g, 17, 0);
        assert_eq!(g.score(), 16);
    }

    #[test]
    fn test_one_strike() {
        let mut g = Game::new();

        roll_strike(&mut g);

        g.roll(3);
        g.roll(4);

        roll_many(&mut g, 16, 0);
        assert_eq!(g.score(), 24);
    }

    #[test]
    fn test_perfect_game() {
        let mut g = Game::new();

        roll_many(&mut g, 12, 10);

        assert_eq!(g.score(), 300);
    }
}
