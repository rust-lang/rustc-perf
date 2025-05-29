pub fn code() {
    println!("Hello from dep_649");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_649");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_649: {t}");
}

pub fn foo() {
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
    dep_341::code();
    dep_341::code_inlined();
    dep_341::code_generic(1u32);
    dep_387::code();
    dep_387::code_inlined();
    dep_387::code_generic(1u32);
    dep_275::code();
    dep_275::code_inlined();
    dep_275::code_generic(1u32);
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
}
