pub fn code() {
    println!("Hello from dep_546");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_546");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_546: {t}");
}

pub fn foo() {
    dep_205::code();
    dep_205::code_inlined();
    dep_205::code_generic(1u32);
    dep_188::code();
    dep_188::code_inlined();
    dep_188::code_generic(1u32);
}
