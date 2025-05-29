pub fn code() {
    println!("Hello from dep_722");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_722");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_722: {t}");
}

pub fn foo() {
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
    dep_195::code();
    dep_195::code_inlined();
    dep_195::code_generic(1u32);
    dep_364::code();
    dep_364::code_inlined();
    dep_364::code_generic(1u32);
    dep_296::code();
    dep_296::code_inlined();
    dep_296::code_generic(1u32);
}
