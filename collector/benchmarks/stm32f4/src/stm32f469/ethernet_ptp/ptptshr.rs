#[doc = "Reader of register PTPTSHR"]
pub type R = crate::R<u32, super::PTPTSHR>;
#[doc = "Reader of field `STS`"]
pub type STS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - STS"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
