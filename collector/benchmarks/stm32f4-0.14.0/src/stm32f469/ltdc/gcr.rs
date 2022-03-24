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
    ACTIVELOW = 0,
    #[doc = "1: Horizontal synchronization polarity is active high"]
    ACTIVEHIGH = 1,
}
impl From<HSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPOL` reader - Horizontal Synchronization Polarity"]
pub struct HSPOL_R(crate::FieldReader<bool, HSPOL_A>);
impl HSPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPOL_A {
        match self.bits {
            false => HSPOL_A::ACTIVELOW,
            true => HSPOL_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == HSPOL_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == HSPOL_A::ACTIVEHIGH
    }
}
impl core::ops::Deref for HSPOL_R {
    type Target = crate::FieldReader<bool, HSPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSPOL` writer - Horizontal Synchronization Polarity"]
pub struct HSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Horizontal synchronization polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(HSPOL_A::ACTIVELOW)
    }
    #[doc = "Horizontal synchronization polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(HSPOL_A::ACTIVEHIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Vertical Synchronization Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSPOL_A {
    #[doc = "0: Vertical synchronization polarity is active low"]
    ACTIVELOW = 0,
    #[doc = "1: Vertical synchronization polarity is active high"]
    ACTIVEHIGH = 1,
}
impl From<VSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: VSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSPOL` reader - Vertical Synchronization Polarity"]
pub struct VSPOL_R(crate::FieldReader<bool, VSPOL_A>);
impl VSPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSPOL_A {
        match self.bits {
            false => VSPOL_A::ACTIVELOW,
            true => VSPOL_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == VSPOL_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == VSPOL_A::ACTIVEHIGH
    }
}
impl core::ops::Deref for VSPOL_R {
    type Target = crate::FieldReader<bool, VSPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSPOL` writer - Vertical Synchronization Polarity"]
pub struct VSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VSPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Vertical synchronization polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(VSPOL_A::ACTIVELOW)
    }
    #[doc = "Vertical synchronization polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(VSPOL_A::ACTIVEHIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Data Enable Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEPOL_A {
    #[doc = "0: Data enable polarity is active low"]
    ACTIVELOW = 0,
    #[doc = "1: Data enable polarity is active high"]
    ACTIVEHIGH = 1,
}
impl From<DEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DEPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEPOL` reader - Data Enable Polarity"]
pub struct DEPOL_R(crate::FieldReader<bool, DEPOL_A>);
impl DEPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEPOL_A {
        match self.bits {
            false => DEPOL_A::ACTIVELOW,
            true => DEPOL_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == DEPOL_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == DEPOL_A::ACTIVEHIGH
    }
}
impl core::ops::Deref for DEPOL_R {
    type Target = crate::FieldReader<bool, DEPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEPOL` writer - Data Enable Polarity"]
pub struct DEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data enable polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(DEPOL_A::ACTIVELOW)
    }
    #[doc = "Data enable polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(DEPOL_A::ACTIVEHIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Pixel Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCPOL_A {
    #[doc = "0: Pixel clock on rising edge"]
    RISINGEDGE = 0,
    #[doc = "1: Pixel clock on falling edge"]
    FALLINGEDGE = 1,
}
impl From<PCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCPOL` reader - Pixel Clock Polarity"]
pub struct PCPOL_R(crate::FieldReader<bool, PCPOL_A>);
impl PCPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCPOL_A {
        match self.bits {
            false => PCPOL_A::RISINGEDGE,
            true => PCPOL_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == PCPOL_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == PCPOL_A::FALLINGEDGE
    }
}
impl core::ops::Deref for PCPOL_R {
    type Target = crate::FieldReader<bool, PCPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCPOL` writer - Pixel Clock Polarity"]
pub struct PCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pixel clock on rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(PCPOL_A::RISINGEDGE)
    }
    #[doc = "Pixel clock on falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(PCPOL_A::FALLINGEDGE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Dither Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN_A {
    #[doc = "0: Dither disabled"]
    DISABLED = 0,
    #[doc = "1: Dither enabled"]
    ENABLED = 1,
}
impl From<DEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEN` reader - Dither Enable"]
pub struct DEN_R(crate::FieldReader<bool, DEN_A>);
impl DEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN_A {
        match self.bits {
            false => DEN_A::DISABLED,
            true => DEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DEN_A::ENABLED
    }
}
impl core::ops::Deref for DEN_R {
    type Target = crate::FieldReader<bool, DEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEN` writer - Dither Enable"]
pub struct DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Dither disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEN_A::DISABLED)
    }
    #[doc = "Dither enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `DRW` reader - Dither Red Width"]
pub struct DRW_R(crate::FieldReader<u8, u8>);
impl DRW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DGW` reader - Dither Green Width"]
pub struct DGW_R(crate::FieldReader<u8, u8>);
impl DGW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DGW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DGW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBW` reader - Dither Blue Width"]
pub struct DBW_R(crate::FieldReader<u8, u8>);
impl DBW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LCD-TFT controller enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCEN_A {
    #[doc = "0: LCD-TFT controller disabled"]
    DISABLED = 0,
    #[doc = "1: LCD-TFT controller enabled"]
    ENABLED = 1,
}
impl From<LTDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCEN` reader - LCD-TFT controller enable bit"]
pub struct LTDCEN_R(crate::FieldReader<bool, LTDCEN_A>);
impl LTDCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCEN_A {
        match self.bits {
            false => LTDCEN_A::DISABLED,
            true => LTDCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LTDCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LTDCEN_A::ENABLED
    }
}
impl core::ops::Deref for LTDCEN_R {
    type Target = crate::FieldReader<bool, LTDCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCEN` writer - LCD-TFT controller enable bit"]
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTDCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LCD-TFT controller disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::DISABLED)
    }
    #[doc = "LCD-TFT controller enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LTDCEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Dither Red Width"]
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Dither Green Width"]
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Dither Blue Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W {
        HSPOL_W { w: self }
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W {
        VSPOL_W { w: self }
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W {
        DEPOL_W { w: self }
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&mut self) -> PCPOL_W {
        PCPOL_W { w: self }
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W {
        DEN_W { w: self }
    }
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
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
