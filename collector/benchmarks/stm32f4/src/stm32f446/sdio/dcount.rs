#[doc = "Reader of register DCOUNT"]
pub type R = crate::R<u32, super::DCOUNT>;
#[doc = "Reader of field `DATACOUNT`"]
pub type DATACOUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
