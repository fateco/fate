use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

pub trait Truncate {
    fn c(&self, length: usize) -> Cow<'_, str>;
    fn e(&self) -> Cow<'_, str>;
}

impl Truncate for Cow<'_, str> {
    fn c(&self, length: usize) -> Cow<'_, str> {
        self.graphemes(false).take(length).collect()
    }

    fn e(&self) -> Cow<'_, str> {
        self.graphemes(true)
            .next()
            .filter(|grapheme| emojis::get(grapheme).is_some())
            .map_or(Cow::Borrowed("‼️"), Cow::Borrowed)
    }
}
