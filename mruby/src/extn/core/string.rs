use byteorder::{NativeEndian, ReadBytesExt};
use std::io::Cursor;

use crate::convert::{FromMrb, TryFromMrb};
use crate::def::{ClassLike, Define};
use crate::eval::MrbEval;
use crate::extn::core::error::{ArgumentError, RubyException};
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::Value;
use crate::MrbError;
use log::trace;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    if interp.borrow().class_spec::<RString>().is_some() {
        return Ok(());
    }
    let string = interp
        .borrow_mut()
        .def_class::<RString>("String", None, None);
    interp.eval(include_str!("string.rb"))?;
    string
        .borrow_mut()
        .add_method("ord", RString::ord, sys::mrb_args_none());
    string.borrow().define(interp).map_err(|_| MrbError::New)?;
    trace!("Patched String onto interpreter");
    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub struct RString;

impl RString {
    unsafe extern "C" fn ord(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let s = unwrap_or_raise!(
            interp,
            String::try_from_mrb(&interp, Value::new(&interp, slf)),
            interp.nil().inner()
        );
        if let Some(first) = s.chars().next() {
            // One UTF-8 character, which are at most 32 bits.
            let mut buf = [0; 4];
            first.encode_utf8(&mut buf);
            let mut reader = Cursor::new(buf);
            if let Ok(ord) = reader.read_u32::<NativeEndian>() {
                Value::from_mrb(&interp, ord).inner()
            } else {
                interp.nil().inner()
            }
        } else {
            ArgumentError::raise(&interp, "empty string")
        }
    }
}
// Tests from String core docs in Ruby 2.6.3
// https://ruby-doc.org/core-2.6.3/String.html
#[cfg(test)]
mod tests {
    use crate::convert::FromMrb;
    use crate::eval::MrbEval;
    use crate::extn::core::string;
    use crate::interpreter::Interpreter;
    use crate::value::{Value, ValueLike};

    #[test]
    fn string_equal_squiggle() {
        let interp = Interpreter::create().expect("mrb init");
        string::patch(&interp).expect("string init");

        let value = interp.eval(r#""cat o' 9 tails" =~ /\d/"#).unwrap();
        assert_eq!(value.try_into::<Option<i64>>(), Ok(Some(7)));
        let value = interp.eval(r#""cat o' 9 tails" =~ 9"#).unwrap();
        assert_eq!(value.try_into::<Option<i64>>(), Ok(None));
    }

    #[test]
    fn string_idx() {
        let interp = Interpreter::create().expect("mrb init");
        string::patch(&interp).expect("string init");

        assert_eq!(
            &interp
                .eval(r"'hello there'[/[aeiou](.)\1/]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "ell"
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/[aeiou](.)\1/, 0]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "ell"
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/[aeiou](.)\1/, 1]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "l"
        );
        assert_eq!(
            interp
                .eval(r"'hello there'[/[aeiou](.)\1/, 2]")
                .unwrap()
                .try_into::<Option<String>>()
                .unwrap(),
            None
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'non_vowel']")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "l"
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'vowel']")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "e"
        );
    }

    #[test]
    fn string_scan() {
        let interp = Interpreter::create().expect("mrb init");
        string::patch(&interp).expect("string init");

        let s = Value::from_mrb(&interp, "abababa");
        let result = s
            .funcall::<Vec<String>, _, _>("scan", &[interp.eval("/./").expect("eval")])
            .expect("funcall");
        assert_eq!(
            result,
            vec!["a", "b", "a", "b", "a", "b", "a"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        );
        let result = s
            .funcall::<Vec<String>, _, _>("scan", &[interp.eval("/../").expect("eval")])
            .expect("funcall");
        assert_eq!(
            result,
            vec!["ab", "ab", "ab"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        );
        let result = s
            .funcall::<Vec<String>, _, _>("scan", &[interp.eval("'aba'").expect("eval")])
            .expect("funcall");
        assert_eq!(
            result,
            vec!["aba", "aba"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        );
        let result = s
            .funcall::<Vec<String>, _, _>("scan", &[interp.eval("'no no no'").expect("eval")])
            .expect("funcall");
        assert_eq!(result, <Vec<String>>::new());
    }

    #[test]
    fn string_unary_minus() {
        let interp = Interpreter::create().expect("mrb init");
        string::patch(&interp).expect("string init");

        let s = interp.eval("-'abababa'").expect("eval");
        let result = s.funcall::<bool, _, _>("frozen?", &[]).expect("funcall");
        assert!(result);
        let result = s.funcall::<String, _, _>("itself", &[]).expect("funcall");
        assert_eq!(result, "abababa");
    }
}
