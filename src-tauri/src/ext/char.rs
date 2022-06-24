use std::ops::RangeInclusive;

static KANJI_RANGE: RangeInclusive<char> = '\u{4E00}' ..= '\u{9FAF}';

pub trait CharExt {
    fn is_kanji(&self) -> bool;
    fn is_open_ruby_parenthesis(&self) -> bool;
    fn is_closed_ruby_parenthesis(&self) -> bool;
}

impl CharExt for char {
    fn is_kanji(&self) -> bool {
       KANJI_RANGE.contains(self)
    }

    fn is_open_ruby_parenthesis(&self) -> bool {
        *self == '「' || *self == '['
    }

    fn is_closed_ruby_parenthesis(&self) -> bool {
        *self == '」' || *self == ']'
    }
}