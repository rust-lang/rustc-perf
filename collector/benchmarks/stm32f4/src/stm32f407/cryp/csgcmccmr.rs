#[doc = "Reader of register CSGCMCCM%sR"]
pub type R = crate::R<u32, super::CSGCMCCMR>;
#[doc = "Writer for register CSGCMCCM%sR"]
pub type W = crate::W<u32, super::CSGCMCCMR>;
#[doc = "Register CSGCMCCM%sR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCMCCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSGCMCCM0R`"]
pub type CSGCMCCM0R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSGCMCCM0R`"]
pub struct CSGCMCCM0R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM0R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&self) -> CSGCMCCM0R_R {
        CSGCMCCM0R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&mut self) -> CSGCMCCM0R_W {
        CSGCMCCM0R_W { w: self }
    }
}
