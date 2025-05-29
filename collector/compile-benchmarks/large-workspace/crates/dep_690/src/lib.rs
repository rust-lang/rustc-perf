pub fn code() {
    println!("Hello from dep_690");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_690");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_690: {t}");
}

pub fn foo() {
    dep_374::code();
    dep_374::code_inlined();
    dep_374::code_generic(1u32);
}
