#[doc = "Reader of register CMPCR"]
pub type R = crate::R<u32, super::CMPCR>;
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP_PD`"]
pub type CMP_PD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 0x01) != 0)
    }
}
