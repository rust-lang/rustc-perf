pub fn code() {
    println!("Hello from dep_865");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_865");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_865: {t}");
}

pub fn foo() {
    dep_289::code();
    dep_289::code_inlined();
    dep_289::code_generic(1u32);
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
    dep_180::code();
    dep_180::code_inlined();
    dep_180::code_generic(1u32);
    dep_309::code();
    dep_309::code_inlined();
    dep_309::code_generic(1u32);
}
