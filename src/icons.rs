use serde_json::Value;

pub fn get_icons(name: &str) -> Value {
    match name {
        "awesome" => awesome_icons(),
        _ => no_icons()
    }
}

fn no_icons() -> Value {
    json!({
        "time": "",
        "music": "",
        "music_play": "  >  ",
        "music_pause": "  ||  ",
        "music_next": " => ",
        "music_prev": " <= ",
        "cogs": "",
        "memory_mem": " MEM",
        "memory_swap": " SWAP"
    })
}

fn awesome_icons() -> Value {
    json!({
        "time": " \u{f017} ",
        "music": " \u{f001} ",
        "music_play": "  \u{f04b}  ",
        "music_pause": "  \u{f04c}  ",
        "music_next": " \u{f061} ",
        "music_prev": " \u{f060} ",
        "cogs": " \u{f085} ",
        "memory_mem": " \u{f2db}",
        "memory_swap": " \u{f0a0}"
    })
}