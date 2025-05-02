#[doc = "Register `GCR` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Horizontal Synchronization Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPOL_A {
    #[doc = "0: Horizontal synchronization polarity is active low"]
    ActiveLow = 0,
    #[doc = "1: Horizontal synchronization polarity is active high"]
    ActiveHigh = 1,
}
impl From<HSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPOL` reader - Horizontal Synchronization Polarity"]
pub type HSPOL_R = crate::BitReader<HSPOL_A>;
impl HSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPOL_A {
        match self.bits {
            false => HSPOL_A::ActiveLow,
            true => HSPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == HSPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == HSPOL_A::ActiveHigh
    }
}
#[doc = "Field `HSPOL` writer - Horizontal Synchronization Polarity"]
pub type HSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, HSPOL_A, O>;
impl<'a, const O: u8> HSPOL_W<'a, O> {
    #[doc = "Horizontal synchronization polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(HSPOL_A::ActiveLow)
    }
    #[doc = "Horizontal synchronization polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(HSPOL_A::ActiveHigh)
    }
}
#[doc = "Vertical Synchronization Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSPOL_A {
    #[doc = "0: Vertical synchronization polarity is active low"]
    ActiveLow = 0,
    #[doc = "1: Vertical synchronization polarity is active high"]
    ActiveHigh = 1,
}
impl From<VSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: VSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSPOL` reader - Vertical Synchronization Polarity"]
pub type VSPOL_R = crate::BitReader<VSPOL_A>;
impl VSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSPOL_A {
        match self.bits {
            false => VSPOL_A::ActiveLow,
            true => VSPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == VSPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == VSPOL_A::ActiveHigh
    }
}
#[doc = "Field `VSPOL` writer - Vertical Synchronization Polarity"]
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, VSPOL_A, O>;
impl<'a, const O: u8> VSPOL_W<'a, O> {
    #[doc = "Vertical synchronization polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(VSPOL_A::ActiveLow)
    }
    #[doc = "Vertical synchronization polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(VSPOL_A::ActiveHigh)
    }
}
#[doc = "Data Enable Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEPOL_A {
    #[doc = "0: Data enable polarity is active low"]
    ActiveLow = 0,
    #[doc = "1: Data enable polarity is active high"]
    ActiveHigh = 1,
}
impl From<DEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DEPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEPOL` reader - Data Enable Polarity"]
pub type DEPOL_R = crate::BitReader<DEPOL_A>;
impl DEPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEPOL_A {
        match self.bits {
            false => DEPOL_A::ActiveLow,
            true => DEPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DEPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DEPOL_A::ActiveHigh
    }
}
#[doc = "Field `DEPOL` writer - Data Enable Polarity"]
pub type DEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, DEPOL_A, O>;
impl<'a, const O: u8> DEPOL_W<'a, O> {
    #[doc = "Data enable polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(DEPOL_A::ActiveLow)
    }
    #[doc = "Data enable polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(DEPOL_A::ActiveHigh)
    }
}
#[doc = "Pixel Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCPOL_A {
    #[doc = "0: Pixel clock on rising edge"]
    RisingEdge = 0,
    #[doc = "1: Pixel clock on falling edge"]
    FallingEdge = 1,
}
impl From<PCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCPOL` reader - Pixel Clock Polarity"]
pub type PCPOL_R = crate::BitReader<PCPOL_A>;
impl PCPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCPOL_A {
        match self.bits {
            false => PCPOL_A::RisingEdge,
            true => PCPOL_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PCPOL_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PCPOL_A::FallingEdge
    }
}
#[doc = "Field `PCPOL` writer - Pixel Clock Polarity"]
pub type PCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, PCPOL_A, O>;
impl<'a, const O: u8> PCPOL_W<'a, O> {
    #[doc = "Pixel clock on rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(PCPOL_A::RisingEdge)
    }
    #[doc = "Pixel clock on falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(PCPOL_A::FallingEdge)
    }
}
#[doc = "Dither Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN_A {
    #[doc = "0: Dither disabled"]
    Disabled = 0,
    #[doc = "1: Dither enabled"]
    Enabled = 1,
}
impl From<DEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEN` reader - Dither Enable"]
pub type DEN_R = crate::BitReader<DEN_A>;
impl DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN_A {
        match self.bits {
            false => DEN_A::Disabled,
            true => DEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEN_A::Enabled
    }
}
#[doc = "Field `DEN` writer - Dither Enable"]
pub type DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, DEN_A, O>;
impl<'a, const O: u8> DEN_W<'a, O> {
    #[doc = "Dither disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEN_A::Disabled)
    }
    #[doc = "Dither enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEN_A::Enabled)
    }
}
#[doc = "Field `DRW` reader - Dither Red Width"]
pub type DRW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DGW` reader - Dither Green Width"]
pub type DGW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBW` reader - Dither Blue Width"]
pub type DBW_R = crate::FieldReader<u8, u8>;
#[doc = "LCD-TFT controller enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCEN_A {
    #[doc = "0: LCD-TFT controller disabled"]
    Disabled = 0,
    #[doc = "1: LCD-TFT controller enabled"]
    Enabled = 1,
}
impl From<LTDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCEN` reader - LCD-TFT controller enable bit"]
pub type LTDCEN_R = crate::BitReader<LTDCEN_A>;
impl LTDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCEN_A {
        match self.bits {
            false => LTDCEN_A::Disabled,
            true => LTDCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN_A::Enabled
    }
}
#[doc = "Field `LTDCEN` writer - LCD-TFT controller enable bit"]
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, LTDCEN_A, O>;
impl<'a, const O: u8> LTDCEN_W<'a, O> {
    #[doc = "LCD-TFT controller disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Disabled)
    }
    #[doc = "LCD-TFT controller enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Dither Red Width"]
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Dither Green Width"]
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Dither Blue Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W<31> {
        HSPOL_W::new(self)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<30> {
        VSPOL_W::new(self)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W<29> {
        DEPOL_W::new(self)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&mut self) -> PCPOL_W<28> {
        PCPOL_W::new(self)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W<16> {
        DEN_W::new(self)
    }
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<0> {
        LTDCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCR to value 0x2220"]
impl crate::Resettable for GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2220
    }
}
