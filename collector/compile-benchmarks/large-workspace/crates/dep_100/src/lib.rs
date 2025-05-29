pub fn code() {
    println!("Hello from dep_100");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_100");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_100: {t}");
}

pub fn foo() {
    dep_25::code();
    dep_25::code_inlined();
    dep_25::code_generic(1u32);
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_40::code();
    dep_40::code_inlined();
    dep_40::code_generic(1u32);
}
