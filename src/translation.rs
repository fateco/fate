use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

pub fn all_t(key: &str, length: usize) -> Vec<(&str, String)> {
    available_locales!()
        .iter()
        .map(|&locale| (locale, t!(key, locale = locale).c(length)))
        .collect()
}

pub trait Truncate {
    fn c(&self, length: usize) -> String;
    fn e(&self) -> Cow<'_, str>;
}

impl Truncate for Cow<'_, str> {
    fn c(&self, length: usize) -> String {
        self.graphemes(false).take(length).collect()
    }

    fn e(&self) -> Cow<'_, str> {
        self.graphemes(true)
            .next()
            .filter(|grapheme| emojis::get(grapheme).is_some())
            .map_or(Cow::Borrowed("‼️"), Cow::Borrowed)
    }
}
