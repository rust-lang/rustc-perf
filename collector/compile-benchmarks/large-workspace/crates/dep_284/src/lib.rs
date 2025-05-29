pub fn code() {
    println!("Hello from dep_284");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_284");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_284: {t}");
}

pub fn foo() {
    dep_60::code();
    dep_60::code_inlined();
    dep_60::code_generic(1u32);
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
    dep_127::code();
    dep_127::code_inlined();
    dep_127::code_generic(1u32);
}
