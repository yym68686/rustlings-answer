// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<v[0].to_string()>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.


use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_Team(name: String, goals_scored: u8, goals_conceded: u8) -> Team {
    Team {
        name,
        goals_scored,
        goals_conceded,
    }
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        // let v[2].parse().unwrap(): u8 = v[2].parse().unwrap();
        // let v[0].to_string() = v[1].to_string();
        // let v[3].parse().unwrap(): u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        let team1 = build_Team (
            v[0].to_string(),
            v[2].parse().unwrap(),
            v[3].parse().unwrap(),
        );
        let team11 = build_Team (
            v[0].to_string(),
            0,
            0,
        );
        let team111 = build_Team (
            v[0].to_string(),
            v[2].parse().unwrap(),
            v[3].parse().unwrap(),
        );
        let team2 = build_Team (
            v[1].to_string(),
            v[3].parse().unwrap(),
            v[2].parse().unwrap(),
        );
        let team22 = build_Team (
            v[1].to_string(),
            0,
            0,
        );
        let team222 = build_Team (
            v[1].to_string(),
            v[3].parse().unwrap(),
            v[2].parse().unwrap(),
        );
        let count1 = scores.entry(team1.name).or_insert(team11);
        (*count1).goals_scored += team111.goals_scored;
        (*count1).goals_conceded += team111.goals_conceded;
        let count2 = scores.entry(team2.name).or_insert(team22);
        (*count2).goals_scored += team222.goals_scored;
        (*count2).goals_conceded += team222.goals_conceded;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
