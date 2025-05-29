pub fn code() {
    println!("Hello from dep_676");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_676");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_676: {t}");
}

pub fn foo() {
    dep_368::code();
    dep_368::code_inlined();
    dep_368::code_generic(1u32);
    dep_202::code();
    dep_202::code_inlined();
    dep_202::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
}
