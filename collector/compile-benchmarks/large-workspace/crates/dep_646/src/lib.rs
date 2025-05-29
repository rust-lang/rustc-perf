pub fn code() {
    println!("Hello from dep_646");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_646");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_646: {t}");
}

pub fn foo() {
    dep_206::code();
    dep_206::code_inlined();
    dep_206::code_generic(1u32);
    dep_296::code();
    dep_296::code_inlined();
    dep_296::code_generic(1u32);
}
