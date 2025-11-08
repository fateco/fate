use twilight_util::builder::command::StringBuilder;

use crate::{slash::CommandLocalize, translation::all_t};

pub const CAMPAIGN_NEW_SET_DEFAULT: &str = "default-skills";

pub fn is_defualt_skills() -> StringBuilder {
    StringBuilder::init_localize(
        CAMPAIGN_NEW_SET_DEFAULT,
        all_t("campaign_new.set_default", 32),
        all_t("campaign_new.set_default_description", 100),
    )
    .required(true)
}
