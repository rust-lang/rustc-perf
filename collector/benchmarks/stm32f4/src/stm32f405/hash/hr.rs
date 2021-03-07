#[doc = "Reader of register HR%s"]
pub type R = crate::R<u32, super::HR>;
#[doc = "Reader of field `H`"]
pub type H_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H0"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
