use twilight_model::id::{Id, marker::UserMarker};
use worker::{D1Database, Result, query};

use crate::database::model::skill::Skill;

pub async fn db_get_skills_by_campaign_name(
    db: &D1Database,
    user_id: Id<UserMarker>,
) -> Result<Vec<Skill>> {
    query!(
        db,
        "SELECT skills.*
            FROM
                skills
            JOIN
                campaigns ON skills.campaign_id = campaigns.id
            WHERE
                campaigns.name = ''
            JOIN
                gms ON campaigns.id = gms.campaign_id
            JOIN
                users ON gms.user_id = users.id
            WHERE
                users.discord_user_id = ?2",
        user_id,
    )?
    .all()
    .await?
    .results()
}
