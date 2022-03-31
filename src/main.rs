use std::borrow::{Borrow, BorrowMut};
use std::io::Write;
use std::ops::{Add, Rem};
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use serde_json::Value;
use match_getter::get_matches;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;

mod match_getter;

fn main() {
    let data:(Vec<GameMatch>, Vec<i32>) = get_data();
    let game_matches:Vec<GameMatch> = data.0;
    let teams:Vec<i32> = data.1;
    let amount_of_scouts:u32 = 15;
    let mut current_best:i32 = i32::MAX;
    let mut running:bool = true;
    let start_time:u64 = uptime_lib::get().expect("time literally go bye bye").as_secs();

    while running {
        let current_data:((Vec<ScoutSpot>, ScoutAssistant), i32);

        current_data = pass(&game_matches, teams.clone(), amount_of_scouts);

        let current_pass:(Vec<ScoutSpot>, Vec<(GameMatch, i32)>) = (current_data.0.0, current_data.0.1.get_tuples());
        let current_conflict = current_data.1;

        if current_conflict < current_best {
            current_best = current_conflict;
            println!("new current best!! {}, it took {} second(s)", current_best, uptime_lib::get().expect("time died").as_secs() - start_time);

            let json: String = serde_json::to_string(&current_pass).expect("failed to parse");
            let mut file: File = std::fs::File::create(String::from("Results\\").add(current_best.to_string().as_str()).add(".json")).expect("good file no creaty prob cause of weird path");
            file.write_all(json.as_bytes()).expect("everything is on fire");

            if current_conflict < 1 {
                running = false;
            }
        }
    }

    println!("we did it boys");
}

fn pass(game_matches:&Vec<GameMatch>, teams: Vec<i32>, amount_of_scouts:u32) -> ((Vec<ScoutSpot>, ScoutAssistant), i32) {
    let mut scout_spots:Vec<ScoutSpot>;
    let mut teams_to_scout:Vec<i32> = teams.clone();
    scout_spots = Vec::with_capacity(amount_of_scouts as usize);

    for i in 0..amount_of_scouts { scout_spots.push(ScoutSpot::new()); }

    teams_to_scout.shuffle(&mut thread_rng());

    let mut j:usize = 0;
    for i in teams_to_scout {
        if j > (amount_of_scouts - 1) as usize {
            j = 0;
        }

        scout_spots[j].teams_to_watch.push(i);
        j += 1;
    }

    let data = &mut *scout_spots;

    let conflict_data: (i32, ScoutAssistant) = calc_conflicts(game_matches, data.to_vec());

    return ((data.to_vec(), conflict_data.1), conflict_data.0);
}

fn calc_conflicts(game_matches:&Vec<GameMatch>, data:Vec<ScoutSpot>) -> (i32, ScoutAssistant) {
    let mut conflicts: i32 = 0;
    let scout_spots: Vec<ScoutSpot> = data.clone();
    let mut scout_assistant: ScoutAssistant = ScoutAssistant::new();

    for i in scout_spots {
        for j in game_matches {
            let mut k:i32 = -1;
            for l in &i.teams_to_watch {
                if *l == j.blue.0 {
                    k += 1;
                    scout_assistant.teams_to_watch.insert(GameMatch{red:j.red, blue:j.blue}, *l);
                }

                if *l == j.blue.1 {
                    k += 1;
                    scout_assistant.teams_to_watch.insert(GameMatch{red:j.red, blue:j.blue}, *l);
                }

                if *l == j.blue.2 {
                    k += 1;
                    scout_assistant.teams_to_watch.insert(GameMatch{red:j.red, blue:j.blue}, *l);
                }

                if *l == j.red.0 {
                    k += 1;
                    scout_assistant.teams_to_watch.insert(GameMatch{red:j.red, blue:j.blue}, *l);
                }

                if *l == j.red.1 {
                    k += 1;
                    scout_assistant.teams_to_watch.insert(GameMatch{red:j.red, blue:j.blue}, *l);
                }

                if *l == j.red.2 {
                    k += 1;
                    scout_assistant.teams_to_watch.insert(GameMatch{red:j.red, blue:j.blue}, *l);
                }
            }

            if k != -1 {
                conflicts += k;
            }
        }
    }

    return (conflicts, scout_assistant);
}

fn get_data() -> (Vec<GameMatch>, Vec<i32>) {
    let mut data:Vec<GameMatch> = Vec::new();
    let mut teams:Vec<i32> = Vec::new();
    let api_data:Value = serde_json::from_str(get_matches().as_str()).expect("didnt do the thing");

    for i in api_data.as_array().unwrap() {
        data.push(GameMatch::from_team_json(i));
    }

    for i in &data {
        if !teams.contains(i.blue.0.borrow()) { teams.push(i.blue.0); }
        if !teams.contains(i.blue.1.borrow()) { teams.push(i.blue.1); }
        if !teams.contains(i.blue.2.borrow()) { teams.push(i.blue.2); }
        if !teams.contains(i.red.0.borrow()) { teams.push(i.red.0); }
        if !teams.contains(i.red.1.borrow()) { teams.push(i.red.1); }
        if !teams.contains(i.red.2.borrow()) { teams.push(i.red.2); }
    }

    return (data, teams);
}

#[derive(Serialize)]
struct ScoutSpot {
    teams_to_watch: Vec<i32>,
}

impl Clone for ScoutSpot {
    fn clone(&self) -> Self {
        return ScoutSpot {teams_to_watch:self.teams_to_watch.clone()};
    }
}

impl ScoutSpot {
    pub fn new() -> ScoutSpot {
        return ScoutSpot {teams_to_watch:Vec::new()};
    }
}

#[derive(Serialize)]
struct ScoutAssistant {
    teams_to_watch: HashMap<GameMatch, i32>
}

impl std::hash::Hash for GameMatch {
    fn hash<H: Hasher>(&self, state: &mut H) where H: std::hash::Hasher {
        state.write_i32(self.red.0);
        state.write_i32(self.red.1);
        state.write_i32(self.red.2);
        state.write_i32(self.blue.0);
        state.write_i32(self.blue.1);
        state.write_i32(self.blue.2);
    }
}

impl PartialEq<Self> for GameMatch {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.blue == other.blue
    }
}

impl Eq for GameMatch {}

impl ScoutAssistant {
    pub fn new() -> ScoutAssistant {
        return ScoutAssistant {teams_to_watch:HashMap::new()};
    }

    pub fn get_tuples(&self) -> Vec<(GameMatch, i32)> {
        let mut re: Vec<(GameMatch, i32)> = Vec::new();

        for i in self.teams_to_watch.iter() {
            re.push((GameMatch{blue: i.0.blue, red: i.0.red}, *i.1));
        }

        return re;
    }
}

#[derive(Serialize)]
struct GameMatch {
    red:(i32, i32, i32),
    blue:(i32, i32, i32)
}

impl GameMatch {
    pub fn from_team_json(v:&Value) -> GameMatch {
        return GameMatch {
            red: (
                String::from(v["alliances"]["red"]["team_keys"][0].as_str().expect("me when")).split("c").collect::<Vec<&str>>()[1].parse().expect("failed parsing json"),
                String::from(v["alliances"]["red"]["team_keys"][1].as_str().expect("me when")).split("c").collect::<Vec<&str>>()[1].parse().expect("failed parsing json"),
                String::from(v["alliances"]["red"]["team_keys"][2].as_str().expect("me when")).split("c").collect::<Vec<&str>>()[1].parse().expect("failed parsing json"))
            ,
            blue: (
                String::from(v["alliances"]["blue"]["team_keys"][0].as_str().expect("me when")).split("c").collect::<Vec<&str>>()[1].parse().expect("failed parsing json"),
                String::from(v["alliances"]["blue"]["team_keys"][1].as_str().expect("me when")).split("c").collect::<Vec<&str>>()[1].parse().expect("failed parsing json"),
                String::from(v["alliances"]["blue"]["team_keys"][2].as_str().expect("me when")).split("c").collect::<Vec<&str>>()[1].parse().expect("failed parsing json")
            )
        };
    }
}