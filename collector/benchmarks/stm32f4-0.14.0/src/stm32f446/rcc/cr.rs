#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLLI2S clock ready flag"]
pub type PLLI2SRDY_A = HSIRDY_A;
#[doc = "Field `PLLI2SRDY` reader - PLLI2S clock ready flag"]
pub type PLLI2SRDY_R = HSIRDY_R;
#[doc = "PLLI2S enable"]
pub type PLLI2SON_A = HSION_A;
#[doc = "Field `PLLI2SON` reader - PLLI2S enable"]
pub type PLLI2SON_R = HSION_R;
#[doc = "Field `PLLI2SON` writer - PLLI2S enable"]
pub struct PLLI2SON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLI2SON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLI2SON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Main PLL (PLL) clock ready flag"]
pub type PLLRDY_A = HSIRDY_A;
#[doc = "Field `PLLRDY` reader - Main PLL (PLL) clock ready flag"]
pub type PLLRDY_R = HSIRDY_R;
#[doc = "Main PLL (PLL) enable"]
pub type PLLON_A = HSION_A;
#[doc = "Field `PLLON` reader - Main PLL (PLL) enable"]
pub type PLLON_R = HSION_R;
#[doc = "Field `PLLON` writer - Main PLL (PLL) enable"]
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Clock security system enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSON_A {
    #[doc = "0: Clock security system disabled (clock detector OFF)"]
    OFF = 0,
    #[doc = "1: Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    ON = 1,
}
impl From<CSSON_A> for bool {
    #[inline(always)]
    fn from(variant: CSSON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSON` reader - Clock security system enable"]
pub struct CSSON_R(crate::FieldReader<bool, CSSON_A>);
impl CSSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSON_A {
        match self.bits {
            false => CSSON_A::OFF,
            true => CSSON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CSSON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CSSON_A::ON
    }
}
impl core::ops::Deref for CSSON_R {
    type Target = crate::FieldReader<bool, CSSON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub struct CSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock security system disabled (clock detector OFF)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CSSON_A::OFF)
    }
    #[doc = "Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CSSON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "HSE clock bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYP_A {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    NOTBYPASSED = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    BYPASSED = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE clock bypass"]
pub struct HSEBYP_R(crate::FieldReader<bool, HSEBYP_A>);
impl HSEBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEBYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NOTBYPASSED,
            true => HSEBYP_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        **self == HSEBYP_A::NOTBYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        **self == HSEBYP_A::BYPASSED
    }
}
impl core::ops::Deref for HSEBYP_R {
    type Target = crate::FieldReader<bool, HSEBYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSEBYP` writer - HSE clock bypass"]
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEBYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NOTBYPASSED)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::BYPASSED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "HSE clock ready flag"]
pub type HSERDY_A = HSIRDY_A;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = HSIRDY_R;
#[doc = "HSE clock enable"]
pub type HSEON_A = HSION_A;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HSEON_R = HSION_R;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSEON_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSEON_A::ON)
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
#[doc = "Field `HSICAL` reader - Internal high-speed clock calibration"]
pub struct HSICAL_R(crate::FieldReader<u8, u8>);
impl HSICAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSICAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSICAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSITRIM` reader - Internal high-speed clock trimming"]
pub struct HSITRIM_R(crate::FieldReader<u8, u8>);
impl HSITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSITRIM` writer - Internal high-speed clock trimming"]
pub struct HSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Internal high-speed clock ready flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDY_A {
    #[doc = "0: Clock not ready"]
    NOTREADY = 0,
    #[doc = "1: Clock ready"]
    READY = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - Internal high-speed clock ready flag"]
pub struct HSIRDY_R(crate::FieldReader<bool, HSIRDY_A>);
impl HSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::NOTREADY,
            true => HSIRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == HSIRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HSIRDY_A::READY
    }
}
impl core::ops::Deref for HSIRDY_R {
    type Target = crate::FieldReader<bool, HSIRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal high-speed clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSION_A {
    #[doc = "0: Clock Off"]
    OFF = 0,
    #[doc = "1: Clock On"]
    ON = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub struct HSION_R(crate::FieldReader<bool, HSION_A>);
impl HSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::OFF,
            true => HSION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == HSION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == HSION_A::ON
    }
}
impl core::ops::Deref for HSION_R {
    type Target = crate::FieldReader<bool, HSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSION_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSION_A::ON)
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
#[doc = "PLLSAI clock ready flag"]
pub type PLLSAIRDY_A = HSIRDY_A;
#[doc = "Field `PLLSAIRDY` reader - PLLSAI clock ready flag"]
pub type PLLSAIRDY_R = HSIRDY_R;
#[doc = "PLLSAI enable"]
pub type PLLSAION_A = HSION_A;
#[doc = "Field `PLLSAION` reader - PLLSAI enable"]
pub type PLLSAION_R = HSION_R;
#[doc = "Field `PLLSAION` writer - PLLSAI enable"]
pub struct PLLSAION_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLSAION_A::OFF)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLSAION_A::ON)
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
impl R {
    #[doc = "Bit 27 - PLLI2S clock ready flag"]
    #[inline(always)]
    pub fn plli2srdy(&self) -> PLLI2SRDY_R {
        PLLI2SRDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline(always)]
    pub fn plli2son(&self) -> PLLI2SON_R {
        PLLI2SON_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Main PLL (PLL) clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Main PLL (PLL) enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Internal high-speed clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 3:7 - Internal high-speed clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 29 - PLLSAI clock ready flag"]
    #[inline(always)]
    pub fn pllsairdy(&self) -> PLLSAIRDY_R {
        PLLSAIRDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PLLSAI enable"]
    #[inline(always)]
    pub fn pllsaion(&self) -> PLLSAION_R {
        PLLSAION_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline(always)]
    pub fn plli2son(&mut self) -> PLLI2SON_W {
        PLLI2SON_W { w: self }
    }
    #[doc = "Bit 24 - Main PLL (PLL) enable"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W {
        CSSON_W { w: self }
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    #[doc = "Bits 3:7 - Internal high-speed clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    #[doc = "Bit 28 - PLLSAI enable"]
    #[inline(always)]
    pub fn pllsaion(&mut self) -> PLLSAION_W {
        PLLSAION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x83"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x83
    }
}
