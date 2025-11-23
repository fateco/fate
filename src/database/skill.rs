use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use twilight_model::id::{Id, marker::UserMarker};
use worker::{D1Database, D1PreparedStatement, Result, query};

#[derive(Serialize, Deserialize)]
struct Skill {
    id: i64,
    name: String,
    emoji: Option<String>,
    description: Option<String>,
    flags: SkillCapabilities,
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

pub fn db_get_skills_by_campaign_name(
    db: &D1Database,
    campaign_name: &str,
    user_id: Id<UserMarker>,
) -> Result<D1PreparedStatement> {
    query!(
        db,
        "
            SELECT s.*
            FROM skills s
            JOIN campaigns c ON s.campaign_id = c.id
            JOIN gms g ON c.id = g.campaign_id
            JOIN users u ON g.user_id = u.id
            WHERE c.name = ?1 AND u.discord_user_id = ?2
        ",
        campaign_name,
        user_id,
    )
}
