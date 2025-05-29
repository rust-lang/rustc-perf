pub fn code() {
    println!("Hello from dep_108");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_108");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_108: {t}");
}

pub fn foo() {
    dep_25::code();
    dep_25::code_inlined();
    dep_25::code_generic(1u32);
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
    dep_35::code();
    dep_35::code_inlined();
    dep_35::code_generic(1u32);
}
