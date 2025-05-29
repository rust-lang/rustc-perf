pub fn code() {
    println!("Hello from dep_841");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_841");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_841: {t}");
}

pub fn foo() {
    dep_371::code();
    dep_371::code_inlined();
    dep_371::code_generic(1u32);
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
    dep_397::code();
    dep_397::code_inlined();
    dep_397::code_generic(1u32);
}
