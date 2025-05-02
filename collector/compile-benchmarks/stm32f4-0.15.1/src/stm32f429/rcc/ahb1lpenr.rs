#[doc = "Register `AHB1LPENR` reader"]
pub struct R(crate::R<AHB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1LPENR` writer"]
pub struct W(crate::W<AHB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1LPENR_SPEC>;
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
impl From<crate::W<AHB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IO port A clock enable during sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOALPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<GPIOALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOALPEN` reader - IO port A clock enable during sleep mode"]
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN_A>;
impl GPIOALPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOALPEN_A {
        match self.bits {
            false => GPIOALPEN_A::DisabledInSleep,
            true => GPIOALPEN_A::EnabledInSleep,
        }
    }
    #[doc = "Checks if the value of the field is `DisabledInSleep`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN_A::DisabledInSleep
    }
    #[doc = "Checks if the value of the field is `EnabledInSleep`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == GPIOALPEN_A::EnabledInSleep
    }
}
#[doc = "Field `GPIOALPEN` writer - IO port A clock enable during sleep mode"]
pub type GPIOALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, GPIOALPEN_A, O>;
impl<'a, const O: u8> GPIOALPEN_W<'a, O> {
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::EnabledInSleep)
    }
}
#[doc = "IO port B clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOBLPEN_A;
#[doc = "IO port C clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOCLPEN_A;
#[doc = "IO port D clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIODLPEN_A;
#[doc = "IO port E clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOELPEN_A;
#[doc = "IO port F clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOFLPEN_A;
#[doc = "IO port G clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOGLPEN_A;
#[doc = "IO port H clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOHLPEN_A;
#[doc = "IO port I clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOILPEN_A;
#[doc = "CRC clock enable during Sleep mode"]
pub use GPIOALPEN_A as CRCLPEN_A;
#[doc = "Flash interface clock enable during Sleep mode"]
pub use GPIOALPEN_A as FLITFLPEN_A;
#[doc = "SRAM 1interface clock enable during Sleep mode"]
pub use GPIOALPEN_A as SRAM1LPEN_A;
#[doc = "SRAM 2 interface clock enable during Sleep mode"]
pub use GPIOALPEN_A as SRAM2LPEN_A;
#[doc = "Backup SRAM interface clock enable during Sleep mode"]
pub use GPIOALPEN_A as BKPSRAMLPEN_A;
#[doc = "DMA1 clock enable during Sleep mode"]
pub use GPIOALPEN_A as DMA1LPEN_A;
#[doc = "DMA2 clock enable during Sleep mode"]
pub use GPIOALPEN_A as DMA2LPEN_A;
#[doc = "Ethernet MAC clock enable during Sleep mode"]
pub use GPIOALPEN_A as ETHMACLPEN_A;
#[doc = "Ethernet transmission clock enable during Sleep mode"]
pub use GPIOALPEN_A as ETHMACTXLPEN_A;
#[doc = "Ethernet reception clock enable during Sleep mode"]
pub use GPIOALPEN_A as ETHMACRXLPEN_A;
#[doc = "Ethernet PTP clock enable during Sleep mode"]
pub use GPIOALPEN_A as ETHMACPTPLPEN_A;
#[doc = "USB OTG HS clock enable during Sleep mode"]
pub use GPIOALPEN_A as OTGHSLPEN_A;
#[doc = "USB OTG HS ULPI clock enable during Sleep mode"]
pub use GPIOALPEN_A as OTGHSULPILPEN_A;
#[doc = "SRAM3 interface clock enable during Sleep mode"]
pub use GPIOALPEN_A as SRAM3LPEN_A;
#[doc = "DMA2D clock enable during Sleep mode"]
pub use GPIOALPEN_A as DMA2DLPEN_A;
#[doc = "IO port J clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOJLPEN_A;
#[doc = "IO port K clock enable during Sleep mode"]
pub use GPIOALPEN_A as GPIOKLPEN_A;
#[doc = "Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOBLPEN_R;
#[doc = "Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOCLPEN_R;
#[doc = "Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIODLPEN_R;
#[doc = "Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOELPEN_R;
#[doc = "Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOFLPEN_R;
#[doc = "Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOGLPEN_R;
#[doc = "Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOHLPEN_R;
#[doc = "Field `GPIOILPEN` reader - IO port I clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOILPEN_R;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during Sleep mode"]
pub use GPIOALPEN_R as CRCLPEN_R;
#[doc = "Field `FLITFLPEN` reader - Flash interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as FLITFLPEN_R;
#[doc = "Field `SRAM1LPEN` reader - SRAM 1interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as SRAM1LPEN_R;
#[doc = "Field `SRAM2LPEN` reader - SRAM 2 interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as SRAM2LPEN_R;
#[doc = "Field `BKPSRAMLPEN` reader - Backup SRAM interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as BKPSRAMLPEN_R;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode"]
pub use GPIOALPEN_R as DMA1LPEN_R;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode"]
pub use GPIOALPEN_R as DMA2LPEN_R;
#[doc = "Field `ETHMACLPEN` reader - Ethernet MAC clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACLPEN_R;
#[doc = "Field `ETHMACTXLPEN` reader - Ethernet transmission clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACTXLPEN_R;
#[doc = "Field `ETHMACRXLPEN` reader - Ethernet reception clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACRXLPEN_R;
#[doc = "Field `ETHMACPTPLPEN` reader - Ethernet PTP clock enable during Sleep mode"]
pub use GPIOALPEN_R as ETHMACPTPLPEN_R;
#[doc = "Field `OTGHSLPEN` reader - USB OTG HS clock enable during Sleep mode"]
pub use GPIOALPEN_R as OTGHSLPEN_R;
#[doc = "Field `OTGHSULPILPEN` reader - USB OTG HS ULPI clock enable during Sleep mode"]
pub use GPIOALPEN_R as OTGHSULPILPEN_R;
#[doc = "Field `SRAM3LPEN` reader - SRAM3 interface clock enable during Sleep mode"]
pub use GPIOALPEN_R as SRAM3LPEN_R;
#[doc = "Field `DMA2DLPEN` reader - DMA2D clock enable during Sleep mode"]
pub use GPIOALPEN_R as DMA2DLPEN_R;
#[doc = "Field `GPIOJLPEN` reader - IO port J clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOJLPEN_R;
#[doc = "Field `GPIOKLPEN` reader - IO port K clock enable during Sleep mode"]
pub use GPIOALPEN_R as GPIOKLPEN_R;
#[doc = "Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOBLPEN_W;
#[doc = "Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOCLPEN_W;
#[doc = "Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIODLPEN_W;
#[doc = "Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOELPEN_W;
#[doc = "Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOFLPEN_W;
#[doc = "Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOGLPEN_W;
#[doc = "Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOHLPEN_W;
#[doc = "Field `GPIOILPEN` writer - IO port I clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOILPEN_W;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during Sleep mode"]
pub use GPIOALPEN_W as CRCLPEN_W;
#[doc = "Field `FLITFLPEN` writer - Flash interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as FLITFLPEN_W;
#[doc = "Field `SRAM1LPEN` writer - SRAM 1interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as SRAM1LPEN_W;
#[doc = "Field `SRAM2LPEN` writer - SRAM 2 interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as SRAM2LPEN_W;
#[doc = "Field `BKPSRAMLPEN` writer - Backup SRAM interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as BKPSRAMLPEN_W;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode"]
pub use GPIOALPEN_W as DMA1LPEN_W;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode"]
pub use GPIOALPEN_W as DMA2LPEN_W;
#[doc = "Field `ETHMACLPEN` writer - Ethernet MAC clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACLPEN_W;
#[doc = "Field `ETHMACTXLPEN` writer - Ethernet transmission clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACTXLPEN_W;
#[doc = "Field `ETHMACRXLPEN` writer - Ethernet reception clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACRXLPEN_W;
#[doc = "Field `ETHMACPTPLPEN` writer - Ethernet PTP clock enable during Sleep mode"]
pub use GPIOALPEN_W as ETHMACPTPLPEN_W;
#[doc = "Field `OTGHSLPEN` writer - USB OTG HS clock enable during Sleep mode"]
pub use GPIOALPEN_W as OTGHSLPEN_W;
#[doc = "Field `OTGHSULPILPEN` writer - USB OTG HS ULPI clock enable during Sleep mode"]
pub use GPIOALPEN_W as OTGHSULPILPEN_W;
#[doc = "Field `SRAM3LPEN` writer - SRAM3 interface clock enable during Sleep mode"]
pub use GPIOALPEN_W as SRAM3LPEN_W;
#[doc = "Field `DMA2DLPEN` writer - DMA2D clock enable during Sleep mode"]
pub use GPIOALPEN_W as DMA2DLPEN_W;
#[doc = "Field `GPIOJLPEN` writer - IO port J clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOJLPEN_W;
#[doc = "Field `GPIOKLPEN` writer - IO port K clock enable during Sleep mode"]
pub use GPIOALPEN_W as GPIOKLPEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmactxlpen(&self) -> ETHMACTXLPEN_R {
        ETHMACTXLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacrxlpen(&self) -> ETHMACRXLPEN_R {
        ETHMACRXLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacptplpen(&self) -> ETHMACPTPLPEN_R {
        ETHMACPTPLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghslpen(&self) -> OTGHSLPEN_R {
        OTGHSLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghsulpilpen(&self) -> OTGHSULPILPEN_R {
        OTGHSULPILPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 19 - SRAM3 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA2D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 9 - IO port J clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IO port K clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<0> {
        GPIOALPEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<1> {
        GPIOBLPEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<2> {
        GPIOCLPEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<3> {
        GPIODLPEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<4> {
        GPIOELPEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<5> {
        GPIOFLPEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<6> {
        GPIOGLPEN_W::new(self)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<7> {
        GPIOHLPEN_W::new(self)
    }
    #[doc = "Bit 8 - IO port I clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<8> {
        GPIOILPEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<12> {
        CRCLPEN_W::new(self)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<15> {
        FLITFLPEN_W::new(self)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<16> {
        SRAM1LPEN_W::new(self)
    }
    #[doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<17> {
        SRAM2LPEN_W::new(self)
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W<18> {
        BKPSRAMLPEN_W::new(self)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<21> {
        DMA1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<22> {
        DMA2LPEN_W::new(self)
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W<25> {
        ETHMACLPEN_W::new(self)
    }
    #[doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmactxlpen(&mut self) -> ETHMACTXLPEN_W<26> {
        ETHMACTXLPEN_W::new(self)
    }
    #[doc = "Bit 27 - Ethernet reception clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacrxlpen(&mut self) -> ETHMACRXLPEN_W<27> {
        ETHMACRXLPEN_W::new(self)
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ethmacptplpen(&mut self) -> ETHMACPTPLPEN_W<28> {
        ETHMACPTPLPEN_W::new(self)
    }
    #[doc = "Bit 29 - USB OTG HS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghslpen(&mut self) -> OTGHSLPEN_W<29> {
        OTGHSLPEN_W::new(self)
    }
    #[doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otghsulpilpen(&mut self) -> OTGHSULPILPEN_W<30> {
        OTGHSULPILPEN_W::new(self)
    }
    #[doc = "Bit 19 - SRAM3 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W<19> {
        SRAM3LPEN_W::new(self)
    }
    #[doc = "Bit 23 - DMA2D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<23> {
        DMA2DLPEN_W::new(self)
    }
    #[doc = "Bit 9 - IO port J clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<9> {
        GPIOJLPEN_W::new(self)
    }
    #[doc = "Bit 10 - IO port K clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<10> {
        GPIOKLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1lpenr](index.html) module"]
pub struct AHB1LPENR_SPEC;
impl crate::RegisterSpec for AHB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1lpenr::R](R) reader structure"]
impl crate::Readable for AHB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1lpenr::W](W) writer structure"]
impl crate::Writable for AHB1LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0x7e67_91ff"]
impl crate::Resettable for AHB1LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7e67_91ff
    }
}
