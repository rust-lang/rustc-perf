#[doc = "Reader of register DMACHTDR"]
pub type R = crate::R<u32, super::DMACHTDR>;
#[doc = "Reader of field `HTDAP`"]
pub type HTDAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit descriptor address pointer"]
    #[inline(always)]
    pub fn htdap(&self) -> HTDAP_R {
        HTDAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
