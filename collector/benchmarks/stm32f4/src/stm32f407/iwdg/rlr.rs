#[doc = "Reader of register RLR"]
pub type R = crate::R<u32, super::RLR>;
#[doc = "Writer for register RLR"]
pub type W = crate::W<u32, super::RLR>;
#[doc = "Register RLR `reset()`'s with value 0x0fff"]
impl crate::ResetValue for super::RLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
#[doc = "Reader of field `RL`"]
pub type RL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RL`"]
pub struct RL_W<'a> {
    w: &'a mut W,
}
impl<'a> RL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W {
        RL_W { w: self }
    }
}
