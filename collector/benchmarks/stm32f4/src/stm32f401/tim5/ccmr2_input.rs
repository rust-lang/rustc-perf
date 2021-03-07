#[doc = "Reader of register CCMR2_Input"]
pub type R = crate::R<u32, super::CCMR2_INPUT>;
#[doc = "Writer for register CCMR2_Input"]
pub type W = crate::W<u32, super::CCMR2_INPUT>;
#[doc = "Register CCMR2_Input `reset()`'s with value 0"]
impl crate::ResetValue for super::CCMR2_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC4F`"]
pub type IC4F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC4F`"]
pub struct IC4F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `IC4PSC`"]
pub type IC4PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC4PSC`"]
pub struct IC4PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Capture/Compare 4 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC4S_A {
    #[doc = "1: CC4 channel is configured as input, IC4 is mapped on TI4"]
    TI4 = 1,
    #[doc = "2: CC4 channel is configured as input, IC4 is mapped on TI3"]
    TI3 = 2,
    #[doc = "3: CC4 channel is configured as input, IC4 is mapped on TRC"]
    TRC = 3,
}
impl From<CC4S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC4S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC4S`"]
pub type CC4S_R = crate::R<u8, CC4S_A>;
impl CC4S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC4S_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CC4S_A::TI4),
            2 => Val(CC4S_A::TI3),
            3 => Val(CC4S_A::TRC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TI4`"]
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        *self == CC4S_A::TI4
    }
    #[doc = "Checks if the value of the field is `TI3`"]
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        *self == CC4S_A::TI3
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC4S_A::TRC
    }
}
#[doc = "Write proxy for field `CC4S`"]
pub struct CC4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC4S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline(always)]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC4S_A::TI4)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline(always)]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC4S_A::TI3)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC4S_A::TRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IC3F`"]
pub type IC3F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3F`"]
pub struct IC3F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `IC3PSC`"]
pub type IC3PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3PSC`"]
pub struct IC3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Capture/compare 3 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC3S_A {
    #[doc = "1: CC3 channel is configured as input, IC3 is mapped on TI3"]
    TI3 = 1,
    #[doc = "2: CC3 channel is configured as input, IC3 is mapped on TI4"]
    TI4 = 2,
    #[doc = "3: CC3 channel is configured as input, IC3 is mapped on TRC"]
    TRC = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC3S`"]
pub type CC3S_R = crate::R<u8, CC3S_A>;
impl CC3S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC3S_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CC3S_A::TI3),
            2 => Val(CC3S_A::TI4),
            3 => Val(CC3S_A::TRC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TI3`"]
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        *self == CC3S_A::TI3
    }
    #[doc = "Checks if the value of the field is `TI4`"]
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        *self == CC3S_A::TI4
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC3S_A::TRC
    }
}
#[doc = "Write proxy for field `CC3S`"]
pub struct CC3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline(always)]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC3S_A::TI3)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline(always)]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC3S_A::TI4)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC3S_A::TRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W {
        IC4F_W { w: self }
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W {
        IC4PSC_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W {
        IC3F_W { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W {
        IC3PSC_W { w: self }
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
}
