use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

pub fn all_t(key: &str, length: usize) -> Vec<(&str, String)> {
    available_locales!()
        .iter()
        .filter(|&&locale| locale != "en-US")
        .map(|&locale| (locale, t!(key, locale = locale).c(length)))
        .collect()
}

pub fn langs(emoji_only: bool) -> Vec<(String, &'static str)> {
    available_locales!()
        .iter()
        .map(|&locale| {
            (
                if emoji_only {
                    t!("language.flag", locale = locale).e().to_string()
                } else {
                    format!(
                        "{} {}",
                        t!("language.flag", locale = locale),
                        t!("language.native", locale = locale)
                    )
                },
                locale,
            )
        })
        .collect()
}

pub trait Truncate {
    fn c(&self, length: usize) -> String;
    fn e(&self) -> &str;
}

impl Truncate for Cow<'_, str> {
    fn c(&self, length: usize) -> String {
        self.graphemes(false).take(length).collect()
    }

    fn e(&self) -> &str {
        self.graphemes(true)
            .next()
            .filter(|grapheme| emojis::get(grapheme).is_some())
            .unwrap_or("‼️")
    }
}
