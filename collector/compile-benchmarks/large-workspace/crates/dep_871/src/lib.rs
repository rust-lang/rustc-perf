pub fn code() {
    println!("Hello from dep_871");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_871");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_871: {t}");
}

pub fn foo() {
    dep_178::code();
    dep_178::code_inlined();
    dep_178::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_215::code();
    dep_215::code_inlined();
    dep_215::code_generic(1u32);
}
