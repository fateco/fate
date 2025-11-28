use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub emoji: Option<String>,
    pub description: Option<String>,
    pub flags: SkillCapabilities,
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct SkillCapabilities: i64 {
        const PHYSICAL_VITAL = 1 << 0;
        const PHYSICAL_MELEE_ATTACK = 1 << 1;
        const PHYSICAL_MELEE_DEFEND = 1 << 2;
        const PHYSICAL_RANGED_ATTACK = 1 << 3;
        const PHYSICAL_RANGED_DEFEND = 1 << 4;
        const MENTAL_VITAL = 1 << 5;
        const MENTAL_MELEE_ATTACK = 1 << 6;
        const MENTAL_MELEE_DEFEND = 1 << 7;
        const MENTAL_RANGED_ATTACK = 1 << 8;
        const MENTAL_RANGED_DEFEND = 1 << 9;
    }
}
