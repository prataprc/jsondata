// Copyright © 2019 R Pratap Chakravarthy. All rights reserved.

//! Module jptr implements [JSON Pointer RFC spec.].
//!
//! [JSON Pointer RFC spec.]: https://tools.ietf.org/html/rfc6901

use crate::{json::Json, ops, Error, Result};

/// quote path fragment using backslash escape and tilde escape defined by the
/// RFC specification.
///
/// After escaping each path-fragment caller can join them with '/'.
pub fn quote(fragment: &str) -> String {
    let mut outs = String::new();
    for ch in fragment.chars() {
        match ch {
            // backslash escape
            '"' | '\\' | '\x00'..='\x1f' => {
                outs.push('\\');
                outs.push(ch)
            }
            // tilde escape
            '~' => {
                outs.push('~');
                outs.push('0')
            }
            '/' => {
                outs.push('~');
                outs.push('1')
            }
            _ => outs.push(ch),
        }
    }
    outs
}

/// unquote path fragment for backslash and tilde escape defined by the
/// RFC specification.
///
/// After un-escaping each path-fragment caller can join them with '/'.
pub fn unquote(fragment: &str) -> Result<String> {
    let mut outs = String::new();
    let (mut escaped, mut tilde) = (false, false);
    for ch in fragment.chars() {
        if escaped {
            escaped = false;
            outs.push(ch);
            continue;
        } else if tilde {
            tilde = false;
            match ch {
                '0' => outs.push('~'),
                '1' => outs.push('/'),
                _ => err_at!(JptrFail, msg: "invalid ~{}", ch)?,
            }
            continue;
        }

        match ch {
            '\\' => escaped = true, // backslash escape
            '~' => tilde = true,    // tilde escape
            _ => outs.push(ch),
        }
    }
    Ok(outs)
}

pub(crate) fn fragments(path: &str) -> Result<(Vec<String>, String)> {
    let mut frags: Vec<String> = vec![];
    let mut frag = String::new();
    let mut state: (bool, bool) = (false, false); // (escaped, tilde)
    for ch in path.chars() {
        state = match ch {
            ch if state.0 => {
                frag.push(ch);
                (false, state.1)
            }
            '0' if state.1 => {
                frag.push('~');
                (state.0, false)
            }
            '1' if state.1 => {
                frag.push('/');
                (state.0, false)
            }
            ch if state.1 => err_at!(JptrFail, msg: "invalid ~{}", ch)?,
            '/' => {
                frags.push(frag.clone());
                frag.truncate(0);
                (state.0, state.1)
            }
            '\\' => (true, state.1),
            '~' => (state.0, true),
            ch => {
                frag.push(ch);
                (state.0, state.1)
            }
        };
    }
    Ok((frags, frag))
}

pub(crate) fn lookup_mut<'a>(
    mut json: &'a mut Json,
    path: &str,
) -> Result<(&'a mut Json, String)> {
    let (frags, key) = fragments(path)?;
    for frag in frags {
        json = ops::index_mut(json, frag.as_str())?
    }
    Ok((json, key))
}

pub(crate) fn lookup_ref<'a>(
    mut json_doc: &'a Json,
    path: &str,
) -> Result<(&'a Json, String)> {
    let (frags, key) = fragments(path)?;
    for frag in frags {
        json_doc = json_doc[frag.as_str()].to_result()?;
    }
    Ok((json_doc, key))
}

pub(crate) fn fix_prefix(path: &str) -> Result<&str> {
    let mut chars = path.chars();
    if chars.next().unwrap() == '/' {
        Ok(chars.as_str())
    } else {
        err_at!(JptrFail, msg: "pointer should start with forward solidus")
    }
}

#[cfg(test)]
#[path = "jptr_test.rs"]
mod jptr_test;
