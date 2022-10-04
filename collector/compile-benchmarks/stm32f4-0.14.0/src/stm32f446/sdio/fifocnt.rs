#[doc = "Register `FIFOCNT` reader"]
pub struct R(crate::R<FIFOCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOCOUNT` reader - Remaining number of words to be written to or read from the FIFO"]
pub struct FIFOCOUNT_R(crate::FieldReader<u32, u32>);
impl FIFOCOUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        FIFOCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOCOUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO"]
    #[inline(always)]
    pub fn fifocount(&self) -> FIFOCOUNT_R {
        FIFOCOUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocnt](index.html) module"]
pub struct FIFOCNT_SPEC;
impl crate::RegisterSpec for FIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifocnt::R](R) reader structure"]
impl crate::Readable for FIFOCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFOCNT to value 0"]
impl crate::Resettable for FIFOCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
