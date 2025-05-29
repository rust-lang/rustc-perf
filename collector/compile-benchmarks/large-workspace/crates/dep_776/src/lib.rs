pub fn code() {
    println!("Hello from dep_776");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_776");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_776: {t}");
}

pub fn foo() {
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
    dep_207::code();
    dep_207::code_inlined();
    dep_207::code_generic(1u32);
}
