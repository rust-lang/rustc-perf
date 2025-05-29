pub fn code() {
    println!("Hello from dep_60");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_60");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_60: {t}");
}

pub fn foo() {
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_28::code();
    dep_28::code_inlined();
    dep_28::code_generic(1u32);
}
