pub fn code() {
    println!("Hello from dep_595");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_595");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_595: {t}");
}

pub fn foo() {
    dep_270::code();
    dep_270::code_inlined();
    dep_270::code_generic(1u32);
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
    dep_238::code();
    dep_238::code_inlined();
    dep_238::code_generic(1u32);
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
}
