pub fn code() {
    println!("Hello from dep_491");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_491");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_491: {t}");
}

pub fn foo() {
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_346::code();
    dep_346::code_inlined();
    dep_346::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_213::code();
    dep_213::code_inlined();
    dep_213::code_generic(1u32);
}
