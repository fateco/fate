use twilight_util::builder::command::StringBuilder;

use crate::{
    slash::CommandLocalize,
    translation::{Truncate, all_t},
};

pub const CAMPAIGN_NEW_SET_SKILLS: &str = "skill";

pub fn is_defualt_skills() -> StringBuilder {
    StringBuilder::init_localize(
        CAMPAIGN_NEW_SET_SKILLS,
        t!("campaign_new.set_skills_description").c(100),
        all_t("campaign_new.set_skills", 32),
        all_t("campaign_new.set_skills_description", 100),
    )
    .required(true)
    .choices([
        (t!("campaign_new.set_skills_default").c(100), "true"),
        (t!("campaign_new.set_skills_custom").c(100), "false"),
    ])
}
