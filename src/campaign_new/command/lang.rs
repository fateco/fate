use std::borrow::Cow;
use twilight_util::builder::command::StringBuilder;

use crate::translation::{Truncate, all_t};

pub const CAMPAIGN_NEW_SET_LANG: &str = "language";

pub fn lang() -> StringBuilder {
    StringBuilder::new(
        Cow::Borrowed(CAMPAIGN_NEW_SET_LANG).cut_n_fill(32),
        t!("campaign_new.lang_description").c(100),
    )
    .required(true)
    .name_localizations(
        all_t("campaign_new.lang")
            .iter()
            .map(|f| (f.0, f.1.cut_n_fill(32))),
    )
    .description_localizations(all_t("campaign_new.lang_description"))
}
