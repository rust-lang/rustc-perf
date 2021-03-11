#[doc = "Reader of register DMACHTBAR"]
pub type R = crate::R<u32, super::DMACHTBAR>;
#[doc = "Reader of field `HTBAP`"]
pub type HTBAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HTBAP"]
    #[inline(always)]
    pub fn htbap(&self) -> HTBAP_R {
        HTBAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
