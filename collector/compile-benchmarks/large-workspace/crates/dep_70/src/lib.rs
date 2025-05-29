pub fn code() {
    println!("Hello from dep_70");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_70");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_70: {t}");
}

pub fn foo() {
    dep_58::code();
    dep_58::code_inlined();
    dep_58::code_generic(1u32);
    dep_12::code();
    dep_12::code_inlined();
    dep_12::code_generic(1u32);
    dep_35::code();
    dep_35::code_inlined();
    dep_35::code_generic(1u32);
    dep_13::code();
    dep_13::code_inlined();
    dep_13::code_generic(1u32);
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
}
