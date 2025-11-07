use rand::RngCore;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

pub fn all_t(key: &str) -> Vec<(&str, Cow<'_, str>)> {
    available_locales!()
        .iter()
        .map(|&locale| (locale, t!(key, locale = locale)))
        .collect()
}

pub trait Truncate {
    fn c(&self, length: usize) -> Cow<'_, str>;
    fn cut_n_fill(&self, length: usize) -> Cow<'_, str>;
    fn e(&self) -> Cow<'_, str>;
}

impl Truncate for Cow<'_, str> {
    fn c(&self, length: usize) -> Cow<'_, str> {
        self.graphemes(false).take(length).collect()
    }

    fn cut_n_fill(&self, length: usize) -> Cow<'_, str> {
        if self.is_empty() {
            let mut err = format!("missing_str_{}", rand::rng().next_u32());
            err.truncate(length);
            err.into()
        } else {
            let text: Cow<'_, str> = self.graphemes(false).take(length).collect();
            text.to_lowercase().replace('.', "-").into()
        }
    }

    fn e(&self) -> Cow<'_, str> {
        self.graphemes(true)
            .next()
            .filter(|grapheme| emojis::get(grapheme).is_some())
            .map_or(Cow::Borrowed("‼️"), Cow::Borrowed)
    }
}
