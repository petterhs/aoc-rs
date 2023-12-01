#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

impl RockPaperScissors {
    fn beats(&self, other: &RockPaperScissors) -> bool {
        match self {
            RockPaperScissors::Rock => match other {
                RockPaperScissors::Rock => false,
                RockPaperScissors::Paper => false,
                RockPaperScissors::Scissors => true,
            },
            RockPaperScissors::Paper => match other {
                RockPaperScissors::Rock => true,
                RockPaperScissors::Paper => false,
                RockPaperScissors::Scissors => false,
            },
            RockPaperScissors::Scissors => match other {
                RockPaperScissors::Rock => false,
                RockPaperScissors::Paper => true,
                RockPaperScissors::Scissors => false,
            },
        }
    }
    fn score(self, other: &RockPaperScissors) -> u32 {
        if self.beats(other) {
            self.self_score() + 6u32
        } else if other.beats(&self) {
            self.self_score()
        } else {
            self.self_score() + 3u32
        }
    }

    fn self_score(self) -> u32 {
        self.into()
    }

    fn from_strategy(opponent: &RockPaperScissors, wanted_result: &Result) -> RockPaperScissors {
        match wanted_result {
            Result::Win => match opponent {
                RockPaperScissors::Rock => RockPaperScissors::Paper,
                RockPaperScissors::Paper => RockPaperScissors::Scissors,
                RockPaperScissors::Scissors => RockPaperScissors::Rock,
            },
            Result::Lose => match opponent {
                RockPaperScissors::Rock => RockPaperScissors::Scissors,
                RockPaperScissors::Paper => RockPaperScissors::Rock,
                RockPaperScissors::Scissors => RockPaperScissors::Paper,
            },
            Result::Draw => match opponent {
                RockPaperScissors::Rock => RockPaperScissors::Rock,
                RockPaperScissors::Paper => RockPaperScissors::Paper,
                RockPaperScissors::Scissors => RockPaperScissors::Scissors,
            },
        }
    }
}

impl Into<u32> for RockPaperScissors {
    fn into(self) -> u32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }
}

impl Into<RockPaperScissors> for char {
    fn into(self) -> RockPaperScissors {
        match self {
            'A' => RockPaperScissors::Rock,
            'B' => RockPaperScissors::Paper,
            'C' => RockPaperScissors::Scissors,
            _ => panic!("Invalid input"),
        }
    }
}

impl Into<Result> for char {
    fn into(self) -> Result {
        match self {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Invalid input"),
        }
    }
}

pub fn run() {
    println!("Hello, day 2");
    let file = std::fs::read_to_string("input/2").unwrap();
    let lines: Vec<&str> = file.split("\n").collect();

    let mut total_score = 0;
    let mut opponent: RockPaperScissors;
    let mut wanted_result: Result;
    let mut me: RockPaperScissors;
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        //parse the line with two chars
        let chars: Vec<char> = line.chars().collect();
        println!("Chars: {:?}", chars);
        opponent = chars[0].into();
        wanted_result = chars[2].into();

        me = RockPaperScissors::from_strategy(&opponent, &wanted_result);
        println!("Opponent: {:?}, Me: {:?}", opponent, me);

        let score = me.score(&opponent);
        println!("Score: {}", score);
        total_score += score;
    }
    println!("Total score: {}", total_score);
}
