pub fn code() {
    println!("Hello from dep_539");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_539");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_539: {t}");
}

pub fn foo() {
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
    dep_249::code();
    dep_249::code_inlined();
    dep_249::code_generic(1u32);
}
