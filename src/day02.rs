#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use anyhow::{anyhow, Context, Result};

    use crate::day02::tests::GameResult::{DRAW, LOSS, WIN};
    use crate::day02::tests::Move::{PAPER, ROCK, SCISSORS};

    #[derive(Copy, Clone)]
    enum Move {
        ROCK = 1,
        PAPER = 2,
        SCISSORS = 3,
    }

    impl TryFrom<char> for Move {
        type Error = anyhow::Error;

        fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
            Ok(match value {
                'A' | 'X' => ROCK,
                'B' | 'Y' => PAPER,
                'C' | 'Z' => SCISSORS,
                other => Err(anyhow!("Invalid move: '{}'", other))?,
            })
        }
    }

    enum GameResult {
        LOSS = 0,
        DRAW = 3,
        WIN = 6,
    }

    impl TryFrom<char> for GameResult {
        type Error = anyhow::Error;

        fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
            Ok(match value {
                'X' => LOSS,
                'Y' => DRAW,
                'Z' => WIN,
                other => Err(anyhow!("Invalid game result: '{}'", other))?,
            })
        }
    }

    impl GameResult {
        fn determine(you: Move, them: Move) -> Self {
            match you {
                ROCK => match them {
                    ROCK => DRAW,
                    PAPER => LOSS,
                    SCISSORS => WIN,
                }
                PAPER => match them {
                    ROCK => WIN,
                    PAPER => DRAW,
                    SCISSORS => LOSS,
                }
                SCISSORS => match them {
                    ROCK => LOSS,
                    PAPER => WIN,
                    SCISSORS => DRAW,
                }
            }
        }

        fn required(&self, them: Move) -> Move {
            match them {
                ROCK => match self {
                    LOSS => SCISSORS,
                    DRAW => ROCK,
                    WIN => PAPER,
                }
                PAPER => match self {
                    LOSS => ROCK,
                    DRAW => PAPER,
                    WIN => SCISSORS,
                }
                SCISSORS => match self {
                    LOSS => PAPER,
                    DRAW => SCISSORS,
                    WIN => ROCK,
                }
            }
        }
    }

    #[test]
    fn part_1() -> Result<()> {
        let file = File::open("day02.txt").unwrap();
        let reader = BufReader::new(file);

        let mut score = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut chars = line.chars();
            let opponent: Move = chars.nth(0).unwrap().try_into().with_context(|| "opponent")?;
            let response: Move = chars.nth(1).unwrap().try_into().with_context(|| "response")?;

            score += response as i32;
            score += GameResult::determine(response, opponent) as i32;
        }
        println!("Total score is: {}", score);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let mut score = 0;
        for line in BufReader::new(File::open("day02.txt").unwrap()).lines() {
            let line = line.unwrap();
            let mut chars = line.chars();
            let opponent: Move = chars.nth(0).unwrap().try_into()?;
            let outcome: GameResult = chars.nth(1).unwrap().try_into()?;

            score += outcome.required(opponent) as i32;
            score += outcome as i32;
        }
        println!("Total score is: {}", score);

        Ok(())
    }
}
