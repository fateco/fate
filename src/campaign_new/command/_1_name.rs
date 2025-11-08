use twilight_util::builder::command::StringBuilder;

use crate::{slash::CommandLocalize, translation::all_t};

pub const CAMPAIGN_NEW_SET_NAME: &str = "name";

pub fn name() -> StringBuilder {
    StringBuilder::init_localize(
        CAMPAIGN_NEW_SET_NAME,
        all_t("campaign_new.name", 32),
        all_t("campaign_new.name_description", 100),
    )
    .required(true)
    .min_length(1)
    .max_length(20)
}
