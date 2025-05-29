pub fn code() {
    println!("Hello from dep_720");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_720");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_720: {t}");
}

pub fn foo() {
    dep_249::code();
    dep_249::code_inlined();
    dep_249::code_generic(1u32);
    dep_185::code();
    dep_185::code_inlined();
    dep_185::code_generic(1u32);
}
