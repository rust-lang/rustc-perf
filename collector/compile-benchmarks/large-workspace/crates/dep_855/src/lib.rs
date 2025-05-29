pub fn code() {
    println!("Hello from dep_855");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_855");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_855: {t}");
}

pub fn foo() {
    dep_371::code();
    dep_371::code_inlined();
    dep_371::code_generic(1u32);
    dep_218::code();
    dep_218::code_inlined();
    dep_218::code_generic(1u32);
}
