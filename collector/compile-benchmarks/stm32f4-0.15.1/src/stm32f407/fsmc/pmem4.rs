#[doc = "Register `PMEM4` reader"]
pub struct R(crate::R<PMEM4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMEM4` writer"]
pub struct W(crate::W<PMEM4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM4_SPEC>;
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
impl From<crate::W<PMEM4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMHIZ` reader - MEMHIZx"]
pub type MEMHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMHIZ` writer - MEMHIZx"]
pub type MEMHIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMHOLD` reader - MEMHOLDx"]
pub type MEMHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMHOLD` writer - MEMHOLDx"]
pub type MEMHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMWAIT` reader - MEMWAITx"]
pub type MEMWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMWAIT` writer - MEMWAITx"]
pub type MEMWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEMSET` reader - MEMSETx"]
pub type MEMSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEMSET` writer - MEMSETx"]
pub type MEMSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMEM4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhiz(&mut self) -> MEMHIZ_W<24> {
        MEMHIZ_W::new(self)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memhold(&mut self) -> MEMHOLD_W<16> {
        MEMHOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W<8> {
        MEMWAIT_W::new(self)
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W<0> {
        MEMSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common memory space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem4](index.html) module"]
pub struct PMEM4_SPEC;
impl crate::RegisterSpec for PMEM4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmem4::R](R) reader structure"]
impl crate::Readable for PMEM4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmem4::W](W) writer structure"]
impl crate::Writable for PMEM4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMEM4 to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
