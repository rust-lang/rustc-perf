#[doc = "Register `FFA1R` reader"]
pub struct R(crate::R<FFA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFA1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFA1R` writer"]
pub struct W(crate::W<FFA1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFA1R_SPEC>;
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
impl From<crate::W<FFA1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFA1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFA0` reader - Filter FIFO assignment for filter 0"]
pub type FFA0_R = crate::BitReader<bool>;
#[doc = "Field `FFA0` writer - Filter FIFO assignment for filter 0"]
pub type FFA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA1` reader - Filter FIFO assignment for filter 1"]
pub type FFA1_R = crate::BitReader<bool>;
#[doc = "Field `FFA1` writer - Filter FIFO assignment for filter 1"]
pub type FFA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA2` reader - Filter FIFO assignment for filter 2"]
pub type FFA2_R = crate::BitReader<bool>;
#[doc = "Field `FFA2` writer - Filter FIFO assignment for filter 2"]
pub type FFA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA3` reader - Filter FIFO assignment for filter 3"]
pub type FFA3_R = crate::BitReader<bool>;
#[doc = "Field `FFA3` writer - Filter FIFO assignment for filter 3"]
pub type FFA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA4` reader - Filter FIFO assignment for filter 4"]
pub type FFA4_R = crate::BitReader<bool>;
#[doc = "Field `FFA4` writer - Filter FIFO assignment for filter 4"]
pub type FFA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA5` reader - Filter FIFO assignment for filter 5"]
pub type FFA5_R = crate::BitReader<bool>;
#[doc = "Field `FFA5` writer - Filter FIFO assignment for filter 5"]
pub type FFA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA6` reader - Filter FIFO assignment for filter 6"]
pub type FFA6_R = crate::BitReader<bool>;
#[doc = "Field `FFA6` writer - Filter FIFO assignment for filter 6"]
pub type FFA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA7` reader - Filter FIFO assignment for filter 7"]
pub type FFA7_R = crate::BitReader<bool>;
#[doc = "Field `FFA7` writer - Filter FIFO assignment for filter 7"]
pub type FFA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA8` reader - Filter FIFO assignment for filter 8"]
pub type FFA8_R = crate::BitReader<bool>;
#[doc = "Field `FFA8` writer - Filter FIFO assignment for filter 8"]
pub type FFA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA9` reader - Filter FIFO assignment for filter 9"]
pub type FFA9_R = crate::BitReader<bool>;
#[doc = "Field `FFA9` writer - Filter FIFO assignment for filter 9"]
pub type FFA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA10` reader - Filter FIFO assignment for filter 10"]
pub type FFA10_R = crate::BitReader<bool>;
#[doc = "Field `FFA10` writer - Filter FIFO assignment for filter 10"]
pub type FFA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA11` reader - Filter FIFO assignment for filter 11"]
pub type FFA11_R = crate::BitReader<bool>;
#[doc = "Field `FFA11` writer - Filter FIFO assignment for filter 11"]
pub type FFA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA12` reader - Filter FIFO assignment for filter 12"]
pub type FFA12_R = crate::BitReader<bool>;
#[doc = "Field `FFA12` writer - Filter FIFO assignment for filter 12"]
pub type FFA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA13` reader - Filter FIFO assignment for filter 13"]
pub type FFA13_R = crate::BitReader<bool>;
#[doc = "Field `FFA13` writer - Filter FIFO assignment for filter 13"]
pub type FFA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA14` reader - Filter FIFO assignment for filter 14"]
pub type FFA14_R = crate::BitReader<bool>;
#[doc = "Field `FFA14` writer - Filter FIFO assignment for filter 14"]
pub type FFA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA15` reader - Filter FIFO assignment for filter 15"]
pub type FFA15_R = crate::BitReader<bool>;
#[doc = "Field `FFA15` writer - Filter FIFO assignment for filter 15"]
pub type FFA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA16` reader - Filter FIFO assignment for filter 16"]
pub type FFA16_R = crate::BitReader<bool>;
#[doc = "Field `FFA16` writer - Filter FIFO assignment for filter 16"]
pub type FFA16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA17` reader - Filter FIFO assignment for filter 17"]
pub type FFA17_R = crate::BitReader<bool>;
#[doc = "Field `FFA17` writer - Filter FIFO assignment for filter 17"]
pub type FFA17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA18` reader - Filter FIFO assignment for filter 18"]
pub type FFA18_R = crate::BitReader<bool>;
#[doc = "Field `FFA18` writer - Filter FIFO assignment for filter 18"]
pub type FFA18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA19` reader - Filter FIFO assignment for filter 19"]
pub type FFA19_R = crate::BitReader<bool>;
#[doc = "Field `FFA19` writer - Filter FIFO assignment for filter 19"]
pub type FFA19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA20` reader - Filter FIFO assignment for filter 20"]
pub type FFA20_R = crate::BitReader<bool>;
#[doc = "Field `FFA20` writer - Filter FIFO assignment for filter 20"]
pub type FFA20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA21` reader - Filter FIFO assignment for filter 21"]
pub type FFA21_R = crate::BitReader<bool>;
#[doc = "Field `FFA21` writer - Filter FIFO assignment for filter 21"]
pub type FFA21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA22` reader - Filter FIFO assignment for filter 22"]
pub type FFA22_R = crate::BitReader<bool>;
#[doc = "Field `FFA22` writer - Filter FIFO assignment for filter 22"]
pub type FFA22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA23` reader - Filter FIFO assignment for filter 23"]
pub type FFA23_R = crate::BitReader<bool>;
#[doc = "Field `FFA23` writer - Filter FIFO assignment for filter 23"]
pub type FFA23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA24` reader - Filter FIFO assignment for filter 24"]
pub type FFA24_R = crate::BitReader<bool>;
#[doc = "Field `FFA24` writer - Filter FIFO assignment for filter 24"]
pub type FFA24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA25` reader - Filter FIFO assignment for filter 25"]
pub type FFA25_R = crate::BitReader<bool>;
#[doc = "Field `FFA25` writer - Filter FIFO assignment for filter 25"]
pub type FFA25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA26` reader - Filter FIFO assignment for filter 26"]
pub type FFA26_R = crate::BitReader<bool>;
#[doc = "Field `FFA26` writer - Filter FIFO assignment for filter 26"]
pub type FFA26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
#[doc = "Field `FFA27` reader - Filter FIFO assignment for filter 27"]
pub type FFA27_R = crate::BitReader<bool>;
#[doc = "Field `FFA27` writer - Filter FIFO assignment for filter 27"]
pub type FFA27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFA1R_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> FFA0_R {
        FFA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> FFA1_R {
        FFA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> FFA2_R {
        FFA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> FFA3_R {
        FFA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> FFA4_R {
        FFA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> FFA5_R {
        FFA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> FFA6_R {
        FFA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> FFA7_R {
        FFA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> FFA8_R {
        FFA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> FFA9_R {
        FFA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> FFA10_R {
        FFA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> FFA11_R {
        FFA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> FFA12_R {
        FFA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> FFA13_R {
        FFA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter FIFO assignment for filter 14"]
    #[inline(always)]
    pub fn ffa14(&self) -> FFA14_R {
        FFA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter FIFO assignment for filter 15"]
    #[inline(always)]
    pub fn ffa15(&self) -> FFA15_R {
        FFA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter FIFO assignment for filter 16"]
    #[inline(always)]
    pub fn ffa16(&self) -> FFA16_R {
        FFA16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter FIFO assignment for filter 17"]
    #[inline(always)]
    pub fn ffa17(&self) -> FFA17_R {
        FFA17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter FIFO assignment for filter 18"]
    #[inline(always)]
    pub fn ffa18(&self) -> FFA18_R {
        FFA18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter FIFO assignment for filter 19"]
    #[inline(always)]
    pub fn ffa19(&self) -> FFA19_R {
        FFA19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter FIFO assignment for filter 20"]
    #[inline(always)]
    pub fn ffa20(&self) -> FFA20_R {
        FFA20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter FIFO assignment for filter 21"]
    #[inline(always)]
    pub fn ffa21(&self) -> FFA21_R {
        FFA21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter FIFO assignment for filter 22"]
    #[inline(always)]
    pub fn ffa22(&self) -> FFA22_R {
        FFA22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter FIFO assignment for filter 23"]
    #[inline(always)]
    pub fn ffa23(&self) -> FFA23_R {
        FFA23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter FIFO assignment for filter 24"]
    #[inline(always)]
    pub fn ffa24(&self) -> FFA24_R {
        FFA24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter FIFO assignment for filter 25"]
    #[inline(always)]
    pub fn ffa25(&self) -> FFA25_R {
        FFA25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter FIFO assignment for filter 26"]
    #[inline(always)]
    pub fn ffa26(&self) -> FFA26_R {
        FFA26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter FIFO assignment for filter 27"]
    #[inline(always)]
    pub fn ffa27(&self) -> FFA27_R {
        FFA27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&mut self) -> FFA0_W<0> {
        FFA0_W::new(self)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&mut self) -> FFA1_W<1> {
        FFA1_W::new(self)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&mut self) -> FFA2_W<2> {
        FFA2_W::new(self)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&mut self) -> FFA3_W<3> {
        FFA3_W::new(self)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&mut self) -> FFA4_W<4> {
        FFA4_W::new(self)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&mut self) -> FFA5_W<5> {
        FFA5_W::new(self)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&mut self) -> FFA6_W<6> {
        FFA6_W::new(self)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&mut self) -> FFA7_W<7> {
        FFA7_W::new(self)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&mut self) -> FFA8_W<8> {
        FFA8_W::new(self)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&mut self) -> FFA9_W<9> {
        FFA9_W::new(self)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&mut self) -> FFA10_W<10> {
        FFA10_W::new(self)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&mut self) -> FFA11_W<11> {
        FFA11_W::new(self)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&mut self) -> FFA12_W<12> {
        FFA12_W::new(self)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&mut self) -> FFA13_W<13> {
        FFA13_W::new(self)
    }
    #[doc = "Bit 14 - Filter FIFO assignment for filter 14"]
    #[inline(always)]
    pub fn ffa14(&mut self) -> FFA14_W<14> {
        FFA14_W::new(self)
    }
    #[doc = "Bit 15 - Filter FIFO assignment for filter 15"]
    #[inline(always)]
    pub fn ffa15(&mut self) -> FFA15_W<15> {
        FFA15_W::new(self)
    }
    #[doc = "Bit 16 - Filter FIFO assignment for filter 16"]
    #[inline(always)]
    pub fn ffa16(&mut self) -> FFA16_W<16> {
        FFA16_W::new(self)
    }
    #[doc = "Bit 17 - Filter FIFO assignment for filter 17"]
    #[inline(always)]
    pub fn ffa17(&mut self) -> FFA17_W<17> {
        FFA17_W::new(self)
    }
    #[doc = "Bit 18 - Filter FIFO assignment for filter 18"]
    #[inline(always)]
    pub fn ffa18(&mut self) -> FFA18_W<18> {
        FFA18_W::new(self)
    }
    #[doc = "Bit 19 - Filter FIFO assignment for filter 19"]
    #[inline(always)]
    pub fn ffa19(&mut self) -> FFA19_W<19> {
        FFA19_W::new(self)
    }
    #[doc = "Bit 20 - Filter FIFO assignment for filter 20"]
    #[inline(always)]
    pub fn ffa20(&mut self) -> FFA20_W<20> {
        FFA20_W::new(self)
    }
    #[doc = "Bit 21 - Filter FIFO assignment for filter 21"]
    #[inline(always)]
    pub fn ffa21(&mut self) -> FFA21_W<21> {
        FFA21_W::new(self)
    }
    #[doc = "Bit 22 - Filter FIFO assignment for filter 22"]
    #[inline(always)]
    pub fn ffa22(&mut self) -> FFA22_W<22> {
        FFA22_W::new(self)
    }
    #[doc = "Bit 23 - Filter FIFO assignment for filter 23"]
    #[inline(always)]
    pub fn ffa23(&mut self) -> FFA23_W<23> {
        FFA23_W::new(self)
    }
    #[doc = "Bit 24 - Filter FIFO assignment for filter 24"]
    #[inline(always)]
    pub fn ffa24(&mut self) -> FFA24_W<24> {
        FFA24_W::new(self)
    }
    #[doc = "Bit 25 - Filter FIFO assignment for filter 25"]
    #[inline(always)]
    pub fn ffa25(&mut self) -> FFA25_W<25> {
        FFA25_W::new(self)
    }
    #[doc = "Bit 26 - Filter FIFO assignment for filter 26"]
    #[inline(always)]
    pub fn ffa26(&mut self) -> FFA26_W<26> {
        FFA26_W::new(self)
    }
    #[doc = "Bit 27 - Filter FIFO assignment for filter 27"]
    #[inline(always)]
    pub fn ffa27(&mut self) -> FFA27_W<27> {
        FFA27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter FIFO assignment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffa1r](index.html) module"]
pub struct FFA1R_SPEC;
impl crate::RegisterSpec for FFA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffa1r::R](R) reader structure"]
impl crate::Readable for FFA1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffa1r::W](W) writer structure"]
impl crate::Writable for FFA1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFA1R to value 0"]
impl crate::Resettable for FFA1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
