pub fn code() {
    println!("Hello from dep_557");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_557");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_557: {t}");
}

pub fn foo() {
    dep_296::code();
    dep_296::code_inlined();
    dep_296::code_generic(1u32);
    dep_409::code();
    dep_409::code_inlined();
    dep_409::code_generic(1u32);
    dep_249::code();
    dep_249::code_inlined();
    dep_249::code_generic(1u32);
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
}
