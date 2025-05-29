pub fn code() {
    println!("Hello from dep_766");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_766");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_766: {t}");
}

pub fn foo() {
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_181::code();
    dep_181::code_inlined();
    dep_181::code_generic(1u32);
    dep_195::code();
    dep_195::code_inlined();
    dep_195::code_generic(1u32);
    dep_165::code();
    dep_165::code_inlined();
    dep_165::code_generic(1u32);
}
