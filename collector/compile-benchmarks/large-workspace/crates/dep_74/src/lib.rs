pub fn code() {
    println!("Hello from dep_74");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_74");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_74: {t}");
}

pub fn foo() {
    dep_22::code();
    dep_22::code_inlined();
    dep_22::code_generic(1u32);
    dep_55::code();
    dep_55::code_inlined();
    dep_55::code_generic(1u32);
    dep_20::code();
    dep_20::code_inlined();
    dep_20::code_generic(1u32);
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
}
