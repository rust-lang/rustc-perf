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
#[doc = "Master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: Use UG bit from TIMx_EGR register"]
    Reset = 0,
    #[doc = "1: Use CNT bit from TIMx_CEN register"]
    Enable = 1,
    #[doc = "2: Use the update event"]
    Update = 2,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MMS_R = crate::FieldReader<u8, MMS_A>;
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMS_A> {
        match self.bits {
            0 => Some(MMS_A::Reset),
            1 => Some(MMS_A::Enable),
            2 => Some(MMS_A::Update),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::Reset
    }
    #[doc = "Checks if the value of the field is `Enable`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::Enable
    }
    #[doc = "Checks if the value of the field is `Update`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::Update
    }
}
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, MMS_A, 3, O>;
impl<'a, const O: u8> MMS_W<'a, O> {
    #[doc = "Use UG bit from TIMx_EGR register"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::Reset)
    }
    #[doc = "Use CNT bit from TIMx_CEN register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::Enable)
    }
    #[doc = "Use the update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::Update)
    }
}
impl R {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
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
