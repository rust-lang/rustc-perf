pub fn code() {
    println!("Hello from dep_497");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_497");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_497: {t}");
}

pub fn foo() {
    dep_246::code();
    dep_246::code_inlined();
    dep_246::code_generic(1u32);
}
