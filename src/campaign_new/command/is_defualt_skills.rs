use std::borrow::Cow;
use twilight_util::builder::command::BooleanBuilder;

use crate::translation::{Truncate, all_t};

pub const CAMPAIGN_NEW_SET_DEFAULT: &str = "default-skills";

pub fn is_defualt_skills() -> BooleanBuilder {
    BooleanBuilder::new(
        Cow::Borrowed(CAMPAIGN_NEW_SET_DEFAULT).cut_n_fill(32),
        t!("campaign_new.use_default_skills_description").c(100),
    )
    .required(true)
    .name_localizations(all_t("test").iter().map(|f| (f.0, f.1.cut_n_fill(32))))
    .description_localizations(all_t("campaign_new.use_default_skills_description"))
}
