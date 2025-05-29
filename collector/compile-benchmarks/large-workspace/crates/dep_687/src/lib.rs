pub fn code() {
    println!("Hello from dep_687");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_687");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_687: {t}");
}

pub fn foo() {
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
    dep_296::code();
    dep_296::code_inlined();
    dep_296::code_generic(1u32);
    dep_377::code();
    dep_377::code_inlined();
    dep_377::code_generic(1u32);
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
}
