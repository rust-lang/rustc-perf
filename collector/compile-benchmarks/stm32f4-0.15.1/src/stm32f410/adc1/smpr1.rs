#[doc = "Register `SMPR1` reader"]
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR1` writer"]
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 18 sampling time selection"]
pub use SMP10_A as SMP18_A;
#[doc = "Channel 17 sampling time selection"]
pub use SMP10_A as SMP17_A;
#[doc = "Channel 16 sampling time selection"]
pub use SMP10_A as SMP16_A;
#[doc = "Channel 15 sampling time selection"]
pub use SMP10_A as SMP15_A;
#[doc = "Channel 14 sampling time selection"]
pub use SMP10_A as SMP14_A;
#[doc = "Channel 13 sampling time selection"]
pub use SMP10_A as SMP13_A;
#[doc = "Channel 12 sampling time selection"]
pub use SMP10_A as SMP12_A;
#[doc = "Channel 11 sampling time selection"]
pub use SMP10_A as SMP11_A;
#[doc = "Field `SMP18` reader - Channel 18 sampling time selection"]
pub use SMP10_R as SMP18_R;
#[doc = "Field `SMP17` reader - Channel 17 sampling time selection"]
pub use SMP10_R as SMP17_R;
#[doc = "Field `SMP16` reader - Channel 16 sampling time selection"]
pub use SMP10_R as SMP16_R;
#[doc = "Field `SMP15` reader - Channel 15 sampling time selection"]
pub use SMP10_R as SMP15_R;
#[doc = "Field `SMP14` reader - Channel 14 sampling time selection"]
pub use SMP10_R as SMP14_R;
#[doc = "Field `SMP13` reader - Channel 13 sampling time selection"]
pub use SMP10_R as SMP13_R;
#[doc = "Field `SMP12` reader - Channel 12 sampling time selection"]
pub use SMP10_R as SMP12_R;
#[doc = "Field `SMP11` reader - Channel 11 sampling time selection"]
pub use SMP10_R as SMP11_R;
#[doc = "Field `SMP18` writer - Channel 18 sampling time selection"]
pub use SMP10_W as SMP18_W;
#[doc = "Field `SMP17` writer - Channel 17 sampling time selection"]
pub use SMP10_W as SMP17_W;
#[doc = "Field `SMP16` writer - Channel 16 sampling time selection"]
pub use SMP10_W as SMP16_W;
#[doc = "Field `SMP15` writer - Channel 15 sampling time selection"]
pub use SMP10_W as SMP15_W;
#[doc = "Field `SMP14` writer - Channel 14 sampling time selection"]
pub use SMP10_W as SMP14_W;
#[doc = "Field `SMP13` writer - Channel 13 sampling time selection"]
pub use SMP10_W as SMP13_W;
#[doc = "Field `SMP12` writer - Channel 12 sampling time selection"]
pub use SMP10_W as SMP12_W;
#[doc = "Field `SMP11` writer - Channel 11 sampling time selection"]
pub use SMP10_W as SMP11_W;
#[doc = "Channel 10 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP10_A {
    #[doc = "0: 3 cycles"]
    Cycles3 = 0,
    #[doc = "1: 15 cycles"]
    Cycles15 = 1,
    #[doc = "2: 28 cycles"]
    Cycles28 = 2,
    #[doc = "3: 56 cycles"]
    Cycles56 = 3,
    #[doc = "4: 84 cycles"]
    Cycles84 = 4,
    #[doc = "5: 112 cycles"]
    Cycles112 = 5,
    #[doc = "6: 144 cycles"]
    Cycles144 = 6,
    #[doc = "7: 480 cycles"]
    Cycles480 = 7,
}
impl From<SMP10_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMP10` reader - Channel 10 sampling time selection"]
pub type SMP10_R = crate::FieldReader<u8, SMP10_A>;
impl SMP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP10_A {
        match self.bits {
            0 => SMP10_A::Cycles3,
            1 => SMP10_A::Cycles15,
            2 => SMP10_A::Cycles28,
            3 => SMP10_A::Cycles56,
            4 => SMP10_A::Cycles84,
            5 => SMP10_A::Cycles112,
            6 => SMP10_A::Cycles144,
            7 => SMP10_A::Cycles480,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles3`"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == SMP10_A::Cycles3
    }
    #[doc = "Checks if the value of the field is `Cycles15`"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == SMP10_A::Cycles15
    }
    #[doc = "Checks if the value of the field is `Cycles28`"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == SMP10_A::Cycles28
    }
    #[doc = "Checks if the value of the field is `Cycles56`"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == SMP10_A::Cycles56
    }
    #[doc = "Checks if the value of the field is `Cycles84`"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == SMP10_A::Cycles84
    }
    #[doc = "Checks if the value of the field is `Cycles112`"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == SMP10_A::Cycles112
    }
    #[doc = "Checks if the value of the field is `Cycles144`"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == SMP10_A::Cycles144
    }
    #[doc = "Checks if the value of the field is `Cycles480`"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == SMP10_A::Cycles480
    }
}
#[doc = "Field `SMP10` writer - Channel 10 sampling time selection"]
pub type SMP10_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR1_SPEC, u8, SMP10_A, 3, O>;
impl<'a, const O: u8> SMP10_W<'a, O> {
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP10_A::Cycles480)
    }
}
impl R {
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W<24> {
        SMP18_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W<21> {
        SMP17_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W<18> {
        SMP16_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W<15> {
        SMP15_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W<12> {
        SMP14_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W<9> {
        SMP13_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W<6> {
        SMP12_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W<3> {
        SMP11_W::new(self)
    }
    #[doc = "Bits 0:2 - Channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W<0> {
        SMP10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](index.html) module"]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr1::R](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr1::W](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
