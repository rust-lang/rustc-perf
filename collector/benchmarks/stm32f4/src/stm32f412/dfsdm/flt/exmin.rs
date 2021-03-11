#[doc = "Reader of register EXMIN"]
pub type R = crate::R<u32, super::EXMIN>;
#[doc = "Reader of field `EXMIN`"]
pub type EXMIN_R = crate::R<u32, u32>;
#[doc = "Reader of field `EXMINCH`"]
pub type EXMINCH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
}
