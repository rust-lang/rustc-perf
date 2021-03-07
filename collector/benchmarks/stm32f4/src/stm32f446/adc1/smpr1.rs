#[doc = "Reader of register SMPR1"]
pub type R = crate::R<u32, super::SMPR1>;
#[doc = "Writer for register SMPR1"]
pub type W = crate::W<u32, super::SMPR1>;
#[doc = "Register SMPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 18 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP18_A {
    #[doc = "0: 3 cycles"]
    CYCLES3 = 0,
    #[doc = "1: 15 cycles"]
    CYCLES15 = 1,
    #[doc = "2: 28 cycles"]
    CYCLES28 = 2,
    #[doc = "3: 56 cycles"]
    CYCLES56 = 3,
    #[doc = "4: 84 cycles"]
    CYCLES84 = 4,
    #[doc = "5: 112 cycles"]
    CYCLES112 = 5,
    #[doc = "6: 144 cycles"]
    CYCLES144 = 6,
    #[doc = "7: 480 cycles"]
    CYCLES480 = 7,
}
impl From<SMP18_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP18_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMP18`"]
pub type SMP18_R = crate::R<u8, SMP18_A>;
impl SMP18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP18_A {
        match self.bits {
            0 => SMP18_A::CYCLES3,
            1 => SMP18_A::CYCLES15,
            2 => SMP18_A::CYCLES28,
            3 => SMP18_A::CYCLES56,
            4 => SMP18_A::CYCLES84,
            5 => SMP18_A::CYCLES112,
            6 => SMP18_A::CYCLES144,
            7 => SMP18_A::CYCLES480,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES3`"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == SMP18_A::CYCLES3
    }
    #[doc = "Checks if the value of the field is `CYCLES15`"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == SMP18_A::CYCLES15
    }
    #[doc = "Checks if the value of the field is `CYCLES28`"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == SMP18_A::CYCLES28
    }
    #[doc = "Checks if the value of the field is `CYCLES56`"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == SMP18_A::CYCLES56
    }
    #[doc = "Checks if the value of the field is `CYCLES84`"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == SMP18_A::CYCLES84
    }
    #[doc = "Checks if the value of the field is `CYCLES112`"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == SMP18_A::CYCLES112
    }
    #[doc = "Checks if the value of the field is `CYCLES144`"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == SMP18_A::CYCLES144
    }
    #[doc = "Checks if the value of the field is `CYCLES480`"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == SMP18_A::CYCLES480
    }
}
#[doc = "Write proxy for field `SMP18`"]
pub struct SMP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP18_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Channel 17 sampling time selection"]
pub type SMP17_A = SMP18_A;
#[doc = "Reader of field `SMP17`"]
pub type SMP17_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP17`"]
pub struct SMP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP17_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Channel 16 sampling time selection"]
pub type SMP16_A = SMP18_A;
#[doc = "Reader of field `SMP16`"]
pub type SMP16_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP16`"]
pub struct SMP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP16_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Channel 15 sampling time selection"]
pub type SMP15_A = SMP18_A;
#[doc = "Reader of field `SMP15`"]
pub type SMP15_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP15`"]
pub struct SMP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Channel 14 sampling time selection"]
pub type SMP14_A = SMP18_A;
#[doc = "Reader of field `SMP14`"]
pub type SMP14_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP14`"]
pub struct SMP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Channel 13 sampling time selection"]
pub type SMP13_A = SMP18_A;
#[doc = "Reader of field `SMP13`"]
pub type SMP13_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP13`"]
pub struct SMP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Channel 12 sampling time selection"]
pub type SMP12_A = SMP18_A;
#[doc = "Reader of field `SMP12`"]
pub type SMP12_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP12`"]
pub struct SMP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Channel 11 sampling time selection"]
pub type SMP11_A = SMP18_A;
#[doc = "Reader of field `SMP11`"]
pub type SMP11_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP11`"]
pub struct SMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Channel 10 sampling time selection"]
pub type SMP10_A = SMP18_A;
#[doc = "Reader of field `SMP10`"]
pub type SMP10_R = crate::R<u8, SMP18_A>;
#[doc = "Write proxy for field `SMP10`"]
pub struct SMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP18_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W {
        SMP18_W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W {
        SMP17_W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W {
        SMP16_W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W {
        SMP15_W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W {
        SMP14_W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W {
        SMP13_W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W {
        SMP12_W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W {
        SMP11_W { w: self }
    }
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W {
        SMP10_W { w: self }
    }
}
