#[doc = "Register `OSPEEDR` reader"]
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPEEDR` writer"]
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR15_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR14_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR13_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR12_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR11_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR10_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR9_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR8_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR7_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR6_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR5_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR4_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR3_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR2_A;
#[doc = "Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_A as OSPEEDR1_A;
#[doc = "Field `OSPEEDR15` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR15_R;
#[doc = "Field `OSPEEDR14` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR14_R;
#[doc = "Field `OSPEEDR13` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR13_R;
#[doc = "Field `OSPEEDR12` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR12_R;
#[doc = "Field `OSPEEDR11` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR11_R;
#[doc = "Field `OSPEEDR10` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR10_R;
#[doc = "Field `OSPEEDR9` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR9_R;
#[doc = "Field `OSPEEDR8` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR8_R;
#[doc = "Field `OSPEEDR7` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR7_R;
#[doc = "Field `OSPEEDR6` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR6_R;
#[doc = "Field `OSPEEDR5` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR5_R;
#[doc = "Field `OSPEEDR4` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR4_R;
#[doc = "Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR3_R;
#[doc = "Field `OSPEEDR2` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR2_R;
#[doc = "Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_R as OSPEEDR1_R;
#[doc = "Field `OSPEEDR15` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR15_W;
#[doc = "Field `OSPEEDR14` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR14_W;
#[doc = "Field `OSPEEDR13` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR13_W;
#[doc = "Field `OSPEEDR12` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR12_W;
#[doc = "Field `OSPEEDR11` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR11_W;
#[doc = "Field `OSPEEDR10` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR10_W;
#[doc = "Field `OSPEEDR9` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR9_W;
#[doc = "Field `OSPEEDR8` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR8_W;
#[doc = "Field `OSPEEDR7` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR7_W;
#[doc = "Field `OSPEEDR6` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR6_W;
#[doc = "Field `OSPEEDR5` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR5_W;
#[doc = "Field `OSPEEDR4` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR4_W;
#[doc = "Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR3_W;
#[doc = "Field `OSPEEDR2` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR2_W;
#[doc = "Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)"]
pub use OSPEEDR0_W as OSPEEDR1_W;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSPEEDR0_A {
    #[doc = "0: Low speed"]
    LowSpeed = 0,
    #[doc = "1: Medium speed"]
    MediumSpeed = 1,
    #[doc = "2: High speed"]
    HighSpeed = 2,
    #[doc = "3: Very high speed"]
    VeryHighSpeed = 3,
}
impl From<OSPEEDR0_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEEDR0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR0_R = crate::FieldReader<u8, OSPEEDR0_A>;
impl OSPEEDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPEEDR0_A {
        match self.bits {
            0 => OSPEEDR0_A::LowSpeed,
            1 => OSPEEDR0_A::MediumSpeed,
            2 => OSPEEDR0_A::HighSpeed,
            3 => OSPEEDR0_A::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LowSpeed`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEEDR0_A::LowSpeed
    }
    #[doc = "Checks if the value of the field is `MediumSpeed`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEEDR0_A::MediumSpeed
    }
    #[doc = "Checks if the value of the field is `HighSpeed`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEEDR0_A::HighSpeed
    }
    #[doc = "Checks if the value of the field is `VeryHighSpeed`"]
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEEDR0_A::VeryHighSpeed
    }
}
#[doc = "Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OSPEEDR_SPEC, u8, OSPEEDR0_A, 2, O>;
impl<'a, const O: u8> OSPEEDR0_W<'a, O> {
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::LowSpeed)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::MediumSpeed)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::HighSpeed)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::VeryHighSpeed)
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR12_R {
        OSPEEDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR11_R {
        OSPEEDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR10_R {
        OSPEEDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR9_R {
        OSPEEDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR8_R {
        OSPEEDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR7_R {
        OSPEEDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W<30> {
        OSPEEDR15_W::new(self)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W<28> {
        OSPEEDR14_W::new(self)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W<26> {
        OSPEEDR13_W::new(self)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr12(&mut self) -> OSPEEDR12_W<24> {
        OSPEEDR12_W::new(self)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr11(&mut self) -> OSPEEDR11_W<22> {
        OSPEEDR11_W::new(self)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr10(&mut self) -> OSPEEDR10_W<20> {
        OSPEEDR10_W::new(self)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr9(&mut self) -> OSPEEDR9_W<18> {
        OSPEEDR9_W::new(self)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr8(&mut self) -> OSPEEDR8_W<16> {
        OSPEEDR8_W::new(self)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr7(&mut self) -> OSPEEDR7_W<14> {
        OSPEEDR7_W::new(self)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W<12> {
        OSPEEDR6_W::new(self)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W<10> {
        OSPEEDR5_W::new(self)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<8> {
        OSPEEDR4_W::new(self)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<6> {
        OSPEEDR3_W::new(self)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<4> {
        OSPEEDR2_W::new(self)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<2> {
        OSPEEDR1_W::new(self)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<0> {
        OSPEEDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](index.html) module"]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospeedr::R](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
