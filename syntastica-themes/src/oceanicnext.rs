//! The oceanicnext theme collection in this module was extracted from <https://github.com/mhartington/oceanic-next> using `auto_extract.py`.

use std::collections::BTreeMap;

use syntastica_core::{
    style::{Color, Style},
    theme::ResolvedTheme,
};

#[rustfmt::skip]
pub fn dark() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(27, 43, 52)),
        ("boolean".to_owned(), Style::new(Color::new(249, 145, 87), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(236, 95, 103), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(171, 121, 103), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(101, 115, 126), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(197, 148, 197), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(249, 145, 87), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(197, 148, 197), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(216, 222, 233), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(236, 95, 103), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(197, 148, 197), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(236, 95, 103), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(249, 145, 87), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(102, 153, 204), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(236, 95, 103), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(197, 148, 197), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(197, 148, 197), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(197, 148, 197), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(236, 95, 103), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(102, 153, 204), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(216, 222, 233), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(249, 145, 87), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(249, 145, 87), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(250, 200, 99), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(250, 200, 99), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(216, 222, 233), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(171, 121, 103), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(250, 200, 99), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(250, 200, 99), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(153, 199, 148), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(171, 121, 103), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(171, 121, 103), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(153, 199, 148), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(216, 222, 233), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(101, 115, 126), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(98, 179, 178), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(102, 153, 204), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(250, 200, 99), false, false, false, false)),
        ("text.underline".to_owned(), Style::new(Color::new(236, 95, 103), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(236, 95, 103), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(250, 200, 99), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(197, 148, 197), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(205, 211, 222), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(236, 95, 103), false, false, false, false)),
    ]))
}

#[rustfmt::skip]
pub fn light() -> ResolvedTheme {
    ResolvedTheme::new(BTreeMap::from([
        ("bg0".to_owned(), Style::color_only(245, 247, 250)),
        ("boolean".to_owned(), Style::new(Color::new(219, 128, 77), false, false, false, false)),
        ("character".to_owned(), Style::new(Color::new(208, 84, 91), false, false, false, false)),
        ("character.special".to_owned(), Style::new(Color::new(150, 106, 91), false, false, false, false)),
        ("comment".to_owned(), Style::new(Color::new(167, 173, 186), false, false, false, false)),
        ("conditional".to_owned(), Style::new(Color::new(173, 130, 173), false, false, false, false)),
        ("constant".to_owned(), Style::new(Color::new(219, 128, 77), false, false, false, false)),
        ("constant.builtin".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("constant.macro".to_owned(), Style::new(Color::new(173, 130, 173), false, false, false, false)),
        ("constructor".to_owned(), Style::new(Color::new(27, 43, 52), false, false, false, false)),
        ("debug".to_owned(), Style::new(Color::new(208, 84, 91), false, false, false, false)),
        ("define".to_owned(), Style::new(Color::new(173, 130, 173), false, false, false, false)),
        ("exception".to_owned(), Style::new(Color::new(208, 84, 91), false, false, false, false)),
        ("field".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("float".to_owned(), Style::new(Color::new(219, 128, 77), false, false, false, false)),
        ("function".to_owned(), Style::new(Color::new(90, 135, 180), false, false, false, false)),
        ("function.builtin".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("function.macro".to_owned(), Style::new(Color::new(208, 84, 91), false, false, false, false)),
        ("include".to_owned(), Style::new(Color::new(173, 130, 173), false, false, false, false)),
        ("keyword".to_owned(), Style::new(Color::new(173, 130, 173), false, false, false, false)),
        ("keyword.operator".to_owned(), Style::new(Color::new(173, 130, 173), false, false, false, false)),
        ("label".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("macro".to_owned(), Style::new(Color::new(208, 84, 91), false, false, false, false)),
        ("method".to_owned(), Style::new(Color::new(90, 135, 180), false, false, false, false)),
        ("namespace".to_owned(), Style::new(Color::new(27, 43, 52), false, false, false, false)),
        ("number".to_owned(), Style::new(Color::new(219, 128, 77), false, false, false, false)),
        ("operator".to_owned(), Style::new(Color::new(219, 128, 77), false, false, false, false)),
        ("parameter".to_owned(), Style::new(Color::new(220, 176, 87), false, false, false, false)),
        ("preproc".to_owned(), Style::new(Color::new(220, 176, 87), false, false, false, false)),
        ("property".to_owned(), Style::new(Color::new(27, 43, 52), false, false, false, false)),
        ("punctuation".to_owned(), Style::new(Color::new(150, 106, 91), false, false, false, false)),
        ("punctuation.bracket".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("punctuation.delimiter".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("punctuation.special".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("repeat".to_owned(), Style::new(Color::new(220, 176, 87), false, false, false, false)),
        ("storageclass".to_owned(), Style::new(Color::new(220, 176, 87), false, false, false, false)),
        ("string".to_owned(), Style::new(Color::new(135, 175, 130), false, false, false, false)),
        ("string.escape".to_owned(), Style::new(Color::new(150, 106, 91), false, false, false, false)),
        ("string.special".to_owned(), Style::new(Color::new(150, 106, 91), false, false, false, false)),
        ("symbol".to_owned(), Style::new(Color::new(135, 175, 130), false, false, false, false)),
        ("tag".to_owned(), Style::new(Color::new(27, 43, 52), false, false, false, false)),
        ("tag.delimiter".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("text.literal".to_owned(), Style::new(Color::new(167, 173, 186), false, false, false, false)),
        ("text.reference".to_owned(), Style::new(Color::new(86, 158, 157), false, false, false, false)),
        ("text.title".to_owned(), Style::new(Color::new(90, 135, 180), false, false, false, true)),
        ("text.todo".to_owned(), Style::new(Color::new(220, 176, 87), false, false, false, false)),
        ("text.underline".to_owned(), Style::new(Color::new(208, 84, 91), true, false, false, false)),
        ("text.uri".to_owned(), Style::new(Color::new(208, 84, 91), true, false, false, false)),
        ("type".to_owned(), Style::new(Color::new(220, 176, 87), false, false, false, false)),
        ("type.definition".to_owned(), Style::new(Color::new(173, 130, 173), false, false, false, false)),
        ("variable".to_owned(), Style::new(Color::new(52, 61, 70), false, false, false, false)),
        ("variable.builtin".to_owned(), Style::new(Color::new(208, 84, 91), false, false, false, false)),
    ]))
}