pub fn code() {
    println!("Hello from dep_133");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_133");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_133: {t}");
}

pub fn foo() {
    dep_38::code();
    dep_38::code_inlined();
    dep_38::code_generic(1u32);
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
}
