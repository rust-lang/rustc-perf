#[doc = "Reader of register MMCRFAECR"]
pub type R = crate::R<u32, super::MMCRFAECR>;
#[doc = "Reader of field `RFAEC`"]
pub type RFAEC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RFAEC"]
    #[inline(always)]
    pub fn rfaec(&self) -> RFAEC_R {
        RFAEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
