pub fn code() {
    println!("Hello from dep_103");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_103");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_103: {t}");
}

pub fn foo() {
    dep_57::code();
    dep_57::code_inlined();
    dep_57::code_generic(1u32);
    dep_20::code();
    dep_20::code_inlined();
    dep_20::code_generic(1u32);
    dep_47::code();
    dep_47::code_inlined();
    dep_47::code_generic(1u32);
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
}
