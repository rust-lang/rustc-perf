pub fn code() {
    println!("Hello from dep_80");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_80");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_80: {t}");
}

pub fn foo() {
    dep_38::code();
    dep_38::code_inlined();
    dep_38::code_generic(1u32);
}
