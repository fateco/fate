use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

pub fn all_t(key: &str, length: usize) -> Vec<(&str, String)> {
    available_locales!()
        .iter()
        .filter(|&&locale| locale != "en-US")
        .map(|&locale| (locale, t!(key, locale = locale).c(length)))
        .collect()
}

pub fn langs(with_text: bool) -> Vec<(String, &'static str)> {
    available_locales!()
        .iter()
        .map(|&locale| {
            (
                if with_text {
                    format!(
                        "{} {}",
                        t!("language.flag", locale = locale),
                        t!("language.native", locale = locale)
                    )
                } else {
                    t!("language.flag", locale = locale).to_string()
                },
                locale,
            )
        })
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
