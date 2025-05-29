pub fn code() {
    println!("Hello from dep_639");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_639");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_639: {t}");
}

pub fn foo() {
    dep_337::code();
    dep_337::code_inlined();
    dep_337::code_generic(1u32);
    dep_249::code();
    dep_249::code_inlined();
    dep_249::code_generic(1u32);
    dep_375::code();
    dep_375::code_inlined();
    dep_375::code_generic(1u32);
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
}
