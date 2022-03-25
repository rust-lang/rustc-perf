#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    #[doc = "0: TXE interrupt masked"]
    MASKED = 0,
    #[doc = "1: TXE interrupt not masked"]
    NOTMASKED = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub struct TXEIE_R(crate::FieldReader<bool, TXEIE_A>);
impl TXEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::MASKED,
            true => TXEIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == TXEIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        **self == TXEIE_A::NOTMASKED
    }
}
impl core::ops::Deref for TXEIE_R {
    type Target = crate::FieldReader<bool, TXEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIE_A::MASKED)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    #[doc = "0: RXE interrupt masked"]
    MASKED = 0,
    #[doc = "1: RXE interrupt not masked"]
    NOTMASKED = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub struct RXNEIE_R(crate::FieldReader<bool, RXNEIE_A>);
impl RXNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::MASKED,
            true => RXNEIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == RXNEIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        **self == RXNEIE_A::NOTMASKED
    }
}
impl core::ops::Deref for RXNEIE_R {
    type Target = crate::FieldReader<bool, RXNEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXNEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::MASKED)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt masked"]
    MASKED = 0,
    #[doc = "1: Error interrupt not masked"]
    NOTMASKED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub struct ERRIE_R(crate::FieldReader<bool, ERRIE_A>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::MASKED,
            true => ERRIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == ERRIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        **self == ERRIE_A::NOTMASKED
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, ERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIE_A::MASKED)
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: SPI Motorola mode"]
    MOTOROLA = 0,
    #[doc = "1: SPI TI mode"]
    TI = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRF` reader - Frame format"]
pub struct FRF_R(crate::FieldReader<bool, FRF_A>);
impl FRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::MOTOROLA,
            true => FRF_A::TI,
        }
    }
    #[doc = "Checks if the value of the field is `MOTOROLA`"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        **self == FRF_A::MOTOROLA
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        **self == FRF_A::TI
    }
}
impl core::ops::Deref for FRF_R {
    type Target = crate::FieldReader<bool, FRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRF` writer - Frame format"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(FRF_A::MOTOROLA)
    }
    #[doc = "SPI TI mode"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::TI)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "SS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled in master mode"]
    DISABLED = 0,
    #[doc = "1: SS output is enabled in master mode"]
    ENABLED = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - SS output enable"]
pub struct SSOE_R(crate::FieldReader<bool, SSOE_A>);
impl SSOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::DISABLED,
            true => SSOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SSOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSOE_A::ENABLED
    }
}
impl core::ops::Deref for SSOE_R {
    type Target = crate::FieldReader<bool, SSOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSOE` writer - SS output enable"]
pub struct SSOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::DISABLED)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Tx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    ENABLED = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub struct TXDMAEN_R(crate::FieldReader<bool, TXDMAEN_A>);
impl TXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::DISABLED,
            true => TXDMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXDMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXDMAEN_A::ENABLED
    }
}
impl core::ops::Deref for TXDMAEN_R {
    type Target = crate::FieldReader<bool, TXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::DISABLED)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    ENABLED = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub struct RXDMAEN_R(crate::FieldReader<bool, RXDMAEN_A>);
impl RXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::DISABLED,
            true => RXDMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXDMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXDMAEN_A::ENABLED
    }
}
impl core::ops::Deref for RXDMAEN_R {
    type Target = crate::FieldReader<bool, RXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::ENABLED)
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
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W {
        SSOE_W { w: self }
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
