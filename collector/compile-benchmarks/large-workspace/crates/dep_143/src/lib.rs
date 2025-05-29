pub fn code() {
    println!("Hello from dep_143");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_143");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_143: {t}");
}

pub fn foo() {
    dep_40::code();
    dep_40::code_inlined();
    dep_40::code_generic(1u32);
    dep_24::code();
    dep_24::code_inlined();
    dep_24::code_generic(1u32);
    dep_46::code();
    dep_46::code_inlined();
    dep_46::code_generic(1u32);
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
}
