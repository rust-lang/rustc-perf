pub fn code() {
    println!("Hello from dep_444");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_444");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_444: {t}");
}

pub fn foo() {
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
    dep_165::code();
    dep_165::code_inlined();
    dep_165::code_generic(1u32);
}
