#[doc = "Reader of register MMCTGFMSCCR"]
pub type R = crate::R<u32, super::MMCTGFMSCCR>;
#[doc = "Reader of field `TGFMSCC`"]
pub type TGFMSCC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TGFMSCC"]
    #[inline(always)]
    pub fn tgfmscc(&self) -> TGFMSCC_R {
        TGFMSCC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
