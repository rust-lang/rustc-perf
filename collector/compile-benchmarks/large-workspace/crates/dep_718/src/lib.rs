pub fn code() {
    println!("Hello from dep_718");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_718");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_718: {t}");
}

pub fn foo() {
    dep_253::code();
    dep_253::code_inlined();
    dep_253::code_generic(1u32);
    dep_200::code();
    dep_200::code_inlined();
    dep_200::code_generic(1u32);
    dep_312::code();
    dep_312::code_inlined();
    dep_312::code_generic(1u32);
    dep_272::code();
    dep_272::code_inlined();
    dep_272::code_generic(1u32);
}
