pub fn code() {
    println!("Hello from dep_572");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_572");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_572: {t}");
}

pub fn foo() {
    dep_374::code();
    dep_374::code_inlined();
    dep_374::code_generic(1u32);
    dep_263::code();
    dep_263::code_inlined();
    dep_263::code_generic(1u32);
    dep_262::code();
    dep_262::code_inlined();
    dep_262::code_generic(1u32);
    dep_275::code();
    dep_275::code_inlined();
    dep_275::code_generic(1u32);
    dep_348::code();
    dep_348::code_inlined();
    dep_348::code_generic(1u32);
}
