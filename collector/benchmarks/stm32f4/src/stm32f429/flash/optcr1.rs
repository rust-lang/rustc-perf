#[doc = "Reader of register OPTCR1"]
pub type R = crate::R<u32, super::OPTCR1>;
#[doc = "Writer for register OPTCR1"]
pub type W = crate::W<u32, super::OPTCR1>;
#[doc = "Register OPTCR1 `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::OPTCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `nWRP`"]
pub type NWRP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `nWRP`"]
pub struct NWRP_W<'a> {
    w: &'a mut W,
}
impl<'a> NWRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&self) -> NWRP_R {
        NWRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&mut self) -> NWRP_W {
        NWRP_W { w: self }
    }
}
