#[doc = "Reader of register FIFOCNT"]
pub type R = crate::R<u32, super::FIFOCNT>;
#[doc = "Reader of field `FIFOCOUNT`"]
pub type FIFOCOUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO"]
    #[inline(always)]
    pub fn fifocount(&self) -> FIFOCOUNT_R {
        FIFOCOUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
