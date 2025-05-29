pub fn code() {
    println!("Hello from dep_192");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_192");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_192: {t}");
}

pub fn foo() {
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
}
