pub mod mruby;
pub mod trampoline;

#[derive(Debug)]
pub struct String;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;
    use bstr::ByteSlice;

    const SUBJECT: &str = "String";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("string_test.rb");

    #[test]
    fn functional() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(FUNCTIONAL_TEST).unwrap();
        let result = interp.eval(b"spec");
        if let Err(exc) = result {
            let backtrace = exc.vm_backtrace(&mut interp);
            let backtrace = bstr::join("\n", backtrace.unwrap_or_default());
            panic!(
                "{} tests failed with message: {:?} and backtrace:\n{:?}",
                SUBJECT,
                exc.message().as_bstr(),
                backtrace.as_bstr()
            );
        }
    }
}
