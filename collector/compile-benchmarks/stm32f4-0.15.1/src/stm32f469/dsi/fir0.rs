#[doc = "Register `FIR0` reader"]
pub struct R(crate::R<FIR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIR0` writer"]
pub struct W(crate::W<FIR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIR0_SPEC>;
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
impl From<crate::W<FIR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPE4` reader - Force PHY Error 4"]
pub type FPE4_R = crate::BitReader<bool>;
#[doc = "Field `FPE4` writer - Force PHY Error 4"]
pub type FPE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE3` reader - Force PHY Error 3"]
pub type FPE3_R = crate::BitReader<bool>;
#[doc = "Field `FPE3` writer - Force PHY Error 3"]
pub type FPE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE2` reader - Force PHY Error 2"]
pub type FPE2_R = crate::BitReader<bool>;
#[doc = "Field `FPE2` writer - Force PHY Error 2"]
pub type FPE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE1` reader - Force PHY Error 1"]
pub type FPE1_R = crate::BitReader<bool>;
#[doc = "Field `FPE1` writer - Force PHY Error 1"]
pub type FPE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FPE0` reader - Force PHY Error 0"]
pub type FPE0_R = crate::BitReader<bool>;
#[doc = "Field `FPE0` writer - Force PHY Error 0"]
pub type FPE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE15` reader - Force Acknowledge Error 15"]
pub type FAE15_R = crate::BitReader<bool>;
#[doc = "Field `FAE15` writer - Force Acknowledge Error 15"]
pub type FAE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE14` reader - Force Acknowledge Error 14"]
pub type FAE14_R = crate::BitReader<bool>;
#[doc = "Field `FAE14` writer - Force Acknowledge Error 14"]
pub type FAE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE13` reader - Force Acknowledge Error 13"]
pub type FAE13_R = crate::BitReader<bool>;
#[doc = "Field `FAE13` writer - Force Acknowledge Error 13"]
pub type FAE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE12` reader - Force Acknowledge Error 12"]
pub type FAE12_R = crate::BitReader<bool>;
#[doc = "Field `FAE12` writer - Force Acknowledge Error 12"]
pub type FAE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE11` reader - Force Acknowledge Error 11"]
pub type FAE11_R = crate::BitReader<bool>;
#[doc = "Field `FAE11` writer - Force Acknowledge Error 11"]
pub type FAE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE10` reader - Force Acknowledge Error 10"]
pub type FAE10_R = crate::BitReader<bool>;
#[doc = "Field `FAE10` writer - Force Acknowledge Error 10"]
pub type FAE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE9` reader - Force Acknowledge Error 9"]
pub type FAE9_R = crate::BitReader<bool>;
#[doc = "Field `FAE9` writer - Force Acknowledge Error 9"]
pub type FAE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE8` reader - Force Acknowledge Error 8"]
pub type FAE8_R = crate::BitReader<bool>;
#[doc = "Field `FAE8` writer - Force Acknowledge Error 8"]
pub type FAE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE7` reader - Force Acknowledge Error 7"]
pub type FAE7_R = crate::BitReader<bool>;
#[doc = "Field `FAE7` writer - Force Acknowledge Error 7"]
pub type FAE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE6` reader - Force Acknowledge Error 6"]
pub type FAE6_R = crate::BitReader<bool>;
#[doc = "Field `FAE6` writer - Force Acknowledge Error 6"]
pub type FAE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE5` reader - Force Acknowledge Error 5"]
pub type FAE5_R = crate::BitReader<bool>;
#[doc = "Field `FAE5` writer - Force Acknowledge Error 5"]
pub type FAE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE4` reader - Force Acknowledge Error 4"]
pub type FAE4_R = crate::BitReader<bool>;
#[doc = "Field `FAE4` writer - Force Acknowledge Error 4"]
pub type FAE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE3` reader - Force Acknowledge Error 3"]
pub type FAE3_R = crate::BitReader<bool>;
#[doc = "Field `FAE3` writer - Force Acknowledge Error 3"]
pub type FAE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE2` reader - Force Acknowledge Error 2"]
pub type FAE2_R = crate::BitReader<bool>;
#[doc = "Field `FAE2` writer - Force Acknowledge Error 2"]
pub type FAE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE1` reader - Force Acknowledge Error 1"]
pub type FAE1_R = crate::BitReader<bool>;
#[doc = "Field `FAE1` writer - Force Acknowledge Error 1"]
pub type FAE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
#[doc = "Field `FAE0` reader - Force Acknowledge Error 0"]
pub type FAE0_R = crate::BitReader<bool>;
#[doc = "Field `FAE0` writer - Force Acknowledge Error 0"]
pub type FAE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 20 - Force PHY Error 4"]
    #[inline(always)]
    pub fn fpe4(&self) -> FPE4_R {
        FPE4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Force PHY Error 3"]
    #[inline(always)]
    pub fn fpe3(&self) -> FPE3_R {
        FPE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Force PHY Error 2"]
    #[inline(always)]
    pub fn fpe2(&self) -> FPE2_R {
        FPE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Force PHY Error 1"]
    #[inline(always)]
    pub fn fpe1(&self) -> FPE1_R {
        FPE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Force PHY Error 0"]
    #[inline(always)]
    pub fn fpe0(&self) -> FPE0_R {
        FPE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Force Acknowledge Error 15"]
    #[inline(always)]
    pub fn fae15(&self) -> FAE15_R {
        FAE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Force Acknowledge Error 14"]
    #[inline(always)]
    pub fn fae14(&self) -> FAE14_R {
        FAE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Acknowledge Error 13"]
    #[inline(always)]
    pub fn fae13(&self) -> FAE13_R {
        FAE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Force Acknowledge Error 12"]
    #[inline(always)]
    pub fn fae12(&self) -> FAE12_R {
        FAE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Force Acknowledge Error 11"]
    #[inline(always)]
    pub fn fae11(&self) -> FAE11_R {
        FAE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Force Acknowledge Error 10"]
    #[inline(always)]
    pub fn fae10(&self) -> FAE10_R {
        FAE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Force Acknowledge Error 9"]
    #[inline(always)]
    pub fn fae9(&self) -> FAE9_R {
        FAE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Force Acknowledge Error 8"]
    #[inline(always)]
    pub fn fae8(&self) -> FAE8_R {
        FAE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Acknowledge Error 7"]
    #[inline(always)]
    pub fn fae7(&self) -> FAE7_R {
        FAE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Acknowledge Error 6"]
    #[inline(always)]
    pub fn fae6(&self) -> FAE6_R {
        FAE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Acknowledge Error 5"]
    #[inline(always)]
    pub fn fae5(&self) -> FAE5_R {
        FAE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Acknowledge Error 4"]
    #[inline(always)]
    pub fn fae4(&self) -> FAE4_R {
        FAE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Acknowledge Error 3"]
    #[inline(always)]
    pub fn fae3(&self) -> FAE3_R {
        FAE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Acknowledge Error 2"]
    #[inline(always)]
    pub fn fae2(&self) -> FAE2_R {
        FAE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Force Acknowledge Error 1"]
    #[inline(always)]
    pub fn fae1(&self) -> FAE1_R {
        FAE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Force Acknowledge Error 0"]
    #[inline(always)]
    pub fn fae0(&self) -> FAE0_R {
        FAE0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Force PHY Error 4"]
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE4_W<20> {
        FPE4_W::new(self)
    }
    #[doc = "Bit 19 - Force PHY Error 3"]
    #[inline(always)]
    pub fn fpe3(&mut self) -> FPE3_W<19> {
        FPE3_W::new(self)
    }
    #[doc = "Bit 18 - Force PHY Error 2"]
    #[inline(always)]
    pub fn fpe2(&mut self) -> FPE2_W<18> {
        FPE2_W::new(self)
    }
    #[doc = "Bit 17 - Force PHY Error 1"]
    #[inline(always)]
    pub fn fpe1(&mut self) -> FPE1_W<17> {
        FPE1_W::new(self)
    }
    #[doc = "Bit 16 - Force PHY Error 0"]
    #[inline(always)]
    pub fn fpe0(&mut self) -> FPE0_W<16> {
        FPE0_W::new(self)
    }
    #[doc = "Bit 15 - Force Acknowledge Error 15"]
    #[inline(always)]
    pub fn fae15(&mut self) -> FAE15_W<15> {
        FAE15_W::new(self)
    }
    #[doc = "Bit 14 - Force Acknowledge Error 14"]
    #[inline(always)]
    pub fn fae14(&mut self) -> FAE14_W<14> {
        FAE14_W::new(self)
    }
    #[doc = "Bit 13 - Force Acknowledge Error 13"]
    #[inline(always)]
    pub fn fae13(&mut self) -> FAE13_W<13> {
        FAE13_W::new(self)
    }
    #[doc = "Bit 12 - Force Acknowledge Error 12"]
    #[inline(always)]
    pub fn fae12(&mut self) -> FAE12_W<12> {
        FAE12_W::new(self)
    }
    #[doc = "Bit 11 - Force Acknowledge Error 11"]
    #[inline(always)]
    pub fn fae11(&mut self) -> FAE11_W<11> {
        FAE11_W::new(self)
    }
    #[doc = "Bit 10 - Force Acknowledge Error 10"]
    #[inline(always)]
    pub fn fae10(&mut self) -> FAE10_W<10> {
        FAE10_W::new(self)
    }
    #[doc = "Bit 9 - Force Acknowledge Error 9"]
    #[inline(always)]
    pub fn fae9(&mut self) -> FAE9_W<9> {
        FAE9_W::new(self)
    }
    #[doc = "Bit 8 - Force Acknowledge Error 8"]
    #[inline(always)]
    pub fn fae8(&mut self) -> FAE8_W<8> {
        FAE8_W::new(self)
    }
    #[doc = "Bit 7 - Force Acknowledge Error 7"]
    #[inline(always)]
    pub fn fae7(&mut self) -> FAE7_W<7> {
        FAE7_W::new(self)
    }
    #[doc = "Bit 6 - Force Acknowledge Error 6"]
    #[inline(always)]
    pub fn fae6(&mut self) -> FAE6_W<6> {
        FAE6_W::new(self)
    }
    #[doc = "Bit 5 - Force Acknowledge Error 5"]
    #[inline(always)]
    pub fn fae5(&mut self) -> FAE5_W<5> {
        FAE5_W::new(self)
    }
    #[doc = "Bit 4 - Force Acknowledge Error 4"]
    #[inline(always)]
    pub fn fae4(&mut self) -> FAE4_W<4> {
        FAE4_W::new(self)
    }
    #[doc = "Bit 3 - Force Acknowledge Error 3"]
    #[inline(always)]
    pub fn fae3(&mut self) -> FAE3_W<3> {
        FAE3_W::new(self)
    }
    #[doc = "Bit 2 - Force Acknowledge Error 2"]
    #[inline(always)]
    pub fn fae2(&mut self) -> FAE2_W<2> {
        FAE2_W::new(self)
    }
    #[doc = "Bit 1 - Force Acknowledge Error 1"]
    #[inline(always)]
    pub fn fae1(&mut self) -> FAE1_W<1> {
        FAE1_W::new(self)
    }
    #[doc = "Bit 0 - Force Acknowledge Error 0"]
    #[inline(always)]
    pub fn fae0(&mut self) -> FAE0_W<0> {
        FAE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Force Interrupt Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir0](index.html) module"]
pub struct FIR0_SPEC;
impl crate::RegisterSpec for FIR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fir0::R](R) reader structure"]
impl crate::Readable for FIR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fir0::W](W) writer structure"]
impl crate::Writable for FIR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIR0 to value 0"]
impl crate::Resettable for FIR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
