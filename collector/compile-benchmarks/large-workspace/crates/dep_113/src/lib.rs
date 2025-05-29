pub fn code() {
    println!("Hello from dep_113");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_113");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_113: {t}");
}

pub fn foo() {
    dep_56::code();
    dep_56::code_inlined();
    dep_56::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
    dep_59::code();
    dep_59::code_inlined();
    dep_59::code_generic(1u32);
}
