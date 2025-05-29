pub fn code() {
    println!("Hello from dep_53");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_53");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_53: {t}");
}

pub fn foo() {
    dep_9::code();
    dep_9::code_inlined();
    dep_9::code_generic(1u32);
    dep_3::code();
    dep_3::code_inlined();
    dep_3::code_generic(1u32);
    dep_8::code();
    dep_8::code_inlined();
    dep_8::code_generic(1u32);
    dep_6::code();
    dep_6::code_inlined();
    dep_6::code_generic(1u32);
    dep_4::code();
    dep_4::code_inlined();
    dep_4::code_generic(1u32);
}
