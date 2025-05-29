pub fn code() {
    println!("Hello from dep_86");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_86");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_86: {t}");
}

pub fn foo() {
    dep_56::code();
    dep_56::code_inlined();
    dep_56::code_generic(1u32);
    dep_20::code();
    dep_20::code_inlined();
    dep_20::code_generic(1u32);
    dep_33::code();
    dep_33::code_inlined();
    dep_33::code_generic(1u32);
    dep_32::code();
    dep_32::code_inlined();
    dep_32::code_generic(1u32);
}
