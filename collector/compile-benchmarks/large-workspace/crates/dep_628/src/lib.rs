pub fn code() {
    println!("Hello from dep_628");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_628");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_628: {t}");
}

pub fn foo() {
    dep_185::code();
    dep_185::code_inlined();
    dep_185::code_generic(1u32);
    dep_240::code();
    dep_240::code_inlined();
    dep_240::code_generic(1u32);
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
}
