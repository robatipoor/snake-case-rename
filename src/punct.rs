use lazy_static::lazy_static;
use regex::Regex;

use crate::result::Result;
use std::collections::VecDeque;
use std::{fs, fs::DirEntry};

lazy_static! {
    pub static ref PUNCT: Regex = Regex::new(r#"[[:punct:]]"#).unwrap();
}

pub fn replace_punct_with_underscore(name: &str) -> String {
    let mut back = VecDeque::<char>::new();
    let mut front = VecDeque::<char>::new();
    let mut front_flag = false;
    let mut back_flag = false;
    let mut double_back_flag = false;
    let mut double_front_flag = false;
    let len = name.len();
    let mut count = len - 1;
    let mid = len / 2;
    for i in 0..mid {
        let b = name.as_bytes()[i] as char;
        let f = name.as_bytes()[count] as char;
        if PUNCT.is_match(&b.to_string()) || b == ' ' {
            if back_flag && !double_back_flag {
                back.push_back('_');
                double_back_flag = true;
            }
        } else {
            back.push_back(b);
            back_flag = true;
            double_back_flag = false;
        }
        if PUNCT.is_match(&f.to_string()) || f == ' ' {
            if front_flag && !double_front_flag {
                front.push_front('_');
                double_front_flag = true;
            }
        } else {
            front.push_front(f);
            front_flag = true;
            double_front_flag = false;
        }
        count -= 1;
    }
    if len % 2 != 0 {
        let f = name.as_bytes()[mid] as char;
        if PUNCT.is_match(&f.to_string()) || f == ' ' {
            back.push_back('_');
        } else {
            back.push_back(f);
        }
    }
    let mut out = back.iter().collect::<String>();
    out.push_str(&front.iter().collect::<String>());
    out
}

pub fn read_current_dir() -> Result<Vec<DirEntry>> {
    let files = fs::read_dir(".")?;
    Ok(files.into_iter().map(|f| f.unwrap()).collect())
}

pub fn snake_case_convert(s: String) -> String {
    let mut buf = Vec::<char>::new();
    let mut cc = 0;
    for (i, c) in s.chars().into_iter().enumerate() {
        if c.is_uppercase() && i > 0 {
            if let Some(s) = buf.get(i - 1 + cc) {
                if s.is_lowercase() && *s != '_' {
                    buf.push('_');
                    cc += 1;
                }
            }
        }
        buf.push(c);
    }
    buf.iter().collect::<String>().to_lowercase()
}

#[cfg(test)]
pub mod tests {
    use crate::punct::*;
    #[test]
    pub fn replace_punct_with_underscore_test_1() {
        let name_input = "book-eng$algo";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_2() {
        let name_input = "book-eng--- algo";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_3() {
        let name_input = "book-!eng!algo";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_4() {
        let name_input = "book())eng_algo";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_5() {
        let name_input = "book_eng_algo";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_6() {
        let name_input = " @!book@__eng_algo@!";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_7() {
        let name_input = "book_eng_algo!@";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_8() {
        let name_input = "book_eng_algo*&^";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_9() {
        let name_input = "book_eng_algo   *&";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_10() {
        let name_input = "  book_eng_algo  -";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("book_eng_algo", name_output);
    }

    #[test]
    pub fn replace_punct_with_underscore_test_11() {
        let name_input = "foo$bar";
        let name_output = replace_punct_with_underscore(name_input);
        assert_eq!("foo_bar", name_output);
    }

    #[test]
    pub fn snake_case_test1() {
        let name_input = "fooBar".to_string();
        let name_output = snake_case_convert(name_input);
        assert_eq!("foo_bar", name_output);
    }

    #[test]
    pub fn snake_case_test2() {
        let name_input = "fooBarBAZR".to_string();
        let name_output = snake_case_convert(name_input);
        assert_eq!("foo_bar_bazr", name_output);
    }

    #[test]
    pub fn snake_case_test3() {
        let name_input = "fooBAR".to_string();
        let name_output = snake_case_convert(name_input);
        assert_eq!("foo_bar", name_output);
    }

    #[test]
    pub fn snake_case_test4() {
        let name_input = "fooBar".to_string();
        let name_output = snake_case_convert(name_input);
        assert_eq!("foo_bar", name_output);
    }

    #[test]
    pub fn snake_case_test() {
        let name_input = "fooBAr".to_string();
        let name_output = snake_case_convert(name_input);
        assert_eq!("foo_bar", name_output);
    }
}
