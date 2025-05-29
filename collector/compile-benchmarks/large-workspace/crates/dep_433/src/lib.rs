pub fn code() {
    println!("Hello from dep_433");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_433");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_433: {t}");
}

pub fn foo() {
    dep_375::code();
    dep_375::code_inlined();
    dep_375::code_generic(1u32);
    dep_218::code();
    dep_218::code_inlined();
    dep_218::code_generic(1u32);
}
