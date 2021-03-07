#[doc = "Reader of register HAINT"]
pub type R = crate::R<u32, super::HAINT>;
#[doc = "Reader of field `HAINT`"]
pub type HAINT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
