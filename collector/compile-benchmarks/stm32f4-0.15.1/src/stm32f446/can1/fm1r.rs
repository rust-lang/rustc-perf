#[doc = "Register `FM1R` reader"]
pub struct R(crate::R<FM1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM1R` writer"]
pub struct W(crate::W<FM1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM1R_SPEC>;
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
impl From<crate::W<FM1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBM0` reader - Filter mode"]
pub type FBM0_R = crate::BitReader<bool>;
#[doc = "Field `FBM0` writer - Filter mode"]
pub type FBM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM1` reader - Filter mode"]
pub type FBM1_R = crate::BitReader<bool>;
#[doc = "Field `FBM1` writer - Filter mode"]
pub type FBM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM2` reader - Filter mode"]
pub type FBM2_R = crate::BitReader<bool>;
#[doc = "Field `FBM2` writer - Filter mode"]
pub type FBM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM3` reader - Filter mode"]
pub type FBM3_R = crate::BitReader<bool>;
#[doc = "Field `FBM3` writer - Filter mode"]
pub type FBM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM4` reader - Filter mode"]
pub type FBM4_R = crate::BitReader<bool>;
#[doc = "Field `FBM4` writer - Filter mode"]
pub type FBM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM5` reader - Filter mode"]
pub type FBM5_R = crate::BitReader<bool>;
#[doc = "Field `FBM5` writer - Filter mode"]
pub type FBM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM6` reader - Filter mode"]
pub type FBM6_R = crate::BitReader<bool>;
#[doc = "Field `FBM6` writer - Filter mode"]
pub type FBM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM7` reader - Filter mode"]
pub type FBM7_R = crate::BitReader<bool>;
#[doc = "Field `FBM7` writer - Filter mode"]
pub type FBM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM8` reader - Filter mode"]
pub type FBM8_R = crate::BitReader<bool>;
#[doc = "Field `FBM8` writer - Filter mode"]
pub type FBM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM9` reader - Filter mode"]
pub type FBM9_R = crate::BitReader<bool>;
#[doc = "Field `FBM9` writer - Filter mode"]
pub type FBM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM10` reader - Filter mode"]
pub type FBM10_R = crate::BitReader<bool>;
#[doc = "Field `FBM10` writer - Filter mode"]
pub type FBM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM11` reader - Filter mode"]
pub type FBM11_R = crate::BitReader<bool>;
#[doc = "Field `FBM11` writer - Filter mode"]
pub type FBM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM12` reader - Filter mode"]
pub type FBM12_R = crate::BitReader<bool>;
#[doc = "Field `FBM12` writer - Filter mode"]
pub type FBM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM13` reader - Filter mode"]
pub type FBM13_R = crate::BitReader<bool>;
#[doc = "Field `FBM13` writer - Filter mode"]
pub type FBM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM14` reader - Filter mode"]
pub type FBM14_R = crate::BitReader<bool>;
#[doc = "Field `FBM14` writer - Filter mode"]
pub type FBM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM15` reader - Filter mode"]
pub type FBM15_R = crate::BitReader<bool>;
#[doc = "Field `FBM15` writer - Filter mode"]
pub type FBM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM16` reader - Filter mode"]
pub type FBM16_R = crate::BitReader<bool>;
#[doc = "Field `FBM16` writer - Filter mode"]
pub type FBM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM17` reader - Filter mode"]
pub type FBM17_R = crate::BitReader<bool>;
#[doc = "Field `FBM17` writer - Filter mode"]
pub type FBM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM18` reader - Filter mode"]
pub type FBM18_R = crate::BitReader<bool>;
#[doc = "Field `FBM18` writer - Filter mode"]
pub type FBM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM19` reader - Filter mode"]
pub type FBM19_R = crate::BitReader<bool>;
#[doc = "Field `FBM19` writer - Filter mode"]
pub type FBM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM20` reader - Filter mode"]
pub type FBM20_R = crate::BitReader<bool>;
#[doc = "Field `FBM20` writer - Filter mode"]
pub type FBM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM21` reader - Filter mode"]
pub type FBM21_R = crate::BitReader<bool>;
#[doc = "Field `FBM21` writer - Filter mode"]
pub type FBM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM22` reader - Filter mode"]
pub type FBM22_R = crate::BitReader<bool>;
#[doc = "Field `FBM22` writer - Filter mode"]
pub type FBM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM23` reader - Filter mode"]
pub type FBM23_R = crate::BitReader<bool>;
#[doc = "Field `FBM23` writer - Filter mode"]
pub type FBM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM24` reader - Filter mode"]
pub type FBM24_R = crate::BitReader<bool>;
#[doc = "Field `FBM24` writer - Filter mode"]
pub type FBM24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM25` reader - Filter mode"]
pub type FBM25_R = crate::BitReader<bool>;
#[doc = "Field `FBM25` writer - Filter mode"]
pub type FBM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM26` reader - Filter mode"]
pub type FBM26_R = crate::BitReader<bool>;
#[doc = "Field `FBM26` writer - Filter mode"]
pub type FBM26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
#[doc = "Field `FBM27` reader - Filter mode"]
pub type FBM27_R = crate::BitReader<bool>;
#[doc = "Field `FBM27` writer - Filter mode"]
pub type FBM27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM1R_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&self) -> FBM0_R {
        FBM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&self) -> FBM1_R {
        FBM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&self) -> FBM2_R {
        FBM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&self) -> FBM3_R {
        FBM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&self) -> FBM4_R {
        FBM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&self) -> FBM5_R {
        FBM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&self) -> FBM6_R {
        FBM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&self) -> FBM7_R {
        FBM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&self) -> FBM8_R {
        FBM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&self) -> FBM9_R {
        FBM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&self) -> FBM10_R {
        FBM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&self) -> FBM11_R {
        FBM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&self) -> FBM12_R {
        FBM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&self) -> FBM13_R {
        FBM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fbm14(&self) -> FBM14_R {
        FBM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fbm15(&self) -> FBM15_R {
        FBM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fbm16(&self) -> FBM16_R {
        FBM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fbm17(&self) -> FBM17_R {
        FBM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fbm18(&self) -> FBM18_R {
        FBM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fbm19(&self) -> FBM19_R {
        FBM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fbm20(&self) -> FBM20_R {
        FBM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fbm21(&self) -> FBM21_R {
        FBM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fbm22(&self) -> FBM22_R {
        FBM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fbm23(&self) -> FBM23_R {
        FBM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fbm24(&self) -> FBM24_R {
        FBM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fbm25(&self) -> FBM25_R {
        FBM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fbm26(&self) -> FBM26_R {
        FBM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fbm27(&self) -> FBM27_R {
        FBM27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&mut self) -> FBM0_W<0> {
        FBM0_W::new(self)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&mut self) -> FBM1_W<1> {
        FBM1_W::new(self)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&mut self) -> FBM2_W<2> {
        FBM2_W::new(self)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&mut self) -> FBM3_W<3> {
        FBM3_W::new(self)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&mut self) -> FBM4_W<4> {
        FBM4_W::new(self)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&mut self) -> FBM5_W<5> {
        FBM5_W::new(self)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&mut self) -> FBM6_W<6> {
        FBM6_W::new(self)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&mut self) -> FBM7_W<7> {
        FBM7_W::new(self)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&mut self) -> FBM8_W<8> {
        FBM8_W::new(self)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&mut self) -> FBM9_W<9> {
        FBM9_W::new(self)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&mut self) -> FBM10_W<10> {
        FBM10_W::new(self)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&mut self) -> FBM11_W<11> {
        FBM11_W::new(self)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&mut self) -> FBM12_W<12> {
        FBM12_W::new(self)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&mut self) -> FBM13_W<13> {
        FBM13_W::new(self)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fbm14(&mut self) -> FBM14_W<14> {
        FBM14_W::new(self)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fbm15(&mut self) -> FBM15_W<15> {
        FBM15_W::new(self)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fbm16(&mut self) -> FBM16_W<16> {
        FBM16_W::new(self)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fbm17(&mut self) -> FBM17_W<17> {
        FBM17_W::new(self)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fbm18(&mut self) -> FBM18_W<18> {
        FBM18_W::new(self)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fbm19(&mut self) -> FBM19_W<19> {
        FBM19_W::new(self)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fbm20(&mut self) -> FBM20_W<20> {
        FBM20_W::new(self)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fbm21(&mut self) -> FBM21_W<21> {
        FBM21_W::new(self)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fbm22(&mut self) -> FBM22_W<22> {
        FBM22_W::new(self)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fbm23(&mut self) -> FBM23_W<23> {
        FBM23_W::new(self)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fbm24(&mut self) -> FBM24_W<24> {
        FBM24_W::new(self)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fbm25(&mut self) -> FBM25_W<25> {
        FBM25_W::new(self)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fbm26(&mut self) -> FBM26_W<26> {
        FBM26_W::new(self)
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fbm27(&mut self) -> FBM27_W<27> {
        FBM27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm1r](index.html) module"]
pub struct FM1R_SPEC;
impl crate::RegisterSpec for FM1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm1r::R](R) reader structure"]
impl crate::Readable for FM1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm1r::W](W) writer structure"]
impl crate::Writable for FM1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FM1R to value 0"]
impl crate::Resettable for FM1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
