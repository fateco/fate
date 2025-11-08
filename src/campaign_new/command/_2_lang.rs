use twilight_util::builder::command::StringBuilder;

use crate::{slash::CommandLocalize, translation::all_t};

pub const CAMPAIGN_NEW_SET_LANG: &str = "language";

pub fn lang() -> StringBuilder {
    StringBuilder::init_localize(
        CAMPAIGN_NEW_SET_LANG,
        all_t("campaign_new.language", 32),
        all_t("campaign_new.language_description", 100),
    )
    .required(true)
}
