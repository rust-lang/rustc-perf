pub fn code() {
    println!("Hello from dep_368");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_368");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_368: {t}");
}

pub fn foo() {
    dep_155::code();
    dep_155::code_inlined();
    dep_155::code_generic(1u32);
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
    dep_147::code();
    dep_147::code_inlined();
    dep_147::code_generic(1u32);
}
