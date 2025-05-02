#[doc = "Register `BDCR` reader"]
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCR` writer"]
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External low-speed oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator Off"]
    Off = 0,
    #[doc = "1: LSE oscillator On"]
    On = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEON` reader - External low-speed oscillator enable"]
pub type LSEON_R = crate::BitReader<LSEON_A>;
impl LSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::Off,
            true => LSEON_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON_A::On
    }
}
#[doc = "Field `LSEON` writer - External low-speed oscillator enable"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEON_A, O>;
impl<'a, const O: u8> LSEON_W<'a, O> {
    #[doc = "LSE oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::Off)
    }
    #[doc = "LSE oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::On)
    }
}
#[doc = "External low-speed oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDY_A {
    #[doc = "0: LSE oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSE oscillator ready"]
    Ready = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDY` reader - External low-speed oscillator ready"]
pub type LSERDY_R = crate::BitReader<LSERDY_A>;
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::NotReady,
            true => LSERDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDY_A::Ready
    }
}
#[doc = "External low-speed oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: LSE crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - External low-speed oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
impl LSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::NotBypassed,
            true => LSEBYP_A::Bypassed,
        }
    }
    #[doc = "Checks if the value of the field is `NotBypassed`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP_A::NotBypassed
    }
    #[doc = "Checks if the value of the field is `Bypassed`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP_A::Bypassed
    }
}
#[doc = "Field `LSEBYP` writer - External low-speed oscillator bypass"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEBYP_A, O>;
impl<'a, const O: u8> LSEBYP_W<'a, O> {
    #[doc = "LSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NotBypassed)
    }
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::Bypassed)
    }
}
#[doc = "External low-speed oscillator mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEMOD_A {
    #[doc = "0: LSE oscillator low power mode selection"]
    Low = 0,
    #[doc = "1: LSE oscillator high drive mode selection"]
    High = 1,
}
impl From<LSEMOD_A> for bool {
    #[inline(always)]
    fn from(variant: LSEMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEMOD` reader - External low-speed oscillator mode"]
pub type LSEMOD_R = crate::BitReader<LSEMOD_A>;
impl LSEMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEMOD_A {
        match self.bits {
            false => LSEMOD_A::Low,
            true => LSEMOD_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEMOD_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEMOD_A::High
    }
}
#[doc = "Field `LSEMOD` writer - External low-speed oscillator mode"]
pub type LSEMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEMOD_A, O>;
impl<'a, const O: u8> LSEMOD_W<'a, O> {
    #[doc = "LSE oscillator low power mode selection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEMOD_A::Low)
    }
    #[doc = "LSE oscillator high drive mode selection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEMOD_A::High)
    }
}
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "0: RTC clock disabled"]
    Disabled = 0,
    #[doc = "1: RTC clock enabled"]
    Enabled = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::Disabled,
            true => RTCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::Enabled
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, RTCEN_A, O>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Disabled)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Enabled)
    }
}
#[doc = "Backup domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRST_A {
    #[doc = "0: Reset not activated"]
    Disabled = 0,
    #[doc = "1: Reset the entire RTC domain"]
    Enabled = 1,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BDRST_R = crate::BitReader<BDRST_A>;
impl BDRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDRST_A {
        match self.bits {
            false => BDRST_A::Disabled,
            true => BDRST_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDRST_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDRST_A::Enabled
    }
}
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, BDRST_A, O>;
impl<'a, const O: u8> BDRST_W<'a, O> {
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDRST_A::Disabled)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDRST_A::Enabled)
    }
}
#[doc = "RTC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSEL_A {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: LSE oscillator clock used as RTC clock"]
    Lse = 1,
    #[doc = "2: LSI oscillator clock used as RTC clock"]
    Lsi = 2,
    #[doc = "3: HSE oscillator clock divided by a prescaler used as RTC clock"]
    Hse = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RTCSEL_R = crate::FieldReader<u8, RTCSEL_A>;
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NoClock,
            1 => RTCSEL_A::Lse,
            2 => RTCSEL_A::Lsi,
            3 => RTCSEL_A::Hse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoClock`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL_A::NoClock
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL_A::Lse
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL_A::Hse
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDCR_SPEC, u8, RTCSEL_A, 2, O>;
impl<'a, const O: u8> RTCSEL_W<'a, O> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NoClock)
    }
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lse)
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lsi)
    }
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Hse)
    }
}
impl R {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External low-speed oscillator mode"]
    #[inline(always)]
    pub fn lsemod(&self) -> LSEMOD_R {
        LSEMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bit 3 - External low-speed oscillator mode"]
    #[inline(always)]
    pub fn lsemod(&mut self) -> LSEMOD_W<3> {
        LSEMOD_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdcr::R](R) reader structure"]
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdcr::W](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
