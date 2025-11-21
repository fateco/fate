use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use worker::{D1Database, Result, query};

#[derive(Serialize, Deserialize, Debug)]
struct Campaign {
    id: i64,
    name: String,
    lang: String,
    gm_id: i64,
    created_at: NaiveDateTime,
}

pub async fn new_campaign(db: &D1Database, name: &str, lang: &str, gm_id: i64) -> Result<()> {
    query!(
        db,
        "INSERT INTO campaigns (name, lang, gm_id) VALUES (?1, ?2, ?3)",
        name,
        lang,
        gm_id
    )?;
    Ok(())
}
