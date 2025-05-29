pub fn code() {
    println!("Hello from dep_401");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_401");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_401: {t}");
}

pub fn foo() {
    dep_78::code();
    dep_78::code_inlined();
    dep_78::code_generic(1u32);
    dep_155::code();
    dep_155::code_inlined();
    dep_155::code_generic(1u32);
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
}
