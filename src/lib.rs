#![forbid(unsafe_code)]

use lazy_static::lazy_static;
use napi_derive::napi;
use once_cell::sync::OnceCell;
use regex::Regex;

static STR_SPLITTERS: OnceCell<Vec<String>> = OnceCell::new();

#[inline(always)]
fn is_some_and_equal<T: PartialEq>(o: Option<T>, b: T) -> bool {
  if let Some(v) = o {
    v == b
  } else {
    false
  }
}

#[inline(always)]
fn is_uppercase(c: char) -> Option<bool> {
  lazy_static! {
    static ref NUMBER_CHAR_RE: Regex = Regex::new(r"[0-9]").unwrap();
  }
  if NUMBER_CHAR_RE.is_match(&c.to_string()) {
    return None;
  }
  Some(c.is_uppercase())
}

#[napi]
pub fn split_by_case(input: String, splitters: Option<Vec<String>>) -> Vec<String> {
  if input.len() == 0 {
    return vec![];
  }

  let mut parts: Vec<String> = vec![];
  let mut buff = "".to_string();

  let splitters = match splitters {
    None => STR_SPLITTERS
      .get_or_init(|| {
        vec![
          "-".to_string(),
          "_".to_string(),
          "/".to_string(),
          ".".to_string(),
        ]
      })
      .to_owned(),
    Some(val) => val,
  };

  let mut previous_upper = Option::<bool>::None;
  let mut previous_splitter = Option::<bool>::None;

  for c in input.chars() {
    let is_splitter = splitters.contains(&c.to_string());
    if is_splitter {
      parts.push(buff);
      buff = "".to_string();
      previous_upper = None;
      continue;
    }

    let is_upper = is_uppercase(c);
    if is_some_and_equal(previous_splitter, false) {
      if is_some_and_equal(previous_upper, false) && is_some_and_equal(is_upper, true) {
        parts.push(buff);
        buff = c.into();
        previous_upper = is_upper;
        continue;
      }

      if is_some_and_equal(previous_upper, true)
        && is_some_and_equal(is_upper, false)
        && buff.len() > 1
      {
        parts.push(buff[0..buff.len() - 1].into());
        buff = format!("{}{}", buff.chars().last().unwrap(), c);
        previous_upper = is_upper;
        continue;
      }
    }
    buff.push(c);
    previous_upper = is_upper;
    previous_splitter = Some(is_splitter);
  }

  parts.push(buff);

  parts
}

#[napi]
pub fn upper_first(mut input: String) -> String {
  if let Some(v) = input.get_mut(0..1) {
    v.make_ascii_uppercase();
  }

  input
}

#[napi]
pub fn lower_first(mut input: String) -> String {
  if let Some(v) = input.get_mut(0..1) {
    v.make_ascii_lowercase();
  }

  input
}

#[napi]
pub fn pascal_case(input: String) -> String {
  let mut parts: Vec<String> = vec![];

  for s in split_by_case(input, None) {
    parts.push(upper_first(s));
  }

  parts.join("")
}

#[napi]
pub fn camel_case(input: String) -> String {
  lower_first(pascal_case(input))
}

#[napi]
pub fn kebab_case(input: String, joiner: Option<String>) -> String {
  let joiner = match joiner {
    None => "-".into(),
    Some(v) => v,
  };

  split_by_case(input, None)
    .iter()
    .map(|v| v.to_lowercase())
    .collect::<Vec<String>>()
    .join(&joiner)
}

#[napi]
pub fn snake_case(input: String) -> String {
  kebab_case(input, Some("_".into()))
}
