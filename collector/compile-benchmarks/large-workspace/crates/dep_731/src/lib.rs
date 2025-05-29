pub fn code() {
    println!("Hello from dep_731");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_731");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_731: {t}");
}

pub fn foo() {
    dep_312::code();
    dep_312::code_inlined();
    dep_312::code_generic(1u32);
}
