use std::borrow::Cow;
use twilight_util::builder::command::StringBuilder;

use crate::translation::{Truncate, all_t};

pub const CAMPAIGN_NEW_SET_NAME: &str = "name";

pub fn name() -> StringBuilder {
    StringBuilder::new(
        Cow::Borrowed(CAMPAIGN_NEW_SET_NAME).cut_n_fill(32),
        t!("campaign_new.name_description").c(100),
    )
    .required(true)
    .min_length(1)
    .max_length(20)
    .name_localizations(
        all_t("campaign_new.name")
            .iter()
            .map(|f| (f.0, f.1.cut_n_fill(32))),
    )
    .description_localizations(all_t("campaign_new.name_description"))
}
