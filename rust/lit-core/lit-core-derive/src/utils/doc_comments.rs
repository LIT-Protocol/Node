//! Copied from: https://github.com/clap-rs/clap/blob/master/clap_derive/src/utils/doc_comments.rs
//! (wasn't exported sadly).
//!
//! The preprocessing we apply to doc comments.
//!
//! #[derive(Parser)] works in terms of "paragraphs". Paragraph is a sequence of
//! non-empty adjacent lines, delimited by sequences of blank (whitespace only) lines.

use std::iter;

#[allow(dead_code)]
pub fn extract_doc_comment(attrs: &[syn::Attribute]) -> Vec<String> {
    use syn::Lit::*;
    use syn::Meta::*;
    use syn::MetaNameValue;

    // multiline comments (`/** ... */`) may have LFs (`\n`) in them,
    // we need to split so we could handle the lines correctly
    //
    // we also need to remove leading and trailing blank lines
    let mut lines: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .filter_map(|attr| {
            if let Ok(NameValue(MetaNameValue { lit: Str(s), .. })) = attr.parse_meta() {
                Some(s.value())
            } else {
                // non #[doc = "..."] attributes are not our concern
                // we leave them for rustc to handle
                None
            }
        })
        .skip_while(|s| is_blank(s))
        .flat_map(|s| {
            let lines = s
                .split('\n')
                .map(|s| {
                    // remove one leading space no matter what
                    let s = s.strip_prefix(' ').unwrap_or(s);
                    s.to_owned()
                })
                .collect::<Vec<_>>();
            lines
        })
        .collect();

    while let Some(true) = lines.last().map(|s| is_blank(s)) {
        lines.pop();
    }

    lines
}

#[allow(dead_code)]
pub fn format_doc_comment(
    lines: &[String], preprocess: bool, force_long: bool,
) -> (Option<String>, Option<String>) {
    if let Some(first_blank) = lines.iter().position(|s| is_blank(s)) {
        let (short, long) = if preprocess {
            let paragraphs = split_paragraphs(lines);
            let short = paragraphs[0].clone();
            let long = paragraphs.join("\n\n");
            (remove_period(short), long)
        } else {
            let short = lines[..first_blank].join("\n");
            let long = lines.join("\n");
            (short, long)
        };

        (Some(short), Some(long))
    } else {
        let (short, long) = if preprocess {
            let short = merge_lines(lines);
            let long = force_long.then(|| short.clone());
            let short = remove_period(short);
            (short, long)
        } else {
            let short = lines.join("\n");
            let long = force_long.then(|| short.clone());
            (short, long)
        };

        (Some(short), long)
    }
}

#[allow(dead_code)]
fn split_paragraphs(lines: &[String]) -> Vec<String> {
    let mut last_line = 0;
    iter::from_fn(|| {
        let slice = &lines[last_line..];
        let start = slice.iter().position(|s| !is_blank(s)).unwrap_or(0);

        let slice = &slice[start..];
        let len = slice.iter().position(|s| is_blank(s)).unwrap_or(slice.len());

        last_line += start + len;

        if len != 0 {
            Some(merge_lines(&slice[..len]))
        } else {
            None
        }
    })
    .collect()
}

#[allow(dead_code)]
fn remove_period(mut s: String) -> String {
    if s.ends_with('.') && !s.ends_with("..") {
        s.pop();
    }
    s
}

#[allow(dead_code)]
fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

#[allow(dead_code)]
fn merge_lines(lines: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    lines.into_iter().map(|s| s.as_ref().trim().to_owned()).collect::<Vec<_>>().join(" ")
}
