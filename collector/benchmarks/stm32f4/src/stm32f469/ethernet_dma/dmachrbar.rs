#[doc = "Reader of register DMACHRBAR"]
pub type R = crate::R<u32, super::DMACHRBAR>;
#[doc = "Reader of field `HRBAP`"]
pub type HRBAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive buffer address pointer"]
    #[inline(always)]
    pub fn hrbap(&self) -> HRBAP_R {
        HRBAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
