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

use std::collections::HashMap;

// A structure to store the goal details of a team.

struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();
    let mut v: Vec<String> = Vec::new();

    for line in results.lines() {
        let items: Vec<&str> = line.split(',').collect();
        for item in &items {
            v.push(format!("{}", item));
        }
    }

    let mut i = 0;
    while i < v.len() {
        if v[i].len() > 1 {
            for (key, _value) in &scores {
                if *key == v[i] && i + 1 < v.len() {
                    i += 1;
                }
            }
            if v[i].len() > 1 {
                scores.insert(
                    v[i].to_string(),
                    Team {
                        goals_scored: 0,
                        goals_conceded: 0,
                    },
                );
                i += 1;
            }
        } else if v[i].len() == 1 {
            let mut goal_conceded_index = 0;

            if i + 1 != v.len() && v[i + 1].len() == 1 {
                goal_conceded_index = i + 1;
            } else if i + 1 == v.len() {
                goal_conceded_index = i - 1;
            } else if v[i + 1].len() > 1 && i + 1 != v.len() {
                goal_conceded_index = i - 1;
            }

            if let Some(team) = scores.get_mut(&v[i - 2].to_string()) {
                team.goals_scored += v[i].parse().unwrap_or(0);
            }

            if let Some(team) = scores.get_mut(&v[i - 2].to_string()) {
                team.goals_conceded += v[goal_conceded_index].parse().unwrap_or(0);
            }

            i += 1;
            goal_conceded_index = 0;
        }
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n" // England : 5 4    France: 5 6     Italy: 1 3    Poland: 2 0  // Germany 2 1  Spain 0 0 
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
