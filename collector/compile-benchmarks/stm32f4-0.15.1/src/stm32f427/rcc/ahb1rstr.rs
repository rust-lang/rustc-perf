#[doc = "Register `AHB1RSTR` reader"]
pub struct R(crate::R<AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1RSTR` writer"]
pub struct W(crate::W<AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RSTR_SPEC>;
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
impl From<crate::W<AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB OTG HS module reset"]
pub use GPIOARST_A as OTGHSRST_A;
#[doc = "Ethernet MAC reset"]
pub use GPIOARST_A as ETHMACRST_A;
#[doc = "DMA2 reset"]
pub use GPIOARST_A as DMA2RST_A;
#[doc = "DMA2 reset"]
pub use GPIOARST_A as DMA1RST_A;
#[doc = "CRC reset"]
pub use GPIOARST_A as CRCRST_A;
#[doc = "IO port I reset"]
pub use GPIOARST_A as GPIOIRST_A;
#[doc = "IO port H reset"]
pub use GPIOARST_A as GPIOHRST_A;
#[doc = "IO port G reset"]
pub use GPIOARST_A as GPIOGRST_A;
#[doc = "IO port F reset"]
pub use GPIOARST_A as GPIOFRST_A;
#[doc = "IO port E reset"]
pub use GPIOARST_A as GPIOERST_A;
#[doc = "IO port D reset"]
pub use GPIOARST_A as GPIODRST_A;
#[doc = "IO port C reset"]
pub use GPIOARST_A as GPIOCRST_A;
#[doc = "IO port B reset"]
pub use GPIOARST_A as GPIOBRST_A;
#[doc = "Field `OTGHSRST` reader - USB OTG HS module reset"]
pub use GPIOARST_R as OTGHSRST_R;
#[doc = "Field `ETHMACRST` reader - Ethernet MAC reset"]
pub use GPIOARST_R as ETHMACRST_R;
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub use GPIOARST_R as DMA2RST_R;
#[doc = "Field `DMA1RST` reader - DMA2 reset"]
pub use GPIOARST_R as DMA1RST_R;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub use GPIOARST_R as CRCRST_R;
#[doc = "Field `GPIOIRST` reader - IO port I reset"]
pub use GPIOARST_R as GPIOIRST_R;
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub use GPIOARST_R as GPIOHRST_R;
#[doc = "Field `GPIOGRST` reader - IO port G reset"]
pub use GPIOARST_R as GPIOGRST_R;
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub use GPIOARST_R as GPIOFRST_R;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub use GPIOARST_R as GPIOERST_R;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub use GPIOARST_R as GPIODRST_R;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub use GPIOARST_R as GPIOCRST_R;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub use GPIOARST_R as GPIOBRST_R;
#[doc = "Field `OTGHSRST` writer - USB OTG HS module reset"]
pub use GPIOARST_W as OTGHSRST_W;
#[doc = "Field `ETHMACRST` writer - Ethernet MAC reset"]
pub use GPIOARST_W as ETHMACRST_W;
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub use GPIOARST_W as DMA2RST_W;
#[doc = "Field `DMA1RST` writer - DMA2 reset"]
pub use GPIOARST_W as DMA1RST_W;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub use GPIOARST_W as CRCRST_W;
#[doc = "Field `GPIOIRST` writer - IO port I reset"]
pub use GPIOARST_W as GPIOIRST_W;
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub use GPIOARST_W as GPIOHRST_W;
#[doc = "Field `GPIOGRST` writer - IO port G reset"]
pub use GPIOARST_W as GPIOGRST_W;
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub use GPIOARST_W as GPIOFRST_W;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub use GPIOARST_W as GPIOERST_W;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub use GPIOARST_W as GPIODRST_W;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub use GPIOARST_W as GPIOCRST_W;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub use GPIOARST_W as GPIOBRST_W;
#[doc = "IO port A reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOARST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GPIOARST_R = crate::BitReader<GPIOARST_A>;
impl GPIOARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIOARST_A> {
        match self.bits {
            true => Some(GPIOARST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST_A::Reset
    }
}
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, GPIOARST_A, O>;
impl<'a, const O: u8> GPIOARST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOARST_A::Reset)
    }
}
#[doc = "DMA2D reset"]
pub use GPIOARST_A as DMA2DRST_A;
#[doc = "IO port J reset"]
pub use GPIOARST_A as GPIOJRST_A;
#[doc = "IO port K reset"]
pub use GPIOARST_A as GPIOKRST_A;
#[doc = "Field `DMA2DRST` reader - DMA2D reset"]
pub use GPIOARST_R as DMA2DRST_R;
#[doc = "Field `GPIOJRST` reader - IO port J reset"]
pub use GPIOARST_R as GPIOJRST_R;
#[doc = "Field `GPIOKRST` reader - IO port K reset"]
pub use GPIOARST_R as GPIOKRST_R;
#[doc = "Field `DMA2DRST` writer - DMA2D reset"]
pub use GPIOARST_W as DMA2DRST_W;
#[doc = "Field `GPIOJRST` writer - IO port J reset"]
pub use GPIOARST_W as GPIOJRST_W;
#[doc = "Field `GPIOKRST` writer - IO port K reset"]
pub use GPIOARST_W as GPIOKRST_W;
impl R {
    #[doc = "Bit 29 - USB OTG HS module reset"]
    #[inline(always)]
    pub fn otghsrst(&self) -> OTGHSRST_R {
        OTGHSRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet MAC reset"]
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA2 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 23 - DMA2D reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 9 - IO port J reset"]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IO port K reset"]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - USB OTG HS module reset"]
    #[inline(always)]
    pub fn otghsrst(&mut self) -> OTGHSRST_W<29> {
        OTGHSRST_W::new(self)
    }
    #[doc = "Bit 25 - Ethernet MAC reset"]
    #[inline(always)]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<25> {
        ETHMACRST_W::new(self)
    }
    #[doc = "Bit 22 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<22> {
        DMA2RST_W::new(self)
    }
    #[doc = "Bit 21 - DMA2 reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<21> {
        DMA1RST_W::new(self)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    #[doc = "Bit 8 - IO port I reset"]
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<8> {
        GPIOIRST_W::new(self)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    #[doc = "Bit 23 - DMA2D reset"]
    #[inline(always)]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<23> {
        DMA2DRST_W::new(self)
    }
    #[doc = "Bit 9 - IO port J reset"]
    #[inline(always)]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<9> {
        GPIOJRST_W::new(self)
    }
    #[doc = "Bit 10 - IO port K reset"]
    #[inline(always)]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<10> {
        GPIOKRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](index.html) module"]
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1rstr::R](R) reader structure"]
impl crate::Readable for AHB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](W) writer structure"]
impl crate::Writable for AHB1RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
