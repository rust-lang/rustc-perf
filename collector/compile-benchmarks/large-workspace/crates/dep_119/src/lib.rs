pub fn code() {
    println!("Hello from dep_119");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_119");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_119: {t}");
}

pub fn foo() {
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
}
