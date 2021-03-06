use std::borrow::{Borrow};
use std::io::Write;
use std::ops::{Add};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use serde_json::Value;
use match_getter::get_matches;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use uptime_lib::get;

mod match_getter;

fn main() {
    let input_data = get_input();
    let amount_of_scouts: u32 = input_data.0 as u32;
    let data:(Vec<GameMatch>, Vec<i32>) = get_data(input_data.1);
    let game_matches:Vec<GameMatch> = data.0;
    let teams:Vec<i32> = data.1;
    let mut current_best:i32 = i32::MAX;
    let mut running:bool = true;
    let start_time:u64 = uptime_lib::get().expect("time literally go bye bye").as_secs();

    while running {
        let current_data:((Vec<ScoutSpot>, ScoutAssistant), i32);

        current_data = pass(&game_matches, teams.clone(), amount_of_scouts);

        let current_pass:(Vec<ScoutSpot>, Vec<(GameMatch, &Vec<i32>)>) = (current_data.0.0, current_data.0.1.get_tuples());
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

fn get_input() -> (i32, String) {
    for i in get_event_codes() {
        println!("{} -- {}", i.0, i.1);
    }

    let mut line = String::new();
    println!("Enter your event code (find from listed above) :");
    std::io::stdin().read_line(&mut line).expect("aaaa");

    let code: String = line;

    let mut line2 = String::new();
    println!("Enter the amount of scouts you have :");
    std::io::stdin().read_line(&mut line2).expect("d");

    let scouts: i32 = line2.trim().parse().expect("a");

    return (scouts, String::from(code.trim()));
}

fn get_event_codes() -> Vec<(String, String)> {
    let mut codes: Vec<(String, String)> = Vec::new();
    let data: Value = serde_json::from_str(match_getter::get_events().as_str()).expect("yaaa");

    for i in data.as_array().expect("ya") {
        codes.push((String::from(i["name"].as_str().expect("deez")), String::from(i["event_code"].as_str().expect("deez"))));
    }

    return codes;
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
                    if k != 0 {
                        let game_match: GameMatch = GameMatch{red:j.red, blue:j.blue};
                        match scout_assistant.teams_to_watch.entry(game_match) {
                            std::collections::hash_map::Entry::Vacant(e) => { e.insert(vec![*l]); }
                            std::collections::hash_map::Entry::Occupied(mut e) => { e.get_mut().push(*l); }
                        }
                    }
                }

                if *l == j.blue.1 {
                    k += 1;
                    if k != 0 {
                        let game_match: GameMatch = GameMatch{red:j.red, blue:j.blue};
                        match scout_assistant.teams_to_watch.entry(game_match) {
                            std::collections::hash_map::Entry::Vacant(e) => { e.insert(vec![*l]); }
                            std::collections::hash_map::Entry::Occupied(mut e) => { e.get_mut().push(*l); }
                        }
                    }
                }

                if *l == j.blue.2 {
                    k += 1;
                    if k != 0 {
                        let game_match: GameMatch = GameMatch{red:j.red, blue:j.blue};
                        match scout_assistant.teams_to_watch.entry(game_match) {
                            std::collections::hash_map::Entry::Vacant(e) => { e.insert(vec![*l]); }
                            std::collections::hash_map::Entry::Occupied(mut e) => { e.get_mut().push(*l); }
                        }
                    }
                }

                if *l == j.red.0 {
                    k += 1;
                    if k != 0 {
                        let game_match: GameMatch = GameMatch{red:j.red, blue:j.blue};
                        match scout_assistant.teams_to_watch.entry(game_match) {
                            std::collections::hash_map::Entry::Vacant(e) => { e.insert(vec![*l]); }
                            std::collections::hash_map::Entry::Occupied(mut e) => { e.get_mut().push(*l); }
                        }
                    }
                }

                if *l == j.red.1 {
                    k += 1;
                    if k != 0 {
                        let game_match: GameMatch = GameMatch{red:j.red, blue:j.blue};
                        match scout_assistant.teams_to_watch.entry(game_match) {
                            std::collections::hash_map::Entry::Vacant(e) => { e.insert(vec![*l]); }
                            std::collections::hash_map::Entry::Occupied(mut e) => { e.get_mut().push(*l); }
                        }
                    }
                }

                if *l == j.red.2 {
                    k += 1;
                    if k != 0 {
                        let game_match: GameMatch = GameMatch{red:j.red, blue:j.blue};
                        match scout_assistant.teams_to_watch.entry(game_match) {
                            std::collections::hash_map::Entry::Vacant(e) => { e.insert(vec![*l]); }
                            std::collections::hash_map::Entry::Occupied(mut e) => { e.get_mut().push(*l); }
                        }
                    }
                }
            }

            if k != -1 {
                conflicts += k;
            }
        }
    }

    return (conflicts, scout_assistant);
}

fn get_data(code: String) -> (Vec<GameMatch>, Vec<i32>) {
    let mut data:Vec<GameMatch> = Vec::new();
    let mut teams:Vec<i32> = Vec::new();
    let api_data:Value = serde_json::from_str(get_matches(code).as_str()).expect("didnt do the thing");

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
    teams_to_watch: HashMap<GameMatch, Vec<i32>>
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

    pub fn get_tuples(&self) -> Vec<(GameMatch, &Vec<i32>)> {
        let mut re: Vec<(GameMatch, &Vec<i32>)> = Vec::new();

        for i in self.teams_to_watch.iter() {
            re.push((GameMatch{blue: i.0.blue, red: i.0.red}, i.1));
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