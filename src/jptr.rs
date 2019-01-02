//! module jptr implements [JSON Pointer RFC spec].
//!
//! [JSON Pointer RFC spec]: https://tools.ietf.org/html/rfc6901

use json::Json;
use property;

/// quote path fragment using backslash escape and tilde escape defined by the
/// RFC spec.
///
/// After escaping each path-fragment caller can join them with '/'.
pub fn quote(fragment: &str) -> String {
    let mut outs = String::new();
    for ch in fragment.chars() {
        match ch {
            // backslash escape
            '"' | '\\' | '\x00'..'\x1f' => {
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
/// RFC spec.
///
/// After un-escaping each path-fragment caller can join them with '/'.
pub fn unquote(fragment: &str) -> Result<String, String> {
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
                _ => return Err(format!("jptr: invalid tilde escape {}", ch)),
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

// TODO: Can we fold lookup and g_lookup into a single implementation
// using macros or somehow using the type parameters ? The main
// different between the two is that lookup takes a mutable reference
// and g_lookup takes a immutable reference.

pub(crate) fn lookup<'a>(json: &'a mut Json, path: &str) -> Result<(&'a mut Json, String), String> {
    let mut frag = String::new();
    let (mut escaped, mut tilde) = (false, false);
    let mut chars = path.chars();
    loop {
        let ch = match chars.next() {
            Some(ch) => ch,
            None => break Ok((json, frag)),
        };
        if escaped {
            escaped = false;
            frag.push(ch);
            continue;
        } else if tilde {
            tilde = false;
            match ch {
                '0' => frag.push('~'),
                '1' => frag.push('/'),
                _ => return Err(format!("jptr: invalid tilde escape {}", ch)),
            }
            continue;
        } else if ch != '/' {
            match ch {
                '\\' => escaped = true, // backslash escape
                '~' => tilde = true,    // tilde escape
                _ => frag.push(ch),
            }
            continue;
        }
        break lookup(lookup_container(json, &frag)?, chars.as_str());
    }
}

pub(crate) fn lookup_container<'a>(json: &'a mut Json, frag: &str) -> Result<&'a mut Json, String> {
    match json {
        Json::Array(arr) => match frag.parse::<usize>() {
            Ok(n) if n >= arr.len() => Err(format!("jptr: index out of bound {}", n)),
            Ok(n) => Ok(&mut arr[n]),
            Err(err) => Err(format!("jptr: not array-index {}", err)),
        },
        Json::Object(props) => match property::search_by_key(props, &frag) {
            Ok(n) => Ok(props[n].value_mut()),
            Err(_) => Err(format!("jptr: key not found {}", frag)),
        },
        _ => Err(format!("jptr: not a container {} at {}", json, frag)),
    }
}

pub(crate) fn g_lookup<'a>(json: &'a Json, path: &str) -> Result<(&'a Json, String), String> {
    let mut frag = String::new();
    let (mut escaped, mut tilde) = (false, false);
    let mut chars = path.chars();
    loop {
        let ch = match chars.next() {
            Some(ch) => ch,
            None => break Ok((json, frag)),
        };
        if escaped {
            escaped = false;
            frag.push(ch);
            continue;
        } else if tilde {
            tilde = false;
            match ch {
                '0' => frag.push('~'),
                '1' => frag.push('/'),
                _ => return Err(format!("jptr: invalid tilde escape {}", ch)),
            }
            continue;
        } else if ch != '/' {
            match ch {
                '\\' => escaped = true, // backslash escape
                '~' => tilde = true,    // tilde escape
                _ => frag.push(ch),
            }
            continue;
        }
        break g_lookup(g_lookup_container(json, &frag)?, chars.as_str());
    }
}

pub(crate) fn g_lookup_container<'a>(json: &'a Json, frag: &str) -> Result<&'a Json, String> {
    match json {
        Json::Array(arr) => match frag.parse::<usize>() {
            Ok(n) if n >= arr.len() => Err(format!("jptr: index out of bound {}", n)),
            Ok(n) => Ok(&arr[n]),
            Err(err) => Err(format!("jptr: not array-index {}", err)),
        },
        Json::Object(props) => match property::search_by_key(props, &frag) {
            Ok(n) => Ok(props[n].value_ref()),
            Err(_) => Err(format!("jptr: key not found {}", frag)),
        },
        _ => Err(format!("jptr: not a container {} at {}", json, frag)),
    }
}

pub(crate) fn fix_prefix(path: &str) -> Result<&str, String> {
    let mut chars = path.chars();
    if chars.next().unwrap() != '/' {
        return Err("jptr: pointer should start with forward solidus".to_string());
    }
    Ok(chars.as_str())
}
