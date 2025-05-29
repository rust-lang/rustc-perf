pub fn code() {
    println!("Hello from dep_815");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_815");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_815: {t}");
}

pub fn foo() {
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
}
