// FIXME: big_s crate should do this

#[allow(non_snake_case)]
#[inline]
pub fn S(s: impl ToString) -> String {
    s.to_string()
}
