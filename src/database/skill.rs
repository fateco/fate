use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Skill {
    id: i64,
    name: String,
    emoji: Option<String>,
    description: Option<String>,
    flags: i64,
}
