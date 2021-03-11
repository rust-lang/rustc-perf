#[doc = "Reader of register MMCRFCECR"]
pub type R = crate::R<u32, super::MMCRFCECR>;
#[doc = "Reader of field `RFCFC`"]
pub type RFCFC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RFCFC"]
    #[inline(always)]
    pub fn rfcfc(&self) -> RFCFC_R {
        RFCFC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
