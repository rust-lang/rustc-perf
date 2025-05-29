pub fn code() {
    println!("Hello from dep_517");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_517");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_517: {t}");
}

pub fn foo() {
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
    dep_350::code();
    dep_350::code_inlined();
    dep_350::code_generic(1u32);
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
}
