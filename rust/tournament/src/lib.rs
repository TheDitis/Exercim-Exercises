use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::GameResult::{Draw, Loss, Win};

pub fn tally(match_results: &str) -> String {
    Tournament::new(match_results).to_string()
}


///----------------------------------------------------------------------------------------
/// TOURNAMENT
///----------------------------------------------------------------------------------------

struct Tournament {
    teams: HashMap<String, Team>,
}

impl Tournament {
    fn new(result_str: &str) -> Self {
        let mut teams = HashMap::new();
        let games: Vec<Game> = result_str.split('\n')
            .filter_map(|s| Game::try_from(s).ok()).collect();
        for game in games {
            let (team1, team2) =& game.teams;
            teams.entry(team1.clone()).or_insert_with(|| Team::new(team1)).add_game(game.clone());
            teams.entry(team2.clone()).or_insert_with(|| Team::new(team2)).add_game(game.clone());
        }
        Tournament { teams }
    }
}

impl ToString for Tournament {
    fn to_string(&self) -> String {
        let mut output = "Team                           | MP |  W |  D |  L |  P".to_string();
        let mut teams: Vec<&Team> = self.teams.values().collect();
        teams.sort_by(|team1, team2| team1.name.cmp(&team2.name));
        teams.sort_by(|team1, team2| team2.points.cmp(&team1.points));
        for team in teams {
            output += team.to_string().as_str();
        }
        output
    }
}

///----------------------------------------------------------------------------------------
/// GAME RESULT
///----------------------------------------------------------------------------------------

#[repr(u32)]
#[derive(Clone, Copy)]
enum GameResult {
    Win = 3,
    Loss = 0,
    Draw = 1,
}

impl GameResult {
    pub fn opposite(&self) -> Self {
        match self {
            Win => Loss,
            Loss => Win,
            Draw => Draw,
        }
    }
}

impl TryFrom<&str> for GameResult {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "win" => Ok(Win),
            "loss" => Ok(Loss),
            "draw" => Ok(Draw),
            _ => Err("Invalid game result str".to_string()),
        }
    }
}

///----------------------------------------------------------------------------------------
/// GAME
///----------------------------------------------------------------------------------------

#[derive(Clone)]
struct Game {
    teams: (String, String),
    result: GameResult,
}

impl TryFrom<&str> for Game {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split(';').collect();
        if split.len() != 3 { return Err("Invalid match string".to_string()) };
        Ok(Game {
            teams: (split[0].to_string(), split[1].to_string()),
            result: GameResult::try_from(split[2])?,
        })
    }
}

///----------------------------------------------------------------------------------------
/// TEAM
///----------------------------------------------------------------------------------------

#[derive(Default)]
struct Team {
    name: String,
    matches_played: u32,
    wins: u32,
    losses: u32,
    draws: u32,
    points: u32,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Team { name: name.to_string(), ..Team::default() }
    }

    pub fn add_game(&mut self, game: Game) {
        let result = if game.teams.0 == self.name { game.result } else{ game.result.opposite() };
        match result {
            Win => self.wins += 1,
            Loss => self.losses += 1,
            Draw => self.draws += 1,
        };
        self.points += result as u32;
        self.matches_played += 1;
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{: <31}{}",
            self.name,
            [self.matches_played, self.wins, self.draws, self.losses, self.points]
                .map(|v| { format!("|{: >3}", v) }).join(" ").as_str()
        )
    }
}
