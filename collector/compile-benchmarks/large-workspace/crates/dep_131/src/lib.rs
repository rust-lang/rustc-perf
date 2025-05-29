pub fn code() {
    println!("Hello from dep_131");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_131");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_131: {t}");
}

pub fn foo() {
    dep_59::code();
    dep_59::code_inlined();
    dep_59::code_generic(1u32);
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
    dep_38::code();
    dep_38::code_inlined();
    dep_38::code_generic(1u32);
}
