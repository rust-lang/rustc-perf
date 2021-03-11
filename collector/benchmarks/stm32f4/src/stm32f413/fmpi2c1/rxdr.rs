#[doc = "Reader of register RXDR"]
pub type R = crate::R<u32, super::RXDR>;
#[doc = "Reader of field `RXDATA`"]
pub type RXDATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RXDATA"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
