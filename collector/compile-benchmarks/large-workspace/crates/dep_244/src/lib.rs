pub fn code() {
    println!("Hello from dep_244");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_244");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_244: {t}");
}

pub fn foo() {
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_133::code();
    dep_133::code_inlined();
    dep_133::code_generic(1u32);
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
    dep_153::code();
    dep_153::code_inlined();
    dep_153::code_generic(1u32);
    dep_62::code();
    dep_62::code_inlined();
    dep_62::code_generic(1u32);
}
