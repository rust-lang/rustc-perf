pub fn code() {
    println!("Hello from dep_644");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_644");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_644: {t}");
}

pub fn foo() {
    dep_359::code();
    dep_359::code_inlined();
    dep_359::code_generic(1u32);
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
    dep_205::code();
    dep_205::code_inlined();
    dep_205::code_generic(1u32);
}
