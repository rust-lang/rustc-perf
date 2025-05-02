#[doc = "Register `BGPFCCR` reader"]
pub struct R(crate::R<BGPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGPFCCR` writer"]
pub struct W(crate::W<BGPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGPFCCR_SPEC>;
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
impl From<crate::W<BGPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALPHA` reader - Alpha value"]
pub type ALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALPHA` writer - Alpha value"]
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BGPFCCR_SPEC, u8, u8, 8, O>;
#[doc = "Alpha mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AM_A {
    #[doc = "0: No modification of alpha channel"]
    NoModify = 0,
    #[doc = "1: Replace with value in ALPHA\\[7:0\\]"]
    Replace = 1,
    #[doc = "2: Multiply with value in ALPHA\\[7:0\\]"]
    Multiply = 2,
}
impl From<AM_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AM` reader - Alpha mode"]
pub type AM_R = crate::FieldReader<u8, AM_A>;
impl AM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AM_A> {
        match self.bits {
            0 => Some(AM_A::NoModify),
            1 => Some(AM_A::Replace),
            2 => Some(AM_A::Multiply),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoModify`"]
    #[inline(always)]
    pub fn is_no_modify(&self) -> bool {
        *self == AM_A::NoModify
    }
    #[doc = "Checks if the value of the field is `Replace`"]
    #[inline(always)]
    pub fn is_replace(&self) -> bool {
        *self == AM_A::Replace
    }
    #[doc = "Checks if the value of the field is `Multiply`"]
    #[inline(always)]
    pub fn is_multiply(&self) -> bool {
        *self == AM_A::Multiply
    }
}
#[doc = "Field `AM` writer - Alpha mode"]
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPFCCR_SPEC, u8, AM_A, 2, O>;
impl<'a, const O: u8> AM_W<'a, O> {
    #[doc = "No modification of alpha channel"]
    #[inline(always)]
    pub fn no_modify(self) -> &'a mut W {
        self.variant(AM_A::NoModify)
    }
    #[doc = "Replace with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn replace(self) -> &'a mut W {
        self.variant(AM_A::Replace)
    }
    #[doc = "Multiply with value in ALPHA\\[7:0\\]"]
    #[inline(always)]
    pub fn multiply(self) -> &'a mut W {
        self.variant(AM_A::Multiply)
    }
}
#[doc = "Field `CS` reader - CLUT size"]
pub type CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS` writer - CLUT size"]
pub type CS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BGPFCCR_SPEC, u8, u8, 8, O>;
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "1: Start the automatic loading of the CLUT"]
    Start = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<START_A>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<START_A> {
        match self.bits {
            true => Some(START_A::Start),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::Start
    }
}
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, BGPFCCR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Start the automatic loading of the CLUT"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::Start)
    }
}
#[doc = "CLUT Color mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCM_A {
    #[doc = "0: CLUT color format ARGB8888"]
    Argb8888 = 0,
    #[doc = "1: CLUT color format RGB888"]
    Rgb888 = 1,
}
impl From<CCM_A> for bool {
    #[inline(always)]
    fn from(variant: CCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCM` reader - CLUT Color mode"]
pub type CCM_R = crate::BitReader<CCM_A>;
impl CCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCM_A {
        match self.bits {
            false => CCM_A::Argb8888,
            true => CCM_A::Rgb888,
        }
    }
    #[doc = "Checks if the value of the field is `Argb8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CCM_A::Argb8888
    }
    #[doc = "Checks if the value of the field is `Rgb888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CCM_A::Rgb888
    }
}
#[doc = "Field `CCM` writer - CLUT Color mode"]
pub type CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BGPFCCR_SPEC, CCM_A, O>;
impl<'a, const O: u8> CCM_W<'a, O> {
    #[doc = "CLUT color format ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CCM_A::Argb8888)
    }
    #[doc = "CLUT color format RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CCM_A::Rgb888)
    }
}
#[doc = "Color mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Color mode ARGB8888"]
    Argb8888 = 0,
    #[doc = "1: Color mode RGB888"]
    Rgb888 = 1,
    #[doc = "2: Color mode RGB565"]
    Rgb565 = 2,
    #[doc = "3: Color mode ARGB1555"]
    Argb1555 = 3,
    #[doc = "4: Color mode ARGB4444"]
    Argb4444 = 4,
    #[doc = "5: Color mode L8"]
    L8 = 5,
    #[doc = "6: Color mode AL44"]
    Al44 = 6,
    #[doc = "7: Color mode AL88"]
    Al88 = 7,
    #[doc = "8: Color mode L4"]
    L4 = 8,
    #[doc = "9: Color mode A8"]
    A8 = 9,
    #[doc = "10: Color mode A4"]
    A4 = 10,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CM` reader - Color mode"]
pub type CM_R = crate::FieldReader<u8, CM_A>;
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CM_A> {
        match self.bits {
            0 => Some(CM_A::Argb8888),
            1 => Some(CM_A::Rgb888),
            2 => Some(CM_A::Rgb565),
            3 => Some(CM_A::Argb1555),
            4 => Some(CM_A::Argb4444),
            5 => Some(CM_A::L8),
            6 => Some(CM_A::Al44),
            7 => Some(CM_A::Al88),
            8 => Some(CM_A::L4),
            9 => Some(CM_A::A8),
            10 => Some(CM_A::A4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Argb8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM_A::Argb8888
    }
    #[doc = "Checks if the value of the field is `Rgb888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM_A::Rgb888
    }
    #[doc = "Checks if the value of the field is `Rgb565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM_A::Rgb565
    }
    #[doc = "Checks if the value of the field is `Argb1555`"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM_A::Argb1555
    }
    #[doc = "Checks if the value of the field is `Argb4444`"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM_A::Argb4444
    }
    #[doc = "Checks if the value of the field is `L8`"]
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == CM_A::L8
    }
    #[doc = "Checks if the value of the field is `Al44`"]
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == CM_A::Al44
    }
    #[doc = "Checks if the value of the field is `Al88`"]
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == CM_A::Al88
    }
    #[doc = "Checks if the value of the field is `L4`"]
    #[inline(always)]
    pub fn is_l4(&self) -> bool {
        *self == CM_A::L4
    }
    #[doc = "Checks if the value of the field is `A8`"]
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == CM_A::A8
    }
    #[doc = "Checks if the value of the field is `A4`"]
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == CM_A::A4
    }
}
#[doc = "Field `CM` writer - Color mode"]
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPFCCR_SPEC, u8, CM_A, 4, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    #[doc = "Color mode ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CM_A::Argb8888)
    }
    #[doc = "Color mode RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CM_A::Rgb888)
    }
    #[doc = "Color mode RGB565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(CM_A::Rgb565)
    }
    #[doc = "Color mode ARGB1555"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(CM_A::Argb1555)
    }
    #[doc = "Color mode ARGB4444"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(CM_A::Argb4444)
    }
    #[doc = "Color mode L8"]
    #[inline(always)]
    pub fn l8(self) -> &'a mut W {
        self.variant(CM_A::L8)
    }
    #[doc = "Color mode AL44"]
    #[inline(always)]
    pub fn al44(self) -> &'a mut W {
        self.variant(CM_A::Al44)
    }
    #[doc = "Color mode AL88"]
    #[inline(always)]
    pub fn al88(self) -> &'a mut W {
        self.variant(CM_A::Al88)
    }
    #[doc = "Color mode L4"]
    #[inline(always)]
    pub fn l4(self) -> &'a mut W {
        self.variant(CM_A::L4)
    }
    #[doc = "Color mode A8"]
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(CM_A::A8)
    }
    #[doc = "Color mode A4"]
    #[inline(always)]
    pub fn a4(self) -> &'a mut W {
        self.variant(CM_A::A4)
    }
}
impl R {
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - CLUT Color mode"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<24> {
        ALPHA_W::new(self)
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<16> {
        AM_W::new(self)
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<8> {
        CS_W::new(self)
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<5> {
        START_W::new(self)
    }
    #[doc = "Bit 4 - CLUT Color mode"]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W<4> {
        CCM_W::new(self)
    }
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "background PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgpfccr](index.html) module"]
pub struct BGPFCCR_SPEC;
impl crate::RegisterSpec for BGPFCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgpfccr::R](R) reader structure"]
impl crate::Readable for BGPFCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgpfccr::W](W) writer structure"]
impl crate::Writable for BGPFCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGPFCCR to value 0"]
impl crate::Resettable for BGPFCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
