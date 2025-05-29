pub fn code() {
    println!("Hello from dep_738");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_738");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_738: {t}");
}

pub fn foo() {
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
    dep_360::code();
    dep_360::code_inlined();
    dep_360::code_generic(1u32);
}
