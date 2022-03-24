#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SLKIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLKIE_A {
    #[doc = "0: No interrupt when SLAKI bit is set"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when SLAKI bit is set"]
    ENABLED = 1,
}
impl From<SLKIE_A> for bool {
    #[inline(always)]
    fn from(variant: SLKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLKIE` reader - SLKIE"]
pub struct SLKIE_R(crate::FieldReader<bool, SLKIE_A>);
impl SLKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLKIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLKIE_A {
        match self.bits {
            false => SLKIE_A::DISABLED,
            true => SLKIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SLKIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SLKIE_A::ENABLED
    }
}
impl core::ops::Deref for SLKIE_R {
    type Target = crate::FieldReader<bool, SLKIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLKIE` writer - SLKIE"]
pub struct SLKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLKIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLKIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt when SLAKI bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLKIE_A::DISABLED)
    }
    #[doc = "Interrupt generated when SLAKI bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLKIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "WKUIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUIE_A {
    #[doc = "0: No interrupt when WKUI is set"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when WKUI bit is set"]
    ENABLED = 1,
}
impl From<WKUIE_A> for bool {
    #[inline(always)]
    fn from(variant: WKUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUIE` reader - WKUIE"]
pub struct WKUIE_R(crate::FieldReader<bool, WKUIE_A>);
impl WKUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUIE_A {
        match self.bits {
            false => WKUIE_A::DISABLED,
            true => WKUIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUIE_A::ENABLED
    }
}
impl core::ops::Deref for WKUIE_R {
    type Target = crate::FieldReader<bool, WKUIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUIE` writer - WKUIE"]
pub struct WKUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt when WKUI is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKUIE_A::DISABLED)
    }
    #[doc = "Interrupt generated when WKUI bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKUIE_A::ENABLED)
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
#[doc = "ERRIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: No interrupt will be generated when an error condition is pending in the CAN_ESR"]
    DISABLED = 0,
    #[doc = "1: An interrupt will be generation when an error condition is pending in the CAN_ESR"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - ERRIE"]
pub struct ERRIE_R(crate::FieldReader<bool, ERRIE_A>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERRIE_A::ENABLED
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, ERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIE` writer - ERRIE"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt will be generated when an error condition is pending in the CAN_ESR"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "An interrupt will be generation when an error condition is pending in the CAN_ESR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "LECIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECIE_A {
    #[doc = "0: ERRI bit will not be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    DISABLED = 0,
    #[doc = "1: ERRI bit will be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    ENABLED = 1,
}
impl From<LECIE_A> for bool {
    #[inline(always)]
    fn from(variant: LECIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LECIE` reader - LECIE"]
pub struct LECIE_R(crate::FieldReader<bool, LECIE_A>);
impl LECIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LECIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LECIE_A {
        match self.bits {
            false => LECIE_A::DISABLED,
            true => LECIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LECIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LECIE_A::ENABLED
    }
}
impl core::ops::Deref for LECIE_R {
    type Target = crate::FieldReader<bool, LECIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LECIE` writer - LECIE"]
pub struct LECIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LECIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LECIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ERRI bit will not be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LECIE_A::DISABLED)
    }
    #[doc = "ERRI bit will be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LECIE_A::ENABLED)
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
#[doc = "BOFIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFIE_A {
    #[doc = "0: ERRI bit will not be set when BOFF is set"]
    DISABLED = 0,
    #[doc = "1: ERRI bit will be set when BOFF is set"]
    ENABLED = 1,
}
impl From<BOFIE_A> for bool {
    #[inline(always)]
    fn from(variant: BOFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFIE` reader - BOFIE"]
pub struct BOFIE_R(crate::FieldReader<bool, BOFIE_A>);
impl BOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFIE_A {
        match self.bits {
            false => BOFIE_A::DISABLED,
            true => BOFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BOFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BOFIE_A::ENABLED
    }
}
impl core::ops::Deref for BOFIE_R {
    type Target = crate::FieldReader<bool, BOFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFIE` writer - BOFIE"]
pub struct BOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ERRI bit will not be set when BOFF is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFIE_A::DISABLED)
    }
    #[doc = "ERRI bit will be set when BOFF is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "EPVIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPVIE_A {
    #[doc = "0: ERRI bit will not be set when EPVF is set"]
    DISABLED = 0,
    #[doc = "1: ERRI bit will be set when EPVF is set"]
    ENABLED = 1,
}
impl From<EPVIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPVIE` reader - EPVIE"]
pub struct EPVIE_R(crate::FieldReader<bool, EPVIE_A>);
impl EPVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPVIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPVIE_A {
        match self.bits {
            false => EPVIE_A::DISABLED,
            true => EPVIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EPVIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EPVIE_A::ENABLED
    }
}
impl core::ops::Deref for EPVIE_R {
    type Target = crate::FieldReader<bool, EPVIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPVIE` writer - EPVIE"]
pub struct EPVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPVIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ERRI bit will not be set when EPVF is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EPVIE_A::DISABLED)
    }
    #[doc = "ERRI bit will be set when EPVF is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EPVIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "EWGIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWGIE_A {
    #[doc = "0: ERRI bit will not be set when EWGF is set"]
    DISABLED = 0,
    #[doc = "1: ERRI bit will be set when EWGF is set"]
    ENABLED = 1,
}
impl From<EWGIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWGIE` reader - EWGIE"]
pub struct EWGIE_R(crate::FieldReader<bool, EWGIE_A>);
impl EWGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWGIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWGIE_A {
        match self.bits {
            false => EWGIE_A::DISABLED,
            true => EWGIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWGIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWGIE_A::ENABLED
    }
}
impl core::ops::Deref for EWGIE_R {
    type Target = crate::FieldReader<bool, EWGIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWGIE` writer - EWGIE"]
pub struct EWGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EWGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWGIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ERRI bit will not be set when EWGF is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWGIE_A::DISABLED)
    }
    #[doc = "ERRI bit will be set when EWGF is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWGIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "FOVIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOVIE1_A {
    #[doc = "0: No interrupt when FOVR is set"]
    DISABLED = 0,
    #[doc = "1: Interrupt generation when FOVR is set"]
    ENABLED = 1,
}
impl From<FOVIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FOVIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOVIE1` reader - FOVIE1"]
pub struct FOVIE1_R(crate::FieldReader<bool, FOVIE1_A>);
impl FOVIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOVIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOVIE1_A {
        match self.bits {
            false => FOVIE1_A::DISABLED,
            true => FOVIE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FOVIE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FOVIE1_A::ENABLED
    }
}
impl core::ops::Deref for FOVIE1_R {
    type Target = crate::FieldReader<bool, FOVIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOVIE1` writer - FOVIE1"]
pub struct FOVIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOVIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt when FOVR is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FOVIE1_A::DISABLED)
    }
    #[doc = "Interrupt generation when FOVR is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FOVIE1_A::ENABLED)
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
#[doc = "FFIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFIE1_A {
    #[doc = "0: No interrupt when FULL bit is set"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when FULL bit is set"]
    ENABLED = 1,
}
impl From<FFIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FFIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFIE1` reader - FFIE1"]
pub struct FFIE1_R(crate::FieldReader<bool, FFIE1_A>);
impl FFIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFIE1_A {
        match self.bits {
            false => FFIE1_A::DISABLED,
            true => FFIE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FFIE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FFIE1_A::ENABLED
    }
}
impl core::ops::Deref for FFIE1_R {
    type Target = crate::FieldReader<bool, FFIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFIE1` writer - FFIE1"]
pub struct FFIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FFIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt when FULL bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FFIE1_A::DISABLED)
    }
    #[doc = "Interrupt generated when FULL bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FFIE1_A::ENABLED)
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
#[doc = "FMPIE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMPIE1_A {
    #[doc = "0: No interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    ENABLED = 1,
}
impl From<FMPIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FMPIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMPIE1` reader - FMPIE1"]
pub struct FMPIE1_R(crate::FieldReader<bool, FMPIE1_A>);
impl FMPIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMPIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMPIE1_A {
        match self.bits {
            false => FMPIE1_A::DISABLED,
            true => FMPIE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FMPIE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FMPIE1_A::ENABLED
    }
}
impl core::ops::Deref for FMPIE1_R {
    type Target = crate::FieldReader<bool, FMPIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMPIE1` writer - FMPIE1"]
pub struct FMPIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMPIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMPIE1_A::DISABLED)
    }
    #[doc = "Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMPIE1_A::ENABLED)
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
#[doc = "FOVIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOVIE0_A {
    #[doc = "0: No interrupt when FOVR bit is set"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when FOVR bit is set"]
    ENABLED = 1,
}
impl From<FOVIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FOVIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOVIE0` reader - FOVIE0"]
pub struct FOVIE0_R(crate::FieldReader<bool, FOVIE0_A>);
impl FOVIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOVIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOVIE0_A {
        match self.bits {
            false => FOVIE0_A::DISABLED,
            true => FOVIE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FOVIE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FOVIE0_A::ENABLED
    }
}
impl core::ops::Deref for FOVIE0_R {
    type Target = crate::FieldReader<bool, FOVIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOVIE0` writer - FOVIE0"]
pub struct FOVIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOVIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt when FOVR bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FOVIE0_A::DISABLED)
    }
    #[doc = "Interrupt generated when FOVR bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FOVIE0_A::ENABLED)
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
#[doc = "FFIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFIE0_A {
    #[doc = "0: No interrupt when FULL bit is set"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when FULL bit is set"]
    ENABLED = 1,
}
impl From<FFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FFIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFIE0` reader - FFIE0"]
pub struct FFIE0_R(crate::FieldReader<bool, FFIE0_A>);
impl FFIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFIE0_A {
        match self.bits {
            false => FFIE0_A::DISABLED,
            true => FFIE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FFIE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FFIE0_A::ENABLED
    }
}
impl core::ops::Deref for FFIE0_R {
    type Target = crate::FieldReader<bool, FFIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFIE0` writer - FFIE0"]
pub struct FFIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FFIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt when FULL bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FFIE0_A::DISABLED)
    }
    #[doc = "Interrupt generated when FULL bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FFIE0_A::ENABLED)
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
#[doc = "FMPIE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMPIE0_A {
    #[doc = "0: No interrupt generated when state of FMP\\[1:0\\]
bits are not 00"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    ENABLED = 1,
}
impl From<FMPIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FMPIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMPIE0` reader - FMPIE0"]
pub struct FMPIE0_R(crate::FieldReader<bool, FMPIE0_A>);
impl FMPIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMPIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMPIE0_A {
        match self.bits {
            false => FMPIE0_A::DISABLED,
            true => FMPIE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FMPIE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FMPIE0_A::ENABLED
    }
}
impl core::ops::Deref for FMPIE0_R {
    type Target = crate::FieldReader<bool, FMPIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMPIE0` writer - FMPIE0"]
pub struct FMPIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMPIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt generated when state of FMP\\[1:0\\]
bits are not 00"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMPIE0_A::DISABLED)
    }
    #[doc = "Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMPIE0_A::ENABLED)
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
#[doc = "TMEIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMEIE_A {
    #[doc = "0: No interrupt when RQCPx bit is set"]
    DISABLED = 0,
    #[doc = "1: Interrupt generated when RQCPx bit is set"]
    ENABLED = 1,
}
impl From<TMEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TMEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEIE` reader - TMEIE"]
pub struct TMEIE_R(crate::FieldReader<bool, TMEIE_A>);
impl TMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMEIE_A {
        match self.bits {
            false => TMEIE_A::DISABLED,
            true => TMEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TMEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TMEIE_A::ENABLED
    }
}
impl core::ops::Deref for TMEIE_R {
    type Target = crate::FieldReader<bool, TMEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMEIE` writer - TMEIE"]
pub struct TMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt when RQCPx bit is set"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TMEIE_A::DISABLED)
    }
    #[doc = "Interrupt generated when RQCPx bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TMEIE_A::ENABLED)
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
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    pub fn slkie(&self) -> SLKIE_R {
        SLKIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    pub fn wkuie(&self) -> WKUIE_R {
        WKUIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    pub fn bofie(&self) -> BOFIE_R {
        BOFIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    pub fn epvie(&self) -> EPVIE_R {
        EPVIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    pub fn ewgie(&self) -> EWGIE_R {
        EWGIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    pub fn fovie1(&self) -> FOVIE1_R {
        FOVIE1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    pub fn ffie1(&self) -> FFIE1_R {
        FFIE1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    pub fn fmpie1(&self) -> FMPIE1_R {
        FMPIE1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    pub fn fovie0(&self) -> FOVIE0_R {
        FOVIE0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    pub fn ffie0(&self) -> FFIE0_R {
        FFIE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    pub fn fmpie0(&self) -> FMPIE0_R {
        FMPIE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    pub fn slkie(&mut self) -> SLKIE_W {
        SLKIE_W { w: self }
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    pub fn wkuie(&mut self) -> WKUIE_W {
        WKUIE_W { w: self }
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    pub fn lecie(&mut self) -> LECIE_W {
        LECIE_W { w: self }
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    pub fn bofie(&mut self) -> BOFIE_W {
        BOFIE_W { w: self }
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    pub fn epvie(&mut self) -> EPVIE_W {
        EPVIE_W { w: self }
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    pub fn ewgie(&mut self) -> EWGIE_W {
        EWGIE_W { w: self }
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    pub fn fovie1(&mut self) -> FOVIE1_W {
        FOVIE1_W { w: self }
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    pub fn ffie1(&mut self) -> FFIE1_W {
        FFIE1_W { w: self }
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    pub fn fmpie1(&mut self) -> FMPIE1_W {
        FMPIE1_W { w: self }
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    pub fn fovie0(&mut self) -> FOVIE0_W {
        FOVIE0_W { w: self }
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    pub fn ffie0(&mut self) -> FFIE0_W {
        FFIE0_W { w: self }
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    pub fn fmpie0(&mut self) -> FMPIE0_W {
        FMPIE0_W { w: self }
    }
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    pub fn tmeie(&mut self) -> TMEIE_W {
        TMEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
