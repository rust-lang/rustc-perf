#[doc = "Register `DCKCFGR` reader"]
pub struct R(crate::R<DCKCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCKCFGR` writer"]
pub struct W(crate::W<DCKCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR_SPEC>;
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
impl From<crate::W<DCKCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIMPRE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPRE_A {
    #[doc = "0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    MUL2 = 0,
    #[doc = "1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    MUL4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPRE` reader - TIMPRE"]
pub struct TIMPRE_R(crate::FieldReader<bool, TIMPRE_A>);
impl TIMPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::MUL2,
            true => TIMPRE_A::MUL4,
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        **self == TIMPRE_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        **self == TIMPRE_A::MUL4
    }
}
impl core::ops::Deref for TIMPRE_R {
    type Target = crate::FieldReader<bool, TIMPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMPRE` writer - TIMPRE"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL2)
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL4)
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
#[doc = "I2SSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SSRC_A {
    #[doc = "0: I2Sx clock frequency = f(PLLCLK_R)"]
    PLLCLKR = 0,
    #[doc = "1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    I2S_CKIN = 1,
    #[doc = "3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    HSI_HSE = 3,
}
impl From<I2SSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2SSRC` reader - I2SSRC"]
pub struct I2SSRC_R(crate::FieldReader<u8, I2SSRC_A>);
impl I2SSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2SSRC_A> {
        match self.bits {
            0 => Some(I2SSRC_A::PLLCLKR),
            1 => Some(I2SSRC_A::I2S_CKIN),
            3 => Some(I2SSRC_A::HSI_HSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLLCLKR`"]
    #[inline(always)]
    pub fn is_pllclkr(&self) -> bool {
        **self == I2SSRC_A::PLLCLKR
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        **self == I2SSRC_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `HSI_HSE`"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        **self == I2SSRC_A::HSI_HSE
    }
}
impl core::ops::Deref for I2SSRC_R {
    type Target = crate::FieldReader<u8, I2SSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SSRC` writer - I2SSRC"]
pub struct I2SSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "I2Sx clock frequency = f(PLLCLK_R)"]
    #[inline(always)]
    pub fn pllclkr(self) -> &'a mut W {
        self.variant(I2SSRC_A::PLLCLKR)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2SSRC_A::I2S_CKIN)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(I2SSRC_A::HSI_HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - I2SSRC"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    #[doc = "Bits 25:26 - I2SSRC"]
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W {
        I2SSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCKCFGR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dckcfgr](index.html) module"]
pub struct DCKCFGR_SPEC;
impl crate::RegisterSpec for DCKCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dckcfgr::R](R) reader structure"]
impl crate::Readable for DCKCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dckcfgr::W](W) writer structure"]
impl crate::Writable for DCKCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCKCFGR to value 0"]
impl crate::Resettable for DCKCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
