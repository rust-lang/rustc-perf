#[doc = "Register `AHB1ENR` reader"]
pub struct R(crate::R<AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1ENR` writer"]
pub struct W(crate::W<AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1ENR_SPEC>;
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
impl From<crate::W<AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB OTG HSULPI clock enable"]
pub use GPIOAEN_A as OTGHSULPIEN_A;
#[doc = "USB OTG HS clock enable"]
pub use GPIOAEN_A as OTGHSEN_A;
#[doc = "DMA2 clock enable"]
pub use GPIOAEN_A as DMA2EN_A;
#[doc = "DMA1 clock enable"]
pub use GPIOAEN_A as DMA1EN_A;
#[doc = "Backup SRAM interface clock enable"]
pub use GPIOAEN_A as BKPSRAMEN_A;
#[doc = "CRC clock enable"]
pub use GPIOAEN_A as CRCEN_A;
#[doc = "IO port H clock enable"]
pub use GPIOAEN_A as GPIOHEN_A;
#[doc = "IO port G clock enable"]
pub use GPIOAEN_A as GPIOGEN_A;
#[doc = "IO port F clock enable"]
pub use GPIOAEN_A as GPIOFEN_A;
#[doc = "IO port E clock enable"]
pub use GPIOAEN_A as GPIOEEN_A;
#[doc = "IO port D clock enable"]
pub use GPIOAEN_A as GPIODEN_A;
#[doc = "IO port C clock enable"]
pub use GPIOAEN_A as GPIOCEN_A;
#[doc = "IO port B clock enable"]
pub use GPIOAEN_A as GPIOBEN_A;
#[doc = "Field `OTGHSULPIEN` reader - USB OTG HSULPI clock enable"]
pub use GPIOAEN_R as OTGHSULPIEN_R;
#[doc = "Field `OTGHSEN` reader - USB OTG HS clock enable"]
pub use GPIOAEN_R as OTGHSEN_R;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use GPIOAEN_R as DMA2EN_R;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub use GPIOAEN_R as DMA1EN_R;
#[doc = "Field `BKPSRAMEN` reader - Backup SRAM interface clock enable"]
pub use GPIOAEN_R as BKPSRAMEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use GPIOAEN_R as CRCEN_R;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `OTGHSULPIEN` writer - USB OTG HSULPI clock enable"]
pub use GPIOAEN_W as OTGHSULPIEN_W;
#[doc = "Field `OTGHSEN` writer - USB OTG HS clock enable"]
pub use GPIOAEN_W as OTGHSEN_W;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use GPIOAEN_W as DMA2EN_W;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub use GPIOAEN_W as DMA1EN_W;
#[doc = "Field `BKPSRAMEN` writer - Backup SRAM interface clock enable"]
pub use GPIOAEN_W as BKPSRAMEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use GPIOAEN_W as CRCEN_W;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOAEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::Disabled,
            true => GPIOAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN_A::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, GPIOAEN_A, O>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline(always)]
    pub fn otghsulpien(&self) -> OTGHSULPIEN_R {
        OTGHSULPIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline(always)]
    pub fn otghsen(&self) -> OTGHSEN_R {
        OTGHSEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline(always)]
    pub fn otghsulpien(&mut self) -> OTGHSULPIEN_W<30> {
        OTGHSULPIEN_W::new(self)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline(always)]
    pub fn otghsen(&mut self) -> OTGHSEN_W<29> {
        OTGHSEN_W::new(self)
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<22> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<21> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<18> {
        BKPSRAMEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](index.html) module"]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1enr::R](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x0010_0000"]
impl crate::Resettable for AHB1ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
