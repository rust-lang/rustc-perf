#[doc = "Register `PUPDR` reader"]
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUPDR` writer"]
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR15_A = PUPDR0_A;
#[doc = "Field `PUPDR15` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR15_R = PUPDR0_R;
#[doc = "Field `PUPDR15` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR15_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR15_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR15_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR14_A = PUPDR0_A;
#[doc = "Field `PUPDR14` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR14_R = PUPDR0_R;
#[doc = "Field `PUPDR14` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR14_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR14_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR14_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR13_A = PUPDR0_A;
#[doc = "Field `PUPDR13` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR13_R = PUPDR0_R;
#[doc = "Field `PUPDR13` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR13_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR13_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR13_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR12_A = PUPDR0_A;
#[doc = "Field `PUPDR12` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR12_R = PUPDR0_R;
#[doc = "Field `PUPDR12` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR12_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR12_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR12_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR11_A = PUPDR0_A;
#[doc = "Field `PUPDR11` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR11_R = PUPDR0_R;
#[doc = "Field `PUPDR11` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR11_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR11_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR11_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR10_A = PUPDR0_A;
#[doc = "Field `PUPDR10` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR10_R = PUPDR0_R;
#[doc = "Field `PUPDR10` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR10_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR10_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR10_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR9_A = PUPDR0_A;
#[doc = "Field `PUPDR9` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR9_R = PUPDR0_R;
#[doc = "Field `PUPDR9` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR9_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR9_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR9_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR8_A = PUPDR0_A;
#[doc = "Field `PUPDR8` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR8_R = PUPDR0_R;
#[doc = "Field `PUPDR8` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR8_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR8_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR8_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR7_A = PUPDR0_A;
#[doc = "Field `PUPDR7` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR7_R = PUPDR0_R;
#[doc = "Field `PUPDR7` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR7_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR7_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR7_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR6_A = PUPDR0_A;
#[doc = "Field `PUPDR6` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR6_R = PUPDR0_R;
#[doc = "Field `PUPDR6` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR6_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR6_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR6_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR5_A = PUPDR0_A;
#[doc = "Field `PUPDR5` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR5_R = PUPDR0_R;
#[doc = "Field `PUPDR5` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR5_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR5_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR5_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR4_A = PUPDR0_A;
#[doc = "Field `PUPDR4` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR4_R = PUPDR0_R;
#[doc = "Field `PUPDR4` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR4_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR4_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR4_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR3_A = PUPDR0_A;
#[doc = "Field `PUPDR3` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR3_R = PUPDR0_R;
#[doc = "Field `PUPDR3` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR3_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR3_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR3_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR2_A = PUPDR0_A;
#[doc = "Field `PUPDR2` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR2_R = PUPDR0_R;
#[doc = "Field `PUPDR2` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR2_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR2_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR2_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPDR1_A = PUPDR0_A;
#[doc = "Field `PUPDR1` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR1_R = PUPDR0_R;
#[doc = "Field `PUPDR1` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR1_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR1_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR1_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PUPDR0_A {
    #[doc = "0: No pull-up, pull-down"]
    FLOATING = 0,
    #[doc = "1: Pull-up"]
    PULLUP = 1,
    #[doc = "2: Pull-down"]
    PULLDOWN = 2,
}
impl From<PUPDR0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPDR0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PUPDR0` reader - Port x configuration bits (y = 0..15)"]
pub struct PUPDR0_R(crate::FieldReader<u8, PUPDR0_A>);
impl PUPDR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUPDR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUPDR0_A> {
        match self.bits {
            0 => Some(PUPDR0_A::FLOATING),
            1 => Some(PUPDR0_A::PULLUP),
            2 => Some(PUPDR0_A::PULLDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLOATING`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        **self == PUPDR0_A::FLOATING
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PUPDR0_A::PULLUP
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PUPDR0_A::PULLDOWN
    }
}
impl core::ops::Deref for PUPDR0_R {
    type Target = crate::FieldReader<u8, PUPDR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUPDR0` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPDR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPDR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR0_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR0_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR0_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR12_R {
        PUPDR12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR11_R {
        PUPDR11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR10_R {
        PUPDR10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR9_R {
        PUPDR9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR8_R {
        PUPDR8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr15(&mut self) -> PUPDR15_W {
        PUPDR15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr14(&mut self) -> PUPDR14_W {
        PUPDR14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr13(&mut self) -> PUPDR13_W {
        PUPDR13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr12(&mut self) -> PUPDR12_W {
        PUPDR12_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr11(&mut self) -> PUPDR11_W {
        PUPDR11_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr10(&mut self) -> PUPDR10_W {
        PUPDR10_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr9(&mut self) -> PUPDR9_W {
        PUPDR9_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr8(&mut self) -> PUPDR8_W {
        PUPDR8_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr7(&mut self) -> PUPDR7_W {
        PUPDR7_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr6(&mut self) -> PUPDR6_W {
        PUPDR6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr5(&mut self) -> PUPDR5_W {
        PUPDR5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr4(&mut self) -> PUPDR4_W {
        PUPDR4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR3_W {
        PUPDR3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr2(&mut self) -> PUPDR2_W {
        PUPDR2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR1_W {
        PUPDR1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR0_W {
        PUPDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](index.html) module"]
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pupdr::R](R) reader structure"]
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pupdr::W](W) writer structure"]
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUPDR to value 0x0100"]
impl crate::Resettable for PUPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
