#[doc = "Reader of register CEC_RXDR"]
pub type R = crate::R<u32, super::CEC_RXDR>;
#[doc = "Reader of field `RXD`"]
pub type RXD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Rx Data register"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
