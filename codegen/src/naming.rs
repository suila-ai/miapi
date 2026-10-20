//! spec に出てくる名前を Rust の識別子に変換する。

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span};

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "static",
    "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try", "gen",
];

/// 生識別子として書けないキーワード。
const UNRAWABLE: &[&str] = &["self", "super", "crate", "Self"];

/// フィールド・モジュール・メソッドに使う snake_case の識別子
/// （`noteId` -> `note_id`、`2fa` -> `_2fa`、`self` -> `self_`）。
/// キーワードはそのまま返す。生識別子にするのは [`ident`] である。
pub fn snake(name: &str) -> String {
    let mut out = name.to_snake_case();
    if out.is_empty() {
        out = "_".to_owned();
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    if UNRAWABLE.contains(&out.as_str()) {
        out.push('_');
    }
    out
}

/// 型と enum のバリアントに使う UpperCamelCase の識別子
/// （`+createdAt` -> `PlusCreatedAt`、`reaction:grouped` -> `ReactionGrouped`、`1` -> `_1`）。
pub fn pascal(name: &str) -> String {
    let mut prefix = String::new();
    let rest = match name.chars().next() {
        Some('+') => {
            prefix.push_str("Plus");
            &name[1..]
        }
        Some('-') => {
            prefix.push_str("Minus");
            &name[1..]
        }
        _ => name,
    };
    let mut out = prefix + &rest.to_upper_camel_case();
    if out.is_empty() {
        out = "Empty".to_owned();
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

/// 識別子の文字列をトークンにする。キーワードは生識別子にする（`type` -> `r#type`）。
/// 生識別子にできない `Self` などは、末尾に `_` を足す（`Self` -> `Self_`）。
/// [`pascal`] の結果は型名の途中にも使うので、そちらで足すと `FooSelf_Item` のような名前になり、
/// non_camel_case_types の警告が出る。
pub fn ident(name: &str) -> Ident {
    if UNRAWABLE.contains(&name) {
        Ident::new(&format!("{name}_"), Span::call_site())
    } else if KEYWORDS.contains(&name) {
        Ident::new_raw(name, Span::call_site())
    } else {
        Ident::new(name, Span::call_site())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_handles_digits_and_unrawable_words() {
        assert_eq!(snake("noteId"), "note_id");
        assert_eq!(snake("abuse-user-reports"), "abuse_user_reports");
        assert_eq!(snake("type"), "type");
        assert_eq!(snake("self"), "self_");
        assert_eq!(snake("2fa"), "_2fa");
        assert_eq!(snake("v2"), "v2");
        assert_eq!(snake("v2024.10.1"), "v2024_10_1");
    }

    #[test]
    fn pascal_handles_signs_symbols_and_digits() {
        assert_eq!(pascal("createdAt"), "CreatedAt");
        assert_eq!(pascal("+createdAt"), "PlusCreatedAt");
        assert_eq!(pascal("-createdAt"), "MinusCreatedAt");
        assert_eq!(pascal("reaction:grouped"), "ReactionGrouped");
        assert_eq!(pascal("1"), "_1");
        assert_eq!(pascal("followersLessThanOrEq"), "FollowersLessThanOrEq");
        assert_eq!(pascal("abuse-user-reports"), "AbuseUserReports");
        assert_eq!(pascal("self"), "Self");
    }

    #[test]
    fn ident_handles_keywords_and_unrawable_words() {
        assert_eq!(ident("type").to_string(), "r#type");
        assert_eq!(ident("move").to_string(), "r#move");
        assert_eq!(ident("override").to_string(), "r#override");
        assert_eq!(ident("note_id").to_string(), "note_id");
        assert_eq!(ident("Self").to_string(), "Self_");
    }
}
