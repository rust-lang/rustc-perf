pub fn code() {
    println!("Hello from dep_164");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_164");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_164: {t}");
}

pub fn foo() {
    dep_134::code();
    dep_134::code_inlined();
    dep_134::code_generic(1u32);
    dep_100::code();
    dep_100::code_inlined();
    dep_100::code_generic(1u32);
    dep_88::code();
    dep_88::code_inlined();
    dep_88::code_generic(1u32);
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
}
