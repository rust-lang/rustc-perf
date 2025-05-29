pub fn code() {
    println!("Hello from dep_542");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_542");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_542: {t}");
}

pub fn foo() {
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_250::code();
    dep_250::code_inlined();
    dep_250::code_generic(1u32);
}
