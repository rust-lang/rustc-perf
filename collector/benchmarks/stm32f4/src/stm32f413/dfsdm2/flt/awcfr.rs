#[doc = "Reader of register AWCFR"]
pub type R = crate::R<u32, super::AWCFR>;
#[doc = "Writer for register AWCFR"]
pub type W = crate::W<u32, super::AWCFR>;
#[doc = "Register AWCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::AWCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRAWHTF`"]
pub type CLRAWHTF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLRAWHTF`"]
pub struct CLRAWHTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRAWHTF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLRAWLTF`"]
pub type CLRAWLTF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLRAWLTF`"]
pub struct CLRAWLTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRAWLTF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W {
        CLRAWHTF_W { w: self }
    }
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W {
        CLRAWLTF_W { w: self }
    }
}
