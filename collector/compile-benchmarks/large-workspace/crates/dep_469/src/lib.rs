pub fn code() {
    println!("Hello from dep_469");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_469");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_469: {t}");
}

pub fn foo() {
    dep_248::code();
    dep_248::code_inlined();
    dep_248::code_generic(1u32);
    dep_175::code();
    dep_175::code_inlined();
    dep_175::code_generic(1u32);
    dep_289::code();
    dep_289::code_inlined();
    dep_289::code_generic(1u32);
    dep_346::code();
    dep_346::code_inlined();
    dep_346::code_generic(1u32);
    dep_310::code();
    dep_310::code_inlined();
    dep_310::code_generic(1u32);
}
