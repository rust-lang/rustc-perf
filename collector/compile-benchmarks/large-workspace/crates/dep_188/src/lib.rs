pub fn code() {
    println!("Hello from dep_188");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_188");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_188: {t}");
}

pub fn foo() {
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
    dep_97::code();
    dep_97::code_inlined();
    dep_97::code_generic(1u32);
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
    dep_101::code();
    dep_101::code_inlined();
    dep_101::code_generic(1u32);
    dep_78::code();
    dep_78::code_inlined();
    dep_78::code_generic(1u32);
}
