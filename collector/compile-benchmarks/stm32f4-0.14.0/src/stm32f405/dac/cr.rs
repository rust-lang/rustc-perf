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
#[doc = "DAC channel2 DMA underrun interrupt enable"]
pub type DMAUDRIE2_A = DMAUDRIE1_A;
#[doc = "Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable"]
pub type DMAUDRIE2_R = DMAUDRIE1_R;
#[doc = "Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable"]
pub struct DMAUDRIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDRIE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE2_A::DISABLED)
    }
    #[doc = "DAC channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE2_A::ENABLED)
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
#[doc = "DAC channel2 DMA enable"]
pub type DMAEN2_A = DMAEN1_A;
#[doc = "Field `DMAEN2` reader - DAC channel2 DMA enable"]
pub type DMAEN2_R = DMAEN1_R;
#[doc = "Field `DMAEN2` writer - DAC channel2 DMA enable"]
pub struct DMAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN2_A::DISABLED)
    }
    #[doc = "DAC channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN2_A::ENABLED)
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
#[doc = "Field `MAMP2` reader - DAC channel2 mask/amplitude selector"]
pub struct MAMP2_R(crate::FieldReader<u8, u8>);
impl MAMP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAMP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAMP2` writer - DAC channel2 mask/amplitude selector"]
pub struct MAMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "DAC channel2 noise/triangle wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVE2_A {
    #[doc = "0: Wave generation disabled"]
    DISABLED = 0,
    #[doc = "1: Noise wave generation enabled"]
    NOISE = 1,
    #[doc = "2: Triangle wave generation enabled"]
    TRIANGLE = 2,
}
impl From<WAVE2_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable"]
pub struct WAVE2_R(crate::FieldReader<u8, WAVE2_A>);
impl WAVE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVE2_A> {
        match self.bits {
            0 => Some(WAVE2_A::DISABLED),
            1 => Some(WAVE2_A::NOISE),
            2 => Some(WAVE2_A::TRIANGLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAVE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        **self == WAVE2_A::NOISE
    }
    #[doc = "Checks if the value of the field is `TRIANGLE`"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        **self == WAVE2_A::TRIANGLE
    }
}
impl core::ops::Deref for WAVE2_R {
    type Target = crate::FieldReader<u8, WAVE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable"]
pub struct WAVE2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE2_A::DISABLED)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE2_A::NOISE)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE2_A::TRIANGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "DAC channel2 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL2_A {
    #[doc = "0: Timer 6 TRGO event"]
    TIM6_TRGO = 0,
    #[doc = "1: Timer 8 TRGO event"]
    TIM8_TRGO = 1,
    #[doc = "2: Timer 7 TRGO event"]
    TIM7_TRGO = 2,
    #[doc = "3: Timer 5 TRGO event"]
    TIM5_TRGO = 3,
    #[doc = "4: Timer 2 TRGO event"]
    TIM2_TRGO = 4,
    #[doc = "5: Timer 4 TRGO event"]
    TIM4_TRGO = 5,
    #[doc = "6: EXTI line9"]
    EXTI9 = 6,
    #[doc = "7: Software trigger"]
    SOFTWARE = 7,
}
impl From<TSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSEL2` reader - DAC channel2 trigger selection"]
pub struct TSEL2_R(crate::FieldReader<u8, TSEL2_A>);
impl TSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEL2_A {
        match self.bits {
            0 => TSEL2_A::TIM6_TRGO,
            1 => TSEL2_A::TIM8_TRGO,
            2 => TSEL2_A::TIM7_TRGO,
            3 => TSEL2_A::TIM5_TRGO,
            4 => TSEL2_A::TIM2_TRGO,
            5 => TSEL2_A::TIM4_TRGO,
            6 => TSEL2_A::EXTI9,
            7 => TSEL2_A::SOFTWARE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIM6_TRGO`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        **self == TSEL2_A::TIM6_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM8_TRGO`"]
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        **self == TSEL2_A::TIM8_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM7_TRGO`"]
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        **self == TSEL2_A::TIM7_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM5_TRGO`"]
    #[inline(always)]
    pub fn is_tim5_trgo(&self) -> bool {
        **self == TSEL2_A::TIM5_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        **self == TSEL2_A::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM4_TRGO`"]
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        **self == TSEL2_A::TIM4_TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        **self == TSEL2_A::EXTI9
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == TSEL2_A::SOFTWARE
    }
}
impl core::ops::Deref for TSEL2_R {
    type Target = crate::FieldReader<u8, TSEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL2` writer - DAC channel2 trigger selection"]
pub struct TSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::TIM6_TRGO)
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::TIM8_TRGO)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::TIM7_TRGO)
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn tim5_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::TIM5_TRGO)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::TIM2_TRGO)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::TIM4_TRGO)
    }
    #[doc = "EXTI line9"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL2_A::EXTI9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL2_A::SOFTWARE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "DAC channel2 trigger enable"]
pub type TEN2_A = TEN1_A;
#[doc = "Field `TEN2` reader - DAC channel2 trigger enable"]
pub type TEN2_R = TEN1_R;
#[doc = "Field `TEN2` writer - DAC channel2 trigger enable"]
pub struct TEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN2_A::DISABLED)
    }
    #[doc = "DAC channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN2_A::ENABLED)
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
#[doc = "DAC channel2 output buffer disable"]
pub type BOFF2_A = BOFF1_A;
#[doc = "Field `BOFF2` reader - DAC channel2 output buffer disable"]
pub type BOFF2_R = BOFF1_R;
#[doc = "Field `BOFF2` writer - DAC channel2 output buffer disable"]
pub struct BOFF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF2_A::ENABLED)
    }
    #[doc = "DAC channel X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF2_A::DISABLED)
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
#[doc = "DAC channel2 enable"]
pub type EN2_A = EN1_A;
#[doc = "Field `EN2` reader - DAC channel2 enable"]
pub type EN2_R = EN1_R;
#[doc = "Field `EN2` writer - DAC channel2 enable"]
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN2_A::DISABLED)
    }
    #[doc = "DAC channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN2_A::ENABLED)
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
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE1_A {
    #[doc = "0: DAC channel X DMA Underrun Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel X DMA Underrun Interrupt enabled"]
    ENABLED = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub struct DMAUDRIE1_R(crate::FieldReader<bool, DMAUDRIE1_A>);
impl DMAUDRIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDRIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::DISABLED,
            true => DMAUDRIE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAUDRIE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAUDRIE1_A::ENABLED
    }
}
impl core::ops::Deref for DMAUDRIE1_R {
    type Target = crate::FieldReader<bool, DMAUDRIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub struct DMAUDRIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDRIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::DISABLED)
    }
    #[doc = "DAC channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN1_A {
    #[doc = "0: DAC channel X DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel X DMA mode enabled"]
    ENABLED = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub struct DMAEN1_R(crate::FieldReader<bool, DMAEN1_A>);
impl DMAEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::DISABLED,
            true => DMAEN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN1_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN1_R {
    type Target = crate::FieldReader<bool, DMAEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub struct DMAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::DISABLED)
    }
    #[doc = "DAC channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub struct MAMP1_R(crate::FieldReader<u8, u8>);
impl MAMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub struct MAMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVE1_A {
    #[doc = "0: Wave generation disabled"]
    DISABLED = 0,
    #[doc = "1: Noise wave generation enabled"]
    NOISE = 1,
    #[doc = "2: Triangle wave generation enabled"]
    TRIANGLE = 2,
}
impl From<WAVE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub struct WAVE1_R(crate::FieldReader<u8, WAVE1_A>);
impl WAVE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVE1_A> {
        match self.bits {
            0 => Some(WAVE1_A::DISABLED),
            1 => Some(WAVE1_A::NOISE),
            2 => Some(WAVE1_A::TRIANGLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAVE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        **self == WAVE1_A::NOISE
    }
    #[doc = "Checks if the value of the field is `TRIANGLE`"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        **self == WAVE1_A::TRIANGLE
    }
}
impl core::ops::Deref for WAVE1_R {
    type Target = crate::FieldReader<u8, WAVE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub struct WAVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE1_A::DISABLED)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE1_A::NOISE)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE1_A::TRIANGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: Timer 6 TRGO event"]
    TIM6_TRGO = 0,
    #[doc = "1: Timer 3 TRGO event"]
    TIM3_TRGO = 1,
    #[doc = "2: Timer 7 TRGO event"]
    TIM7_TRGO = 2,
    #[doc = "3: Timer 15 TRGO event"]
    TIM15_TRGO = 3,
    #[doc = "4: Timer 2 TRGO event"]
    TIM2_TRGO = 4,
    #[doc = "6: EXTI line9"]
    EXTI9 = 6,
    #[doc = "7: Software trigger"]
    SOFTWARE = 7,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub struct TSEL1_R(crate::FieldReader<u8, TSEL1_A>);
impl TSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEL1_A> {
        match self.bits {
            0 => Some(TSEL1_A::TIM6_TRGO),
            1 => Some(TSEL1_A::TIM3_TRGO),
            2 => Some(TSEL1_A::TIM7_TRGO),
            3 => Some(TSEL1_A::TIM15_TRGO),
            4 => Some(TSEL1_A::TIM2_TRGO),
            6 => Some(TSEL1_A::EXTI9),
            7 => Some(TSEL1_A::SOFTWARE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIM6_TRGO`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        **self == TSEL1_A::TIM6_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM3_TRGO`"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        **self == TSEL1_A::TIM3_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM7_TRGO`"]
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        **self == TSEL1_A::TIM7_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM15_TRGO`"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        **self == TSEL1_A::TIM15_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        **self == TSEL1_A::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        **self == TSEL1_A::EXTI9
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == TSEL1_A::SOFTWARE
    }
}
impl core::ops::Deref for TSEL1_R {
    type Target = crate::FieldReader<u8, TSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub struct TSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM6_TRGO)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM3_TRGO)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM7_TRGO)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM15_TRGO)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM2_TRGO)
    }
    #[doc = "EXTI line9"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1_A::EXTI9)
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL1_A::SOFTWARE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN1_A {
    #[doc = "0: DAC channel X trigger disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel X trigger enabled"]
    ENABLED = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub struct TEN1_R(crate::FieldReader<bool, TEN1_A>);
impl TEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::DISABLED,
            true => TEN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TEN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TEN1_A::ENABLED
    }
}
impl core::ops::Deref for TEN1_R {
    type Target = crate::FieldReader<bool, TEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub struct TEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1_A::DISABLED)
    }
    #[doc = "DAC channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1_A::ENABLED)
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
#[doc = "DAC channel1 output buffer disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF1_A {
    #[doc = "0: DAC channel X output buffer enabled"]
    ENABLED = 0,
    #[doc = "1: DAC channel X output buffer disabled"]
    DISABLED = 1,
}
impl From<BOFF1_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFF1` reader - DAC channel1 output buffer disable"]
pub struct BOFF1_R(crate::FieldReader<bool, BOFF1_A>);
impl BOFF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOFF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFF1_A {
        match self.bits {
            false => BOFF1_A::ENABLED,
            true => BOFF1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BOFF1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BOFF1_A::DISABLED
    }
}
impl core::ops::Deref for BOFF1_R {
    type Target = crate::FieldReader<bool, BOFF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFF1` writer - DAC channel1 output buffer disable"]
pub struct BOFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X output buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF1_A::ENABLED)
    }
    #[doc = "DAC channel X output buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF1_A::DISABLED)
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
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1_A {
    #[doc = "0: DAC channel X disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel X enabled"]
    ENABLED = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub struct EN1_R(crate::FieldReader<bool, EN1_A>);
impl EN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::DISABLED,
            true => EN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN1_A::ENABLED
    }
}
impl core::ops::Deref for EN1_R {
    type Target = crate::FieldReader<bool, EN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1_A::DISABLED)
    }
    #[doc = "DAC channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1_A::ENABLED)
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
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline(always)]
    pub fn boff2(&self) -> BOFF2_R {
        BOFF2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W {
        DMAUDRIE2_W { w: self }
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W {
        DMAEN2_W { w: self }
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP2_W {
        MAMP2_W { w: self }
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE2_W {
        WAVE2_W { w: self }
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    pub fn tsel2(&mut self) -> TSEL2_W {
        TSEL2_W { w: self }
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN2_W {
        TEN2_W { w: self }
    }
    #[doc = "Bit 17 - DAC channel2 output buffer disable"]
    #[inline(always)]
    pub fn boff2(&mut self) -> BOFF2_W {
        BOFF2_W { w: self }
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W {
        DMAUDRIE1_W { w: self }
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W {
        DMAEN1_W { w: self }
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W {
        MAMP1_W { w: self }
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W {
        WAVE1_W { w: self }
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W {
        TSEL1_W { w: self }
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W {
        TEN1_W { w: self }
    }
    #[doc = "Bit 1 - DAC channel1 output buffer disable"]
    #[inline(always)]
    pub fn boff1(&mut self) -> BOFF1_W {
        BOFF1_W { w: self }
    }
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
