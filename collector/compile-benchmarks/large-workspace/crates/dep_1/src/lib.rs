pub fn code() {
    println!("Hello from dep_1");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_1");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_1: {t}");
}

