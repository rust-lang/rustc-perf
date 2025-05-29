pub fn code() {
    println!("Hello from dep_567");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_567");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_567: {t}");
}

pub fn foo() {
    dep_375::code();
    dep_375::code_inlined();
    dep_375::code_generic(1u32);
    dep_366::code();
    dep_366::code_inlined();
    dep_366::code_generic(1u32);
}
