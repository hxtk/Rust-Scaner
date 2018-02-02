/// Copyright (c) Peter Sanders. All rights reserved.
/// Date: 2018-02-01
///
/// Unit tests for Rust implementation of Scanner.
use super::*;

#[test]
fn next_works_once_when_good_input() {
    let mut string: &[u8] = b"hello";
    let mut test: Scanner = Scanner::new(&mut string);

    if let Some(res) = test.next() {
        assert_eq!(&res[..], "hello");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_breaks_at_char_delim() {
    let mut string: &[u8] = b"hello, world";
    let mut test: Scanner = Scanner::new(&mut string);

    if let Some(res) = test.next() {
        assert_eq!(&res[..], "hello,");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_skips_leading_delims() {
    let mut string: &[u8] = b"hello,  world";
    let mut test: Scanner = Scanner::new(&mut string);
    test.next();

    if let Some(res) = test.next() {
        assert_eq!(&res[..], "world");
    } else {
        assert_eq!(true, false);
    }
}

/// When this test was written, the first delimiter character after
/// the string read by `Scanner.next()` would be consumed, which affected
/// the result of the next data operation if that operation used a different
/// delimiter.
#[test]
fn next_preserves_trailing_delim() {
    let mut string: &[u8] = b"hello,  world";
    let mut test: Scanner = Scanner::new(&mut string);

    test.next();
    if let Some(res) = test.next_line() {
        assert_eq!(&res[..], "  world");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_handles_line_wrap() {
    let mut string: &[u8] = b"hello\nworld";
    let mut test = Scanner::new(&mut string);

    if let Some(res) = test.next() {
        assert_eq!(&res[..], "hello");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_line_reads_whole_line() {
    let mut string: &[u8] = b"hello,  world\ngoodbye, world";
    let mut test: Scanner = Scanner::new(&mut string);

    if let Some(res) = test.next_line() {
        assert_eq!(&res[..], "hello,  world");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_line_reads_last_line() {
    let mut string: &[u8] = b"foo bar baz";
    let mut test: Scanner = Scanner::new(&mut string);

    if let Some(res) = test.next_line() {
        assert_eq!(&res[..], "foo bar baz");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_works_after_next_line() {
    let mut string: &[u8] = b"hello,  world\ngoodbye, world";
    let mut test: Scanner = Scanner::new(&mut string);
    test.next_line();

    if let Some(res) = test.next() {
        assert_eq!(&res[..], "goodbye,");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_int_handles_commas() {
    let mut string: &[u8] = b"2,147,483,647";
    let mut test: Scanner = Scanner::new(&mut string);

    assert_eq!(test.next_int::<i32>(), Some(2147483647));
}

#[test]
fn next_int_none_on_positive_overflow() {
    let mut string: &[u8] = b"2147483648";
    let mut test: Scanner = Scanner::new(&mut string);

    let res = test.next_int::<i32>();
    assert_eq!(res, None);
}

#[test]
fn next_i32_none_on_negative_overflow() {
    let mut string: &[u8] = b"-2147483649";
    let mut test: Scanner = Scanner::new(&mut string);

    let res = test.next_int::<i32>();
    assert_eq!(res, None);
}

#[test]
fn arbitrary_delim() {
    let mut string: &[u8] = b"foohello, worldfoo";
    let mut test: Scanner = Scanner::new(&mut string);
    test.set_delim(Regex::new(r"foo").unwrap());

    if let Some(res) = test.next() {
        assert_eq!(&res[..], "hello, world");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn next_float() {
    let mut string: &[u8] = b"2.5";
    let mut test: Scanner = Scanner::new(&mut string);

    assert_eq!(test.next_float::<f64>(), Some(2.5));
}

#[test]
fn next_int_custom_radix() {
    let mut string: &[u8] = b"11010";
    let mut test = Scanner::new(&mut string);

    // invalid radix should return None and not consume `Scanner.next()`
    assert_eq!(test.next_int_radix::<i32>(1), None);

    // 2 is a valid radix.
    assert_eq!(test.next_int_radix(2), Some(26));
}

#[test]
fn next_float_base_2() {
    let mut string: &[u8] = b"11010.1";
    let mut test = Scanner::new(&mut string);

    // invalid radix should return None and not consume `Scanner.next()`
    assert_eq!(test.next_float_radix::<f64>(1), None);

    // 2 is a valid radix.
    assert_eq!(test.next_float_radix(2), Some(26.5));
}
    
#[test]
fn str_delim_escapes_regexes() {
    let mut string: &[u8] = b"foo[a-z]+bar";
    let mut test: Scanner = Scanner::new(&mut string);
    test.set_delim_str("[a-z]+");

    test.next();
    if let Some(res) = test.next() {
        assert_eq!(&res[..], "bar");
    } else {
        assert_eq!(true, false);
    }
}

#[test]
fn radix_between_2_36() {
    let mut string: &[u8] = b"";
    let mut test = Scanner::new(&mut string);

    assert_eq!(test.get_radix(), 10);
    test.set_radix(1);
    assert_eq!(test.get_radix(), 10);
    test.set_radix(37);
    assert_eq!(test.get_radix(), 10);
    test.set_radix(36);
    assert_eq!(test.get_radix(), 36);
}
