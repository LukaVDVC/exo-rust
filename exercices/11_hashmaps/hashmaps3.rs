// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::collections::HashMap;
#[derive(Debug, Default)]
struct Team {
goals_scored: u8,
goals_conceded: u8,
}
fn build_scores_table(results: &str) -> HashMap<String, Team> {
let mut scores: HashMap<String, Team> = HashMap::new();
for r in results.lines() {
let v: Vec<&str> = r.split(',').collect();
let team_1_name = v[0].to_string();
let team_1_score: u8 = v[2].parse().unwrap();
let team_2_name = v[1].to_string();
let team_2_score: u8 = v[3].parse().unwrap();
let team1 = scores.entry(team_1_name.clone()).or_default();
team1.goals_scored += team_1_score;
team1.goals_conceded += team_2_score;
let team2 = scores.entry(team_2_name.clone()).or_default();
team2.goals_scored += team_2_score;
team2.goals_conceded += team_1_score;
}
scores
}
