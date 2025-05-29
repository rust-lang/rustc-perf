pub fn code() {
    println!("Hello from dep_157");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_157");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_157: {t}");
}

pub fn foo() {
    dep_53::code();
    dep_53::code_inlined();
    dep_53::code_generic(1u32);
    dep_36::code();
    dep_36::code_inlined();
    dep_36::code_generic(1u32);
    dep_33::code();
    dep_33::code_inlined();
    dep_33::code_generic(1u32);
    dep_38::code();
    dep_38::code_inlined();
    dep_38::code_generic(1u32);
}
