pub fn code() {
    println!("Hello from dep_421");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_421");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_421: {t}");
}

pub fn foo() {
    dep_409::code();
    dep_409::code_inlined();
    dep_409::code_generic(1u32);
    dep_390::code();
    dep_390::code_inlined();
    dep_390::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
    dep_337::code();
    dep_337::code_inlined();
    dep_337::code_generic(1u32);
}
