pub fn code() {
    println!("Hello from dep_308");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_308");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_308: {t}");
}

pub fn foo() {
    dep_73::code();
    dep_73::code_inlined();
    dep_73::code_generic(1u32);
    dep_101::code();
    dep_101::code_inlined();
    dep_101::code_generic(1u32);
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
}
