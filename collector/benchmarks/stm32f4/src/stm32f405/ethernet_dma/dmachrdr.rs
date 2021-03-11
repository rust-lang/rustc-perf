#[doc = "Reader of register DMACHRDR"]
pub type R = crate::R<u32, super::DMACHRDR>;
#[doc = "Reader of field `HRDAP`"]
pub type HRDAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HRDAP"]
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
