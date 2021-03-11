#[doc = "Reader of register EXMAX"]
pub type R = crate::R<u32, super::EXMAX>;
#[doc = "Reader of field `EXMAX`"]
pub type EXMAX_R = crate::R<u32, u32>;
#[doc = "Reader of field `EXMAXCH`"]
pub type EXMAXCH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:31 - Extremes detector maximum value"]
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - Extremes detector maximum data channel"]
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 0x07) as u8)
    }
}
