#![allow(dead_code)]

// Reduced from the tuple SqlType impls in diesel 2.2.10.
//
// Without the param-env normalization behavior from rust-lang/rust#158643,
// the nested associated-type projections in this impl are much more expensive
// to normalize.

trait Combine<Other> {
    type Out;
}

trait HasAssoc {
    type Assoc;
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> HasAssoc for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: HasAssoc,
    T1: HasAssoc,
    T2: HasAssoc,
    T3: HasAssoc,
    T4: HasAssoc,
    T5: HasAssoc,
    T6: HasAssoc,
    T7: HasAssoc,
    T8: HasAssoc,
    T9: HasAssoc,
    T10: HasAssoc,
    T11: HasAssoc,
    T12: HasAssoc,
    T13: HasAssoc,
    T14: HasAssoc,
    T15: HasAssoc,
    T16: HasAssoc,
    T17: HasAssoc,
    T0::Assoc: Combine<T1::Assoc>,
    <T0::Assoc as Combine<T1::Assoc>>::Out: Combine<T2::Assoc>,
    <<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out: Combine<T3::Assoc>,
    <<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out: Combine<T4::Assoc>,
    <<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out: Combine<T5::Assoc>,
    <<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out: Combine<T6::Assoc>,
    <<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out: Combine<T7::Assoc>,
    <<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out: Combine<T8::Assoc>,
    <<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out: Combine<T9::Assoc>,
    <<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out: Combine<T10::Assoc>,
    <<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out: Combine<T11::Assoc>,
    <<<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out as Combine<T11::Assoc>>::Out: Combine<T12::Assoc>,
    <<<<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out as Combine<T11::Assoc>>::Out as Combine<T12::Assoc>>::Out: Combine<T13::Assoc>,
    <<<<<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out as Combine<T11::Assoc>>::Out as Combine<T12::Assoc>>::Out as Combine<T13::Assoc>>::Out: Combine<T14::Assoc>,
    <<<<<<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out as Combine<T11::Assoc>>::Out as Combine<T12::Assoc>>::Out as Combine<T13::Assoc>>::Out as Combine<T14::Assoc>>::Out: Combine<T15::Assoc>,
    <<<<<<<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out as Combine<T11::Assoc>>::Out as Combine<T12::Assoc>>::Out as Combine<T13::Assoc>>::Out as Combine<T14::Assoc>>::Out as Combine<T15::Assoc>>::Out: Combine<T16::Assoc>,
    <<<<<<<<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out as Combine<T11::Assoc>>::Out as Combine<T12::Assoc>>::Out as Combine<T13::Assoc>>::Out as Combine<T14::Assoc>>::Out as Combine<T15::Assoc>>::Out as Combine<T16::Assoc>>::Out: Combine<T17::Assoc>,
{
    type Assoc = <<<<<<<<<<<<<<<<<T0::Assoc as Combine<T1::Assoc>>::Out as Combine<T2::Assoc>>::Out as Combine<T3::Assoc>>::Out as Combine<T4::Assoc>>::Out as Combine<T5::Assoc>>::Out as Combine<T6::Assoc>>::Out as Combine<T7::Assoc>>::Out as Combine<T8::Assoc>>::Out as Combine<T9::Assoc>>::Out as Combine<T10::Assoc>>::Out as Combine<T11::Assoc>>::Out as Combine<T12::Assoc>>::Out as Combine<T13::Assoc>>::Out as Combine<T14::Assoc>>::Out as Combine<T15::Assoc>>::Out as Combine<T16::Assoc>>::Out as Combine<T17::Assoc>>::Out;
}
