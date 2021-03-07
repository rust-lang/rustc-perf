#[doc = "Reader of register DIR"]
pub type R = crate::R<u32, super::DIR>;
#[doc = "Reader of field `THI`"]
pub type THI_R = crate::R<u16, u16>;
#[doc = "Reader of field `TLO`"]
pub type TLO_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - Threshold HIGH"]
    #[inline(always)]
    pub fn thi(&self) -> THI_R {
        THI_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Threshold LOW"]
    #[inline(always)]
    pub fn tlo(&self) -> TLO_R {
        TLO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
