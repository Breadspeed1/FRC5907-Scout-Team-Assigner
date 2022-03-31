pub fn get_matches(event_code: String) -> String {
    let res = reqwest::blocking::Client::new()
        .get((String::from("https://www.thebluealliance.com/api/v3/event/2022") + event_code.as_str() + "/matches").as_str())
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

pub fn get_events() -> String {
    let res = reqwest::blocking::Client::new()
        .get("https://www.thebluealliance.com/api/v3/events/2022/simple")
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