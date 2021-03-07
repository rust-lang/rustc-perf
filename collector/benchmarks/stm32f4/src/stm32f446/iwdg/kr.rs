#[doc = "Writer for register KR"]
pub type W = crate::W<u32, super::KR>;
#[doc = "Register KR `reset()`'s with value 0"]
impl crate::ResetValue for super::KR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Key value (write only, read 0000h)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum KEY_AW {
    #[doc = "21845: Enable access to PR, RLR and WINR registers (0x5555)"]
    ENABLE = 21845,
    #[doc = "43690: Reset the watchdog value (0xAAAA)"]
    RESET = 43690,
    #[doc = "52428: Start the watchdog (0xCCCC)"]
    START = 52428,
}
impl From<KEY_AW> for u16 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(KEY_AW::ENABLE)
    }
    #[doc = "Reset the watchdog value (0xAAAA)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(KEY_AW::RESET)
    }
    #[doc = "Start the watchdog (0xCCCC)"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(KEY_AW::START)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Key value (write only, read 0000h)"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
