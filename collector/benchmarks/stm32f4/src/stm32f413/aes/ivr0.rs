#[doc = "Reader of register IVR0"]
pub type R = crate::R<u32, super::IVR0>;
#[doc = "Writer for register IVR0"]
pub type W = crate::W<u32, super::IVR0>;
#[doc = "Register IVR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IVR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_IVR0`"]
pub type AES_IVR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_IVR0`"]
pub struct AES_IVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_IVR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - initialization vector register (LSB IVR \\[31:0\\])"]
    #[inline(always)]
    pub fn aes_ivr0(&self) -> AES_IVR0_R {
        AES_IVR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - initialization vector register (LSB IVR \\[31:0\\])"]
    #[inline(always)]
    pub fn aes_ivr0(&mut self) -> AES_IVR0_W {
        AES_IVR0_W { w: self }
    }
}
