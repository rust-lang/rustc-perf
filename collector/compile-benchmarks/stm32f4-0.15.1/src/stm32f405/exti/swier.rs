#[doc = "Register `SWIER` reader"]
pub struct R(crate::R<SWIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER` writer"]
pub struct W(crate::W<SWIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER_SPEC>;
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
impl From<crate::W<SWIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software Interrupt on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER0_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER0_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER0` reader - Software Interrupt on line 0"]
pub type SWIER0_R = crate::BitReader<SWIER0_A>;
impl SWIER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER0_A> {
        match self.bits {
            true => Some(SWIER0_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER0_A::Pend
    }
}
#[doc = "Field `SWIER0` writer - Software Interrupt on line 0"]
pub type SWIER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER_SPEC, SWIER0_A, O>;
impl<'a, const O: u8> SWIER0_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER0_A::Pend)
    }
}
#[doc = "Software Interrupt on line 1"]
pub use SWIER0_A as SWIER1_A;
#[doc = "Software Interrupt on line 2"]
pub use SWIER0_A as SWIER2_A;
#[doc = "Software Interrupt on line 3"]
pub use SWIER0_A as SWIER3_A;
#[doc = "Software Interrupt on line 4"]
pub use SWIER0_A as SWIER4_A;
#[doc = "Software Interrupt on line 5"]
pub use SWIER0_A as SWIER5_A;
#[doc = "Software Interrupt on line 6"]
pub use SWIER0_A as SWIER6_A;
#[doc = "Software Interrupt on line 7"]
pub use SWIER0_A as SWIER7_A;
#[doc = "Software Interrupt on line 8"]
pub use SWIER0_A as SWIER8_A;
#[doc = "Software Interrupt on line 9"]
pub use SWIER0_A as SWIER9_A;
#[doc = "Software Interrupt on line 10"]
pub use SWIER0_A as SWIER10_A;
#[doc = "Software Interrupt on line 11"]
pub use SWIER0_A as SWIER11_A;
#[doc = "Software Interrupt on line 12"]
pub use SWIER0_A as SWIER12_A;
#[doc = "Software Interrupt on line 13"]
pub use SWIER0_A as SWIER13_A;
#[doc = "Software Interrupt on line 14"]
pub use SWIER0_A as SWIER14_A;
#[doc = "Software Interrupt on line 15"]
pub use SWIER0_A as SWIER15_A;
#[doc = "Software Interrupt on line 16"]
pub use SWIER0_A as SWIER16_A;
#[doc = "Software Interrupt on line 17"]
pub use SWIER0_A as SWIER17_A;
#[doc = "Software Interrupt on line 18"]
pub use SWIER0_A as SWIER18_A;
#[doc = "Software Interrupt on line 19"]
pub use SWIER0_A as SWIER19_A;
#[doc = "Software Interrupt on line 20"]
pub use SWIER0_A as SWIER20_A;
#[doc = "Software Interrupt on line 21"]
pub use SWIER0_A as SWIER21_A;
#[doc = "Software Interrupt on line 22"]
pub use SWIER0_A as SWIER22_A;
#[doc = "Field `SWIER1` reader - Software Interrupt on line 1"]
pub use SWIER0_R as SWIER1_R;
#[doc = "Field `SWIER2` reader - Software Interrupt on line 2"]
pub use SWIER0_R as SWIER2_R;
#[doc = "Field `SWIER3` reader - Software Interrupt on line 3"]
pub use SWIER0_R as SWIER3_R;
#[doc = "Field `SWIER4` reader - Software Interrupt on line 4"]
pub use SWIER0_R as SWIER4_R;
#[doc = "Field `SWIER5` reader - Software Interrupt on line 5"]
pub use SWIER0_R as SWIER5_R;
#[doc = "Field `SWIER6` reader - Software Interrupt on line 6"]
pub use SWIER0_R as SWIER6_R;
#[doc = "Field `SWIER7` reader - Software Interrupt on line 7"]
pub use SWIER0_R as SWIER7_R;
#[doc = "Field `SWIER8` reader - Software Interrupt on line 8"]
pub use SWIER0_R as SWIER8_R;
#[doc = "Field `SWIER9` reader - Software Interrupt on line 9"]
pub use SWIER0_R as SWIER9_R;
#[doc = "Field `SWIER10` reader - Software Interrupt on line 10"]
pub use SWIER0_R as SWIER10_R;
#[doc = "Field `SWIER11` reader - Software Interrupt on line 11"]
pub use SWIER0_R as SWIER11_R;
#[doc = "Field `SWIER12` reader - Software Interrupt on line 12"]
pub use SWIER0_R as SWIER12_R;
#[doc = "Field `SWIER13` reader - Software Interrupt on line 13"]
pub use SWIER0_R as SWIER13_R;
#[doc = "Field `SWIER14` reader - Software Interrupt on line 14"]
pub use SWIER0_R as SWIER14_R;
#[doc = "Field `SWIER15` reader - Software Interrupt on line 15"]
pub use SWIER0_R as SWIER15_R;
#[doc = "Field `SWIER16` reader - Software Interrupt on line 16"]
pub use SWIER0_R as SWIER16_R;
#[doc = "Field `SWIER17` reader - Software Interrupt on line 17"]
pub use SWIER0_R as SWIER17_R;
#[doc = "Field `SWIER18` reader - Software Interrupt on line 18"]
pub use SWIER0_R as SWIER18_R;
#[doc = "Field `SWIER19` reader - Software Interrupt on line 19"]
pub use SWIER0_R as SWIER19_R;
#[doc = "Field `SWIER20` reader - Software Interrupt on line 20"]
pub use SWIER0_R as SWIER20_R;
#[doc = "Field `SWIER21` reader - Software Interrupt on line 21"]
pub use SWIER0_R as SWIER21_R;
#[doc = "Field `SWIER22` reader - Software Interrupt on line 22"]
pub use SWIER0_R as SWIER22_R;
#[doc = "Field `SWIER1` writer - Software Interrupt on line 1"]
pub use SWIER0_W as SWIER1_W;
#[doc = "Field `SWIER2` writer - Software Interrupt on line 2"]
pub use SWIER0_W as SWIER2_W;
#[doc = "Field `SWIER3` writer - Software Interrupt on line 3"]
pub use SWIER0_W as SWIER3_W;
#[doc = "Field `SWIER4` writer - Software Interrupt on line 4"]
pub use SWIER0_W as SWIER4_W;
#[doc = "Field `SWIER5` writer - Software Interrupt on line 5"]
pub use SWIER0_W as SWIER5_W;
#[doc = "Field `SWIER6` writer - Software Interrupt on line 6"]
pub use SWIER0_W as SWIER6_W;
#[doc = "Field `SWIER7` writer - Software Interrupt on line 7"]
pub use SWIER0_W as SWIER7_W;
#[doc = "Field `SWIER8` writer - Software Interrupt on line 8"]
pub use SWIER0_W as SWIER8_W;
#[doc = "Field `SWIER9` writer - Software Interrupt on line 9"]
pub use SWIER0_W as SWIER9_W;
#[doc = "Field `SWIER10` writer - Software Interrupt on line 10"]
pub use SWIER0_W as SWIER10_W;
#[doc = "Field `SWIER11` writer - Software Interrupt on line 11"]
pub use SWIER0_W as SWIER11_W;
#[doc = "Field `SWIER12` writer - Software Interrupt on line 12"]
pub use SWIER0_W as SWIER12_W;
#[doc = "Field `SWIER13` writer - Software Interrupt on line 13"]
pub use SWIER0_W as SWIER13_W;
#[doc = "Field `SWIER14` writer - Software Interrupt on line 14"]
pub use SWIER0_W as SWIER14_W;
#[doc = "Field `SWIER15` writer - Software Interrupt on line 15"]
pub use SWIER0_W as SWIER15_W;
#[doc = "Field `SWIER16` writer - Software Interrupt on line 16"]
pub use SWIER0_W as SWIER16_W;
#[doc = "Field `SWIER17` writer - Software Interrupt on line 17"]
pub use SWIER0_W as SWIER17_W;
#[doc = "Field `SWIER18` writer - Software Interrupt on line 18"]
pub use SWIER0_W as SWIER18_W;
#[doc = "Field `SWIER19` writer - Software Interrupt on line 19"]
pub use SWIER0_W as SWIER19_W;
#[doc = "Field `SWIER20` writer - Software Interrupt on line 20"]
pub use SWIER0_W as SWIER20_W;
#[doc = "Field `SWIER21` writer - Software Interrupt on line 21"]
pub use SWIER0_W as SWIER21_W;
#[doc = "Field `SWIER22` writer - Software Interrupt on line 22"]
pub use SWIER0_W as SWIER22_W;
impl R {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swier17(&self) -> SWIER17_R {
        SWIER17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    pub fn swier18(&self) -> SWIER18_R {
        SWIER18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swier19(&self) -> SWIER19_R {
        SWIER19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swier20(&self) -> SWIER20_R {
        SWIER20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swier21(&self) -> SWIER21_R {
        SWIER21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swier22(&self) -> SWIER22_R {
        SWIER22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swier0(&mut self) -> SWIER0_W<0> {
        SWIER0_W::new(self)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swier1(&mut self) -> SWIER1_W<1> {
        SWIER1_W::new(self)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swier2(&mut self) -> SWIER2_W<2> {
        SWIER2_W::new(self)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swier3(&mut self) -> SWIER3_W<3> {
        SWIER3_W::new(self)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swier4(&mut self) -> SWIER4_W<4> {
        SWIER4_W::new(self)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swier5(&mut self) -> SWIER5_W<5> {
        SWIER5_W::new(self)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swier6(&mut self) -> SWIER6_W<6> {
        SWIER6_W::new(self)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swier7(&mut self) -> SWIER7_W<7> {
        SWIER7_W::new(self)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swier8(&mut self) -> SWIER8_W<8> {
        SWIER8_W::new(self)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swier9(&mut self) -> SWIER9_W<9> {
        SWIER9_W::new(self)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swier10(&mut self) -> SWIER10_W<10> {
        SWIER10_W::new(self)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swier11(&mut self) -> SWIER11_W<11> {
        SWIER11_W::new(self)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swier12(&mut self) -> SWIER12_W<12> {
        SWIER12_W::new(self)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swier13(&mut self) -> SWIER13_W<13> {
        SWIER13_W::new(self)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swier14(&mut self) -> SWIER14_W<14> {
        SWIER14_W::new(self)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swier15(&mut self) -> SWIER15_W<15> {
        SWIER15_W::new(self)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swier16(&mut self) -> SWIER16_W<16> {
        SWIER16_W::new(self)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swier17(&mut self) -> SWIER17_W<17> {
        SWIER17_W::new(self)
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    pub fn swier18(&mut self) -> SWIER18_W<18> {
        SWIER18_W::new(self)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swier19(&mut self) -> SWIER19_W<19> {
        SWIER19_W::new(self)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swier20(&mut self) -> SWIER20_W<20> {
        SWIER20_W::new(self)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swier21(&mut self) -> SWIER21_W<21> {
        SWIER21_W::new(self)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swier22(&mut self) -> SWIER22_W<22> {
        SWIER22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt event register (EXTI_SWIER)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier](index.html) module"]
pub struct SWIER_SPEC;
impl crate::RegisterSpec for SWIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier::R](R) reader structure"]
impl crate::Readable for SWIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier::W](W) writer structure"]
impl crate::Writable for SWIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SWIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
