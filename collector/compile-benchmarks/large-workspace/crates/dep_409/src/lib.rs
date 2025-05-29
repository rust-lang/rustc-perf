pub fn code() {
    println!("Hello from dep_409");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_409");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_409: {t}");
}

pub fn foo() {
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
}
