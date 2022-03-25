#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "One sample bit method enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEBIT_A {
    #[doc = "0: Three sample bit method"]
    SAMPLE3 = 0,
    #[doc = "1: One sample bit method"]
    SAMPLE1 = 1,
}
impl From<ONEBIT_A> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONEBIT` reader - One sample bit method enable"]
pub struct ONEBIT_R(crate::FieldReader<bool, ONEBIT_A>);
impl ONEBIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONEBIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONEBIT_A {
        match self.bits {
            false => ONEBIT_A::SAMPLE3,
            true => ONEBIT_A::SAMPLE1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE3`"]
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        **self == ONEBIT_A::SAMPLE3
    }
    #[doc = "Checks if the value of the field is `SAMPLE1`"]
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        **self == ONEBIT_A::SAMPLE1
    }
}
impl core::ops::Deref for ONEBIT_R {
    type Target = crate::FieldReader<bool, ONEBIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONEBIT` writer - One sample bit method enable"]
pub struct ONEBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONEBIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn sample3(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE3)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn sample1(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "DMA enable transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAT_A {
    #[doc = "0: DMA mode is disabled for transmission"]
    DISABLED = 0,
    #[doc = "1: DMA mode is enabled for transmission"]
    ENABLED = 1,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub struct DMAT_R(crate::FieldReader<bool, DMAT_A>);
impl DMAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            false => DMAT_A::DISABLED,
            true => DMAT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAT_A::ENABLED
    }
}
impl core::ops::Deref for DMAT_R {
    type Target = crate::FieldReader<bool, DMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub struct DMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAT_A::DISABLED)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAT_A::ENABLED)
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
#[doc = "DMA enable receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR_A {
    #[doc = "0: DMA mode is disabled for reception"]
    DISABLED = 0,
    #[doc = "1: DMA mode is enabled for reception"]
    ENABLED = 1,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub struct DMAR_R(crate::FieldReader<bool, DMAR_A>);
impl DMAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            false => DMAR_A::DISABLED,
            true => DMAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAR_A::ENABLED
    }
}
impl core::ops::Deref for DMAR_R {
    type Target = crate::FieldReader<bool, DMAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub struct DMAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAR_A::DISABLED)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAR_A::ENABLED)
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
#[doc = "Half-duplex selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSEL_A {
    #[doc = "0: Half duplex mode is not selected"]
    FULLDUPLEX = 0,
    #[doc = "1: Half duplex mode is selected"]
    HALFDUPLEX = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub struct HDSEL_R(crate::FieldReader<bool, HDSEL_A>);
impl HDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::FULLDUPLEX,
            true => HDSEL_A::HALFDUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `FULLDUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        **self == HDSEL_A::FULLDUPLEX
    }
    #[doc = "Checks if the value of the field is `HALFDUPLEX`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        **self == HDSEL_A::HALFDUPLEX
    }
}
impl core::ops::Deref for HDSEL_R {
    type Target = crate::FieldReader<bool, HDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub struct HDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(HDSEL_A::FULLDUPLEX)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(HDSEL_A::HALFDUPLEX)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "IrDA low-power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLP_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Low-power mode"]
    LOWPOWER = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub struct IRLP_R(crate::FieldReader<bool, IRLP_A>);
impl IRLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::NORMAL,
            true => IRLP_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == IRLP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        **self == IRLP_A::LOWPOWER
    }
}
impl core::ops::Deref for IRLP_R {
    type Target = crate::FieldReader<bool, IRLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub struct IRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> IRLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLP_A::NORMAL)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLP_A::LOWPOWER)
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
#[doc = "IrDA mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: IrDA disabled"]
    DISABLED = 0,
    #[doc = "1: IrDA enabled"]
    ENABLED = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub struct IREN_R(crate::FieldReader<bool, IREN_A>);
impl IREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::DISABLED,
            true => IREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IREN_A::ENABLED
    }
}
impl core::ops::Deref for IREN_R {
    type Target = crate::FieldReader<bool, IREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::DISABLED)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::ENABLED)
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
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    #[doc = "0: Error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Error interrupt enabled"]
    ENABLED = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub struct EIE_R(crate::FieldReader<bool, EIE_A>);
impl EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::DISABLED,
            true => EIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EIE_A::ENABLED
    }
}
impl core::ops::Deref for EIE_R {
    type Target = crate::FieldReader<bool, EIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIE_A::DISABLED)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIE_A::ENABLED)
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
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W {
        ONEBIT_W { w: self }
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W {
        DMAT_W { w: self }
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W {
        DMAR_W { w: self }
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W {
        HDSEL_W { w: self }
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W {
        IRLP_W { w: self }
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
