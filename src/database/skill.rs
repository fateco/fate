use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Skill {
    id: i64,
    name: String,
    emoji: Option<String>,
    description: Option<String>,
}
