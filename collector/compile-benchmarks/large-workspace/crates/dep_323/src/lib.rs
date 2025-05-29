pub fn code() {
    println!("Hello from dep_323");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_323");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_323: {t}");
}

pub fn foo() {
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_105::code();
    dep_105::code_inlined();
    dep_105::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
}
