#[doc = "Register `FS1R` reader"]
pub struct R(crate::R<FS1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS1R` writer"]
pub struct W(crate::W<FS1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS1R_SPEC>;
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
impl From<crate::W<FS1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSC0` reader - Filter scale configuration"]
pub type FSC0_R = crate::BitReader<bool>;
#[doc = "Field `FSC0` writer - Filter scale configuration"]
pub type FSC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC1` reader - Filter scale configuration"]
pub type FSC1_R = crate::BitReader<bool>;
#[doc = "Field `FSC1` writer - Filter scale configuration"]
pub type FSC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC2` reader - Filter scale configuration"]
pub type FSC2_R = crate::BitReader<bool>;
#[doc = "Field `FSC2` writer - Filter scale configuration"]
pub type FSC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC3` reader - Filter scale configuration"]
pub type FSC3_R = crate::BitReader<bool>;
#[doc = "Field `FSC3` writer - Filter scale configuration"]
pub type FSC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC4` reader - Filter scale configuration"]
pub type FSC4_R = crate::BitReader<bool>;
#[doc = "Field `FSC4` writer - Filter scale configuration"]
pub type FSC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC5` reader - Filter scale configuration"]
pub type FSC5_R = crate::BitReader<bool>;
#[doc = "Field `FSC5` writer - Filter scale configuration"]
pub type FSC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC6` reader - Filter scale configuration"]
pub type FSC6_R = crate::BitReader<bool>;
#[doc = "Field `FSC6` writer - Filter scale configuration"]
pub type FSC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC7` reader - Filter scale configuration"]
pub type FSC7_R = crate::BitReader<bool>;
#[doc = "Field `FSC7` writer - Filter scale configuration"]
pub type FSC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC8` reader - Filter scale configuration"]
pub type FSC8_R = crate::BitReader<bool>;
#[doc = "Field `FSC8` writer - Filter scale configuration"]
pub type FSC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC9` reader - Filter scale configuration"]
pub type FSC9_R = crate::BitReader<bool>;
#[doc = "Field `FSC9` writer - Filter scale configuration"]
pub type FSC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC10` reader - Filter scale configuration"]
pub type FSC10_R = crate::BitReader<bool>;
#[doc = "Field `FSC10` writer - Filter scale configuration"]
pub type FSC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC11` reader - Filter scale configuration"]
pub type FSC11_R = crate::BitReader<bool>;
#[doc = "Field `FSC11` writer - Filter scale configuration"]
pub type FSC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC12` reader - Filter scale configuration"]
pub type FSC12_R = crate::BitReader<bool>;
#[doc = "Field `FSC12` writer - Filter scale configuration"]
pub type FSC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC13` reader - Filter scale configuration"]
pub type FSC13_R = crate::BitReader<bool>;
#[doc = "Field `FSC13` writer - Filter scale configuration"]
pub type FSC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC14` reader - Filter scale configuration"]
pub type FSC14_R = crate::BitReader<bool>;
#[doc = "Field `FSC14` writer - Filter scale configuration"]
pub type FSC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC15` reader - Filter scale configuration"]
pub type FSC15_R = crate::BitReader<bool>;
#[doc = "Field `FSC15` writer - Filter scale configuration"]
pub type FSC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC16` reader - Filter scale configuration"]
pub type FSC16_R = crate::BitReader<bool>;
#[doc = "Field `FSC16` writer - Filter scale configuration"]
pub type FSC16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC17` reader - Filter scale configuration"]
pub type FSC17_R = crate::BitReader<bool>;
#[doc = "Field `FSC17` writer - Filter scale configuration"]
pub type FSC17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC18` reader - Filter scale configuration"]
pub type FSC18_R = crate::BitReader<bool>;
#[doc = "Field `FSC18` writer - Filter scale configuration"]
pub type FSC18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC19` reader - Filter scale configuration"]
pub type FSC19_R = crate::BitReader<bool>;
#[doc = "Field `FSC19` writer - Filter scale configuration"]
pub type FSC19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC20` reader - Filter scale configuration"]
pub type FSC20_R = crate::BitReader<bool>;
#[doc = "Field `FSC20` writer - Filter scale configuration"]
pub type FSC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC21` reader - Filter scale configuration"]
pub type FSC21_R = crate::BitReader<bool>;
#[doc = "Field `FSC21` writer - Filter scale configuration"]
pub type FSC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC22` reader - Filter scale configuration"]
pub type FSC22_R = crate::BitReader<bool>;
#[doc = "Field `FSC22` writer - Filter scale configuration"]
pub type FSC22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC23` reader - Filter scale configuration"]
pub type FSC23_R = crate::BitReader<bool>;
#[doc = "Field `FSC23` writer - Filter scale configuration"]
pub type FSC23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC24` reader - Filter scale configuration"]
pub type FSC24_R = crate::BitReader<bool>;
#[doc = "Field `FSC24` writer - Filter scale configuration"]
pub type FSC24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC25` reader - Filter scale configuration"]
pub type FSC25_R = crate::BitReader<bool>;
#[doc = "Field `FSC25` writer - Filter scale configuration"]
pub type FSC25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC26` reader - Filter scale configuration"]
pub type FSC26_R = crate::BitReader<bool>;
#[doc = "Field `FSC26` writer - Filter scale configuration"]
pub type FSC26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
#[doc = "Field `FSC27` reader - Filter scale configuration"]
pub type FSC27_R = crate::BitReader<bool>;
#[doc = "Field `FSC27` writer - Filter scale configuration"]
pub type FSC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS1R_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&self) -> FSC0_R {
        FSC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&self) -> FSC1_R {
        FSC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&self) -> FSC2_R {
        FSC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&self) -> FSC3_R {
        FSC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&self) -> FSC4_R {
        FSC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&self) -> FSC5_R {
        FSC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&self) -> FSC6_R {
        FSC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&self) -> FSC7_R {
        FSC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&self) -> FSC8_R {
        FSC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&self) -> FSC9_R {
        FSC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&self) -> FSC10_R {
        FSC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&self) -> FSC11_R {
        FSC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&self) -> FSC12_R {
        FSC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&self) -> FSC13_R {
        FSC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&self) -> FSC14_R {
        FSC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&self) -> FSC15_R {
        FSC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&self) -> FSC16_R {
        FSC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&self) -> FSC17_R {
        FSC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&self) -> FSC18_R {
        FSC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&self) -> FSC19_R {
        FSC19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&self) -> FSC20_R {
        FSC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&self) -> FSC21_R {
        FSC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&self) -> FSC22_R {
        FSC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&self) -> FSC23_R {
        FSC23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&self) -> FSC24_R {
        FSC24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&self) -> FSC25_R {
        FSC25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&self) -> FSC26_R {
        FSC26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&self) -> FSC27_R {
        FSC27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&mut self) -> FSC0_W<0> {
        FSC0_W::new(self)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&mut self) -> FSC1_W<1> {
        FSC1_W::new(self)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&mut self) -> FSC2_W<2> {
        FSC2_W::new(self)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&mut self) -> FSC3_W<3> {
        FSC3_W::new(self)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&mut self) -> FSC4_W<4> {
        FSC4_W::new(self)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&mut self) -> FSC5_W<5> {
        FSC5_W::new(self)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&mut self) -> FSC6_W<6> {
        FSC6_W::new(self)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&mut self) -> FSC7_W<7> {
        FSC7_W::new(self)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&mut self) -> FSC8_W<8> {
        FSC8_W::new(self)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&mut self) -> FSC9_W<9> {
        FSC9_W::new(self)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&mut self) -> FSC10_W<10> {
        FSC10_W::new(self)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&mut self) -> FSC11_W<11> {
        FSC11_W::new(self)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&mut self) -> FSC12_W<12> {
        FSC12_W::new(self)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&mut self) -> FSC13_W<13> {
        FSC13_W::new(self)
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&mut self) -> FSC14_W<14> {
        FSC14_W::new(self)
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&mut self) -> FSC15_W<15> {
        FSC15_W::new(self)
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&mut self) -> FSC16_W<16> {
        FSC16_W::new(self)
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&mut self) -> FSC17_W<17> {
        FSC17_W::new(self)
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&mut self) -> FSC18_W<18> {
        FSC18_W::new(self)
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&mut self) -> FSC19_W<19> {
        FSC19_W::new(self)
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&mut self) -> FSC20_W<20> {
        FSC20_W::new(self)
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&mut self) -> FSC21_W<21> {
        FSC21_W::new(self)
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&mut self) -> FSC22_W<22> {
        FSC22_W::new(self)
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&mut self) -> FSC23_W<23> {
        FSC23_W::new(self)
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&mut self) -> FSC24_W<24> {
        FSC24_W::new(self)
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&mut self) -> FSC25_W<25> {
        FSC25_W::new(self)
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&mut self) -> FSC26_W<26> {
        FSC26_W::new(self)
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&mut self) -> FSC27_W<27> {
        FSC27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter scale register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs1r](index.html) module"]
pub struct FS1R_SPEC;
impl crate::RegisterSpec for FS1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs1r::R](R) reader structure"]
impl crate::Readable for FS1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs1r::W](W) writer structure"]
impl crate::Writable for FS1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FS1R to value 0"]
impl crate::Resettable for FS1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
