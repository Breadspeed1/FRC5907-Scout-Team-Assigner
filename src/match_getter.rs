pub fn get_matches() -> String {
    let res = reqwest::blocking::Client::new()
        .get("https://www.thebluealliance.com/api/v3/event/2022miket/matches")
        .header(
            "X-TBA-Auth-Key",
            "X2PBbgfhbG4iE0GX4UEXQnn16DcEoCl82FIwcPJtbBit6eoz15oG6fYztG7xCZXm"
        )
        .send()
        .expect("request failed")
        .text()
        .expect("no thingy >:(");

    return res;
}