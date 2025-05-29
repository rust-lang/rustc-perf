pub fn code() {
    println!("Hello from dep_527");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_527");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_527: {t}");
}

pub fn foo() {
    dep_246::code();
    dep_246::code_inlined();
    dep_246::code_generic(1u32);
    dep_208::code();
    dep_208::code_inlined();
    dep_208::code_generic(1u32);
    dep_363::code();
    dep_363::code_inlined();
    dep_363::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
}
