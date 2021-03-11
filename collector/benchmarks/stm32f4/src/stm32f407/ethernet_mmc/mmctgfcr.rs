#[doc = "Reader of register MMCTGFCR"]
pub type R = crate::R<u32, super::MMCTGFCR>;
#[doc = "Reader of field `TGFC`"]
pub type TGFC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HTL"]
    #[inline(always)]
    pub fn tgfc(&self) -> TGFC_R {
        TGFC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
