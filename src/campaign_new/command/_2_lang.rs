use twilight_util::builder::command::StringBuilder;

use crate::{
    slash::CommandLocalize,
    translation::{Truncate, all_t, langs},
};

pub const CAMPAIGN_NEW_SET_LANG: &str = "language";

pub fn lang() -> StringBuilder {
    StringBuilder::init_localize(
        CAMPAIGN_NEW_SET_LANG,
        t!("campaign_new.language_description").c(100),
        all_t("campaign_new.language", 32),
        all_t("campaign_new.language_description", 100),
    )
    .required(true)
    .choices(langs(true))
}
