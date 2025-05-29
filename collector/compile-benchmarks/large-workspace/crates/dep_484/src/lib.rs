pub fn code() {
    println!("Hello from dep_484");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_484");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_484: {t}");
}

pub fn foo() {
    dep_397::code();
    dep_397::code_inlined();
    dep_397::code_generic(1u32);
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_295::code();
    dep_295::code_inlined();
    dep_295::code_generic(1u32);
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
}
