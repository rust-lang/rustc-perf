pub fn code() {
    println!("Hello from dep_89");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_89");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_89: {t}");
}

pub fn foo() {
    dep_45::code();
    dep_45::code_inlined();
    dep_45::code_generic(1u32);
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
    dep_16::code();
    dep_16::code_inlined();
    dep_16::code_generic(1u32);
    dep_41::code();
    dep_41::code_inlined();
    dep_41::code_generic(1u32);
}
