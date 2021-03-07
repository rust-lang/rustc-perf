#[doc = "Reader of register PLLSAICFGR"]
pub type R = crate::R<u32, super::PLLSAICFGR>;
#[doc = "Writer for register PLLSAICFGR"]
pub type W = crate::W<u32, super::PLLSAICFGR>;
#[doc = "Register PLLSAICFGR `reset()`'s with value 0x2400_3000"]
impl crate::ResetValue for super::PLLSAICFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2400_3000
    }
}
#[doc = "Reader of field `PLLSAIM`"]
pub type PLLSAIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAIM`"]
pub struct PLLSAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `PLLSAIN`"]
pub type PLLSAIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLLSAIN`"]
pub struct PLLSAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
#[doc = "PLLSAI division factor for 48 MHz clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAIP_A {
    #[doc = "0: PLL*P=2"]
    DIV2 = 0,
    #[doc = "1: PLL*P=4"]
    DIV4 = 1,
    #[doc = "2: PLL*P=6"]
    DIV6 = 2,
    #[doc = "3: PLL*P=8"]
    DIV8 = 3,
}
impl From<PLLSAIP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLSAIP`"]
pub type PLLSAIP_R = crate::R<u8, PLLSAIP_A>;
impl PLLSAIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIP_A {
        match self.bits {
            0 => PLLSAIP_A::DIV2,
            1 => PLLSAIP_A::DIV4,
            2 => PLLSAIP_A::DIV6,
            3 => PLLSAIP_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIP_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIP_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIP_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIP_A::DIV8
    }
}
#[doc = "Write proxy for field `PLLSAIP`"]
pub struct PLLSAIP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAIP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLL*P=2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIP_A::DIV2)
    }
    #[doc = "PLL*P=4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIP_A::DIV4)
    }
    #[doc = "PLL*P=6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAIP_A::DIV6)
    }
    #[doc = "PLL*P=8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIP_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PLLSAIQ`"]
pub type PLLSAIQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAIQ`"]
pub struct PLLSAIQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Division factor for audio PLLSAI input clock"]
    #[inline(always)]
    pub fn pllsaim(&self) -> PLLSAIM_R {
        PLLSAIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&self) -> PLLSAIN_R {
        PLLSAIN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - PLLSAI division factor for 48 MHz clock"]
    #[inline(always)]
    pub fn pllsaip(&self) -> PLLSAIP_R {
        PLLSAIP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAIs clock"]
    #[inline(always)]
    pub fn pllsaiq(&self) -> PLLSAIQ_R {
        PLLSAIQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Division factor for audio PLLSAI input clock"]
    #[inline(always)]
    pub fn pllsaim(&mut self) -> PLLSAIM_W {
        PLLSAIM_W { w: self }
    }
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&mut self) -> PLLSAIN_W {
        PLLSAIN_W { w: self }
    }
    #[doc = "Bits 16:17 - PLLSAI division factor for 48 MHz clock"]
    #[inline(always)]
    pub fn pllsaip(&mut self) -> PLLSAIP_W {
        PLLSAIP_W { w: self }
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAIs clock"]
    #[inline(always)]
    pub fn pllsaiq(&mut self) -> PLLSAIQ_W {
        PLLSAIQ_W { w: self }
    }
}
