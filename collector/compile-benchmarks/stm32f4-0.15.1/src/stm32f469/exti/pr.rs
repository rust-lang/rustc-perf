#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR` writer"]
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pending bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR0_A> for bool {
    #[inline(always)]
    fn from(variant: PR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR0` reader - Pending bit 0"]
pub type PR0_R = crate::BitReader<PR0_A>;
impl PR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR0_A {
        match self.bits {
            false => PR0_A::NotPending,
            true => PR0_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR0_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR0_A::Pending
    }
}
#[doc = "Pending bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR0_AW> for bool {
    #[inline(always)]
    fn from(variant: PR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR0` writer - Pending bit 0"]
pub type PR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, PR0_AW, O>;
impl<'a, const O: u8> PR0_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::Clear)
    }
}
#[doc = "Pending bit 1"]
pub use PR0_A as PR1_A;
#[doc = "Pending bit 2"]
pub use PR0_A as PR2_A;
#[doc = "Pending bit 3"]
pub use PR0_A as PR3_A;
#[doc = "Pending bit 4"]
pub use PR0_A as PR4_A;
#[doc = "Pending bit 5"]
pub use PR0_A as PR5_A;
#[doc = "Pending bit 6"]
pub use PR0_A as PR6_A;
#[doc = "Pending bit 7"]
pub use PR0_A as PR7_A;
#[doc = "Pending bit 8"]
pub use PR0_A as PR8_A;
#[doc = "Pending bit 9"]
pub use PR0_A as PR9_A;
#[doc = "Pending bit 10"]
pub use PR0_A as PR10_A;
#[doc = "Pending bit 11"]
pub use PR0_A as PR11_A;
#[doc = "Pending bit 12"]
pub use PR0_A as PR12_A;
#[doc = "Pending bit 13"]
pub use PR0_A as PR13_A;
#[doc = "Pending bit 14"]
pub use PR0_A as PR14_A;
#[doc = "Pending bit 15"]
pub use PR0_A as PR15_A;
#[doc = "Pending bit 16"]
pub use PR0_A as PR16_A;
#[doc = "Pending bit 17"]
pub use PR0_A as PR17_A;
#[doc = "Pending bit 18"]
pub use PR0_A as PR18_A;
#[doc = "Pending bit 19"]
pub use PR0_A as PR19_A;
#[doc = "Pending bit 20"]
pub use PR0_A as PR20_A;
#[doc = "Pending bit 21"]
pub use PR0_A as PR21_A;
#[doc = "Pending bit 22"]
pub use PR0_A as PR22_A;
#[doc = "Pending bit 1"]
pub use PR0_AW as PR1_AW;
#[doc = "Pending bit 2"]
pub use PR0_AW as PR2_AW;
#[doc = "Pending bit 3"]
pub use PR0_AW as PR3_AW;
#[doc = "Pending bit 4"]
pub use PR0_AW as PR4_AW;
#[doc = "Pending bit 5"]
pub use PR0_AW as PR5_AW;
#[doc = "Pending bit 6"]
pub use PR0_AW as PR6_AW;
#[doc = "Pending bit 7"]
pub use PR0_AW as PR7_AW;
#[doc = "Pending bit 8"]
pub use PR0_AW as PR8_AW;
#[doc = "Pending bit 9"]
pub use PR0_AW as PR9_AW;
#[doc = "Pending bit 10"]
pub use PR0_AW as PR10_AW;
#[doc = "Pending bit 11"]
pub use PR0_AW as PR11_AW;
#[doc = "Pending bit 12"]
pub use PR0_AW as PR12_AW;
#[doc = "Pending bit 13"]
pub use PR0_AW as PR13_AW;
#[doc = "Pending bit 14"]
pub use PR0_AW as PR14_AW;
#[doc = "Pending bit 15"]
pub use PR0_AW as PR15_AW;
#[doc = "Pending bit 16"]
pub use PR0_AW as PR16_AW;
#[doc = "Pending bit 17"]
pub use PR0_AW as PR17_AW;
#[doc = "Pending bit 18"]
pub use PR0_AW as PR18_AW;
#[doc = "Pending bit 19"]
pub use PR0_AW as PR19_AW;
#[doc = "Pending bit 20"]
pub use PR0_AW as PR20_AW;
#[doc = "Pending bit 21"]
pub use PR0_AW as PR21_AW;
#[doc = "Pending bit 22"]
pub use PR0_AW as PR22_AW;
#[doc = "Field `PR1` reader - Pending bit 1"]
pub use PR0_R as PR1_R;
#[doc = "Field `PR2` reader - Pending bit 2"]
pub use PR0_R as PR2_R;
#[doc = "Field `PR3` reader - Pending bit 3"]
pub use PR0_R as PR3_R;
#[doc = "Field `PR4` reader - Pending bit 4"]
pub use PR0_R as PR4_R;
#[doc = "Field `PR5` reader - Pending bit 5"]
pub use PR0_R as PR5_R;
#[doc = "Field `PR6` reader - Pending bit 6"]
pub use PR0_R as PR6_R;
#[doc = "Field `PR7` reader - Pending bit 7"]
pub use PR0_R as PR7_R;
#[doc = "Field `PR8` reader - Pending bit 8"]
pub use PR0_R as PR8_R;
#[doc = "Field `PR9` reader - Pending bit 9"]
pub use PR0_R as PR9_R;
#[doc = "Field `PR10` reader - Pending bit 10"]
pub use PR0_R as PR10_R;
#[doc = "Field `PR11` reader - Pending bit 11"]
pub use PR0_R as PR11_R;
#[doc = "Field `PR12` reader - Pending bit 12"]
pub use PR0_R as PR12_R;
#[doc = "Field `PR13` reader - Pending bit 13"]
pub use PR0_R as PR13_R;
#[doc = "Field `PR14` reader - Pending bit 14"]
pub use PR0_R as PR14_R;
#[doc = "Field `PR15` reader - Pending bit 15"]
pub use PR0_R as PR15_R;
#[doc = "Field `PR16` reader - Pending bit 16"]
pub use PR0_R as PR16_R;
#[doc = "Field `PR17` reader - Pending bit 17"]
pub use PR0_R as PR17_R;
#[doc = "Field `PR18` reader - Pending bit 18"]
pub use PR0_R as PR18_R;
#[doc = "Field `PR19` reader - Pending bit 19"]
pub use PR0_R as PR19_R;
#[doc = "Field `PR20` reader - Pending bit 20"]
pub use PR0_R as PR20_R;
#[doc = "Field `PR21` reader - Pending bit 21"]
pub use PR0_R as PR21_R;
#[doc = "Field `PR22` reader - Pending bit 22"]
pub use PR0_R as PR22_R;
#[doc = "Field `PR1` writer - Pending bit 1"]
pub use PR0_W as PR1_W;
#[doc = "Field `PR2` writer - Pending bit 2"]
pub use PR0_W as PR2_W;
#[doc = "Field `PR3` writer - Pending bit 3"]
pub use PR0_W as PR3_W;
#[doc = "Field `PR4` writer - Pending bit 4"]
pub use PR0_W as PR4_W;
#[doc = "Field `PR5` writer - Pending bit 5"]
pub use PR0_W as PR5_W;
#[doc = "Field `PR6` writer - Pending bit 6"]
pub use PR0_W as PR6_W;
#[doc = "Field `PR7` writer - Pending bit 7"]
pub use PR0_W as PR7_W;
#[doc = "Field `PR8` writer - Pending bit 8"]
pub use PR0_W as PR8_W;
#[doc = "Field `PR9` writer - Pending bit 9"]
pub use PR0_W as PR9_W;
#[doc = "Field `PR10` writer - Pending bit 10"]
pub use PR0_W as PR10_W;
#[doc = "Field `PR11` writer - Pending bit 11"]
pub use PR0_W as PR11_W;
#[doc = "Field `PR12` writer - Pending bit 12"]
pub use PR0_W as PR12_W;
#[doc = "Field `PR13` writer - Pending bit 13"]
pub use PR0_W as PR13_W;
#[doc = "Field `PR14` writer - Pending bit 14"]
pub use PR0_W as PR14_W;
#[doc = "Field `PR15` writer - Pending bit 15"]
pub use PR0_W as PR15_W;
#[doc = "Field `PR16` writer - Pending bit 16"]
pub use PR0_W as PR16_W;
#[doc = "Field `PR17` writer - Pending bit 17"]
pub use PR0_W as PR17_W;
#[doc = "Field `PR18` writer - Pending bit 18"]
pub use PR0_W as PR18_W;
#[doc = "Field `PR19` writer - Pending bit 19"]
pub use PR0_W as PR19_W;
#[doc = "Field `PR20` writer - Pending bit 20"]
pub use PR0_W as PR20_W;
#[doc = "Field `PR21` writer - Pending bit 21"]
pub use PR0_W as PR21_W;
#[doc = "Field `PR22` writer - Pending bit 22"]
pub use PR0_W as PR22_W;
impl R {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    pub fn pr20(&self) -> PR20_R {
        PR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn pr21(&self) -> PR21_R {
        PR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn pr22(&self) -> PR22_R {
        PR22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W<0> {
        PR0_W::new(self)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W<1> {
        PR1_W::new(self)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W<2> {
        PR2_W::new(self)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W<3> {
        PR3_W::new(self)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W<4> {
        PR4_W::new(self)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W<5> {
        PR5_W::new(self)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W<6> {
        PR6_W::new(self)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W<7> {
        PR7_W::new(self)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W<8> {
        PR8_W::new(self)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W<9> {
        PR9_W::new(self)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W<10> {
        PR10_W::new(self)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W<11> {
        PR11_W::new(self)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W<12> {
        PR12_W::new(self)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W<13> {
        PR13_W::new(self)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W<14> {
        PR14_W::new(self)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W<15> {
        PR15_W::new(self)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&mut self) -> PR16_W<16> {
        PR16_W::new(self)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&mut self) -> PR17_W<17> {
        PR17_W::new(self)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&mut self) -> PR18_W<18> {
        PR18_W::new(self)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    pub fn pr19(&mut self) -> PR19_W<19> {
        PR19_W::new(self)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    pub fn pr20(&mut self) -> PR20_W<20> {
        PR20_W::new(self)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn pr21(&mut self) -> PR21_W<21> {
        PR21_W::new(self)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn pr22(&mut self) -> PR22_W<22> {
        PR22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register (EXTI_PR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr::W](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
