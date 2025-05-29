pub fn code() {
    println!("Hello from dep_578");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_578");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_578: {t}");
}

pub fn foo() {
    dep_167::code();
    dep_167::code_inlined();
    dep_167::code_generic(1u32);
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
}
