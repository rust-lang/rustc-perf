#[doc = "Reader of register DOR2"]
pub type R = crate::R<u32, super::DOR2>;
#[doc = "Reader of field `DACC2DOR`"]
pub type DACC2DOR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 data output"]
    #[inline(always)]
    pub fn dacc2dor(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
