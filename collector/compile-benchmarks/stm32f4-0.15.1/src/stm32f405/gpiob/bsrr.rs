#[doc = "Register `BSRR` writer"]
pub struct W(crate::W<BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSRR_SPEC>;
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
impl From<crate::W<BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR15_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR14_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR13_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR12_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR11_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR10_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR9_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR8_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR7_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR6_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR5_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR4_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR3_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR2_AW;
#[doc = "Port x reset bit y (y = 0..15)"]
pub use BR0_AW as BR1_AW;
#[doc = "Field `BR15` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR15_W;
#[doc = "Field `BR14` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR14_W;
#[doc = "Field `BR13` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR13_W;
#[doc = "Field `BR12` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR12_W;
#[doc = "Field `BR11` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR11_W;
#[doc = "Field `BR10` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR10_W;
#[doc = "Field `BR9` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR9_W;
#[doc = "Field `BR8` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR8_W;
#[doc = "Field `BR7` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR7_W;
#[doc = "Field `BR6` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR6_W;
#[doc = "Field `BR5` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR5_W;
#[doc = "Field `BR4` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR4_W;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR3_W;
#[doc = "Field `BR2` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR2_W;
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub use BR0_W as BR1_W;
#[doc = "Port x set bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR0_AW {
    #[doc = "1: Resets the corresponding ODRx bit"]
    Reset = 1,
}
impl From<BR0_AW> for bool {
    #[inline(always)]
    fn from(variant: BR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BR0_AW, O>;
impl<'a, const O: u8> BR0_W<'a, O> {
    #[doc = "Resets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0_AW::Reset)
    }
}
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS15_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS14_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS13_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS12_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS11_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS10_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS9_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS8_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS7_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS6_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS5_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS4_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS3_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS2_AW;
#[doc = "Port x set bit y (y= 0..15)"]
pub use BS0_AW as BS1_AW;
#[doc = "Field `BS15` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS15_W;
#[doc = "Field `BS14` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS14_W;
#[doc = "Field `BS13` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS13_W;
#[doc = "Field `BS12` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS12_W;
#[doc = "Field `BS11` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS11_W;
#[doc = "Field `BS10` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS10_W;
#[doc = "Field `BS9` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS9_W;
#[doc = "Field `BS8` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS8_W;
#[doc = "Field `BS7` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS7_W;
#[doc = "Field `BS6` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS6_W;
#[doc = "Field `BS5` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS5_W;
#[doc = "Field `BS4` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS4_W;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS3_W;
#[doc = "Field `BS2` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS2_W;
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub use BS0_W as BS1_W;
#[doc = "Port x set bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS0_AW {
    #[doc = "1: Sets the corresponding ODRx bit"]
    Set = 1,
}
impl From<BS0_AW> for bool {
    #[inline(always)]
    fn from(variant: BS0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type BS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BS0_AW, O>;
impl<'a, const O: u8> BS0_W<'a, O> {
    #[doc = "Sets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BS0_AW::Set)
    }
}
impl W {
    #[doc = "Bit 31 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<31> {
        BR15_W::new(self)
    }
    #[doc = "Bit 30 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<30> {
        BR14_W::new(self)
    }
    #[doc = "Bit 29 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<29> {
        BR13_W::new(self)
    }
    #[doc = "Bit 28 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<28> {
        BR12_W::new(self)
    }
    #[doc = "Bit 27 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<27> {
        BR11_W::new(self)
    }
    #[doc = "Bit 26 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<26> {
        BR10_W::new(self)
    }
    #[doc = "Bit 25 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<25> {
        BR9_W::new(self)
    }
    #[doc = "Bit 24 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<24> {
        BR8_W::new(self)
    }
    #[doc = "Bit 23 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<23> {
        BR7_W::new(self)
    }
    #[doc = "Bit 22 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<22> {
        BR6_W::new(self)
    }
    #[doc = "Bit 21 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<21> {
        BR5_W::new(self)
    }
    #[doc = "Bit 20 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<20> {
        BR4_W::new(self)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<19> {
        BR3_W::new(self)
    }
    #[doc = "Bit 18 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<18> {
        BR2_W::new(self)
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<17> {
        BR1_W::new(self)
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<16> {
        BR0_W::new(self)
    }
    #[doc = "Bit 15 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs15(&mut self) -> BS15_W<15> {
        BS15_W::new(self)
    }
    #[doc = "Bit 14 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs14(&mut self) -> BS14_W<14> {
        BS14_W::new(self)
    }
    #[doc = "Bit 13 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs13(&mut self) -> BS13_W<13> {
        BS13_W::new(self)
    }
    #[doc = "Bit 12 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs12(&mut self) -> BS12_W<12> {
        BS12_W::new(self)
    }
    #[doc = "Bit 11 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs11(&mut self) -> BS11_W<11> {
        BS11_W::new(self)
    }
    #[doc = "Bit 10 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs10(&mut self) -> BS10_W<10> {
        BS10_W::new(self)
    }
    #[doc = "Bit 9 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs9(&mut self) -> BS9_W<9> {
        BS9_W::new(self)
    }
    #[doc = "Bit 8 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs8(&mut self) -> BS8_W<8> {
        BS8_W::new(self)
    }
    #[doc = "Bit 7 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs7(&mut self) -> BS7_W<7> {
        BS7_W::new(self)
    }
    #[doc = "Bit 6 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs6(&mut self) -> BS6_W<6> {
        BS6_W::new(self)
    }
    #[doc = "Bit 5 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs5(&mut self) -> BS5_W<5> {
        BS5_W::new(self)
    }
    #[doc = "Bit 4 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs4(&mut self) -> BS4_W<4> {
        BS4_W::new(self)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W<3> {
        BS3_W::new(self)
    }
    #[doc = "Bit 2 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W<2> {
        BS2_W::new(self)
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W<1> {
        BS1_W::new(self)
    }
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W<0> {
        BS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](index.html) module"]
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bsrr::W](W) writer structure"]
impl crate::Writable for BSRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
