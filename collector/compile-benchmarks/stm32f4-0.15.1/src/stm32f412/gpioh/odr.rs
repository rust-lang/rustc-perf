#[doc = "Register `ODR` reader"]
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODR` writer"]
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR15_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR14_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR13_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR12_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR11_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR10_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR9_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR8_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR7_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR6_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR5_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR4_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR3_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR2_A;
#[doc = "Port output data (y = 0..15)"]
pub use ODR0_A as ODR1_A;
#[doc = "Field `ODR15` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR15_R;
#[doc = "Field `ODR14` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR14_R;
#[doc = "Field `ODR13` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR13_R;
#[doc = "Field `ODR12` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR12_R;
#[doc = "Field `ODR11` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR11_R;
#[doc = "Field `ODR10` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR10_R;
#[doc = "Field `ODR9` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR9_R;
#[doc = "Field `ODR8` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR8_R;
#[doc = "Field `ODR7` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR7_R;
#[doc = "Field `ODR6` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR6_R;
#[doc = "Field `ODR5` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR5_R;
#[doc = "Field `ODR4` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR4_R;
#[doc = "Field `ODR3` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR3_R;
#[doc = "Field `ODR2` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR2_R;
#[doc = "Field `ODR1` reader - Port output data (y = 0..15)"]
pub use ODR0_R as ODR1_R;
#[doc = "Field `ODR15` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR15_W;
#[doc = "Field `ODR14` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR14_W;
#[doc = "Field `ODR13` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR13_W;
#[doc = "Field `ODR12` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR12_W;
#[doc = "Field `ODR11` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR11_W;
#[doc = "Field `ODR10` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR10_W;
#[doc = "Field `ODR9` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR9_W;
#[doc = "Field `ODR8` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR8_W;
#[doc = "Field `ODR7` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR7_W;
#[doc = "Field `ODR6` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR6_W;
#[doc = "Field `ODR5` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR5_W;
#[doc = "Field `ODR4` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR4_W;
#[doc = "Field `ODR3` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR3_W;
#[doc = "Field `ODR2` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR2_W;
#[doc = "Field `ODR1` writer - Port output data (y = 0..15)"]
pub use ODR0_W as ODR1_W;
#[doc = "Port output data (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR0_A {
    #[doc = "0: Set output to logic low"]
    Low = 0,
    #[doc = "1: Set output to logic high"]
    High = 1,
}
impl From<ODR0_A> for bool {
    #[inline(always)]
    fn from(variant: ODR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODR0` reader - Port output data (y = 0..15)"]
pub type ODR0_R = crate::BitReader<ODR0_A>;
impl ODR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODR0_A {
        match self.bits {
            false => ODR0_A::Low,
            true => ODR0_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ODR0_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ODR0_A::High
    }
}
#[doc = "Field `ODR0` writer - Port output data (y = 0..15)"]
pub type ODR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, ODR0_A, O>;
impl<'a, const O: u8> ODR0_W<'a, O> {
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR0_A::Low)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR0_A::High)
    }
}
impl R {
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr15(&self) -> ODR15_R {
        ODR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr14(&self) -> ODR14_R {
        ODR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr13(&self) -> ODR13_R {
        ODR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr12(&self) -> ODR12_R {
        ODR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr11(&self) -> ODR11_R {
        ODR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr10(&self) -> ODR10_R {
        ODR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr9(&self) -> ODR9_R {
        ODR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr8(&self) -> ODR8_R {
        ODR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr7(&self) -> ODR7_R {
        ODR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr15(&mut self) -> ODR15_W<15> {
        ODR15_W::new(self)
    }
    #[doc = "Bit 14 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr14(&mut self) -> ODR14_W<14> {
        ODR14_W::new(self)
    }
    #[doc = "Bit 13 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr13(&mut self) -> ODR13_W<13> {
        ODR13_W::new(self)
    }
    #[doc = "Bit 12 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr12(&mut self) -> ODR12_W<12> {
        ODR12_W::new(self)
    }
    #[doc = "Bit 11 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr11(&mut self) -> ODR11_W<11> {
        ODR11_W::new(self)
    }
    #[doc = "Bit 10 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr10(&mut self) -> ODR10_W<10> {
        ODR10_W::new(self)
    }
    #[doc = "Bit 9 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr9(&mut self) -> ODR9_W<9> {
        ODR9_W::new(self)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr8(&mut self) -> ODR8_W<8> {
        ODR8_W::new(self)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr7(&mut self) -> ODR7_W<7> {
        ODR7_W::new(self)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr6(&mut self) -> ODR6_W<6> {
        ODR6_W::new(self)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr5(&mut self) -> ODR5_W<5> {
        ODR5_W::new(self)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr4(&mut self) -> ODR4_W<4> {
        ODR4_W::new(self)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR3_W<3> {
        ODR3_W::new(self)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr2(&mut self) -> ODR2_W<2> {
        ODR2_W::new(self)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr1(&mut self) -> ODR1_W<1> {
        ODR1_W::new(self)
    }
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn odr0(&mut self) -> ODR0_W<0> {
        ODR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](index.html) module"]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odr::R](R) reader structure"]
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odr::W](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
