use twilight_util::builder::command::StringBuilder;

use crate::{slash::CommandLocalize, translation::all_t};

pub const CAMPAIGN_NEW_SET_SKILLS: &str = "default-skills";

pub fn is_defualt_skills() -> StringBuilder {
    StringBuilder::init_localize(
        CAMPAIGN_NEW_SET_SKILLS,
        all_t("campaign_new.set_skills", 32),
        all_t("campaign_new.set_skills_description", 100),
    )
    .required(true)
    .choices([
        (t!("campaign_new.set_skills_default"), "true"),
        (t!("campaign_new.set_skills_custom"), "false"),
    ])
}
