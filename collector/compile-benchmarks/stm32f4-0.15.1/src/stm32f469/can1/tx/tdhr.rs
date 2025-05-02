#[doc = "Register `TDHR` reader"]
pub struct R(crate::R<TDHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDHR` writer"]
pub struct W(crate::W<TDHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDHR_SPEC>;
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
impl From<crate::W<TDHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA7` reader - DATA7"]
pub type DATA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA7` writer - DATA7"]
pub type DATA7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDHR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA6` reader - DATA6"]
pub type DATA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA6` writer - DATA6"]
pub type DATA6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDHR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA5` reader - DATA5"]
pub type DATA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA5` writer - DATA5"]
pub type DATA5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDHR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type DATA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA4` writer - DATA4"]
pub type DATA4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDHR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W<24> {
        DATA7_W::new(self)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W<16> {
        DATA6_W::new(self)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W<8> {
        DATA5_W::new(self)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W<0> {
        DATA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdhr](index.html) module"]
pub struct TDHR_SPEC;
impl crate::RegisterSpec for TDHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdhr::R](R) reader structure"]
impl crate::Readable for TDHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdhr::W](W) writer structure"]
impl crate::Writable for TDHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TDHR to value 0"]
impl crate::Resettable for TDHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
