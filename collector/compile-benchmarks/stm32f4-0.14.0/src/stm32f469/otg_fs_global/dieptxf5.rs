#[doc = "Register `DIEPTXF5` reader"]
pub struct R(crate::R<DIEPTXF5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF5` writer"]
pub struct W(crate::W<DIEPTXF5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF5_SPEC>;
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
impl From<crate::W<DIEPTXF5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF5_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf5](index.html) module"]
pub struct DIEPTXF5_SPEC;
impl crate::RegisterSpec for DIEPTXF5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf5::R](R) reader structure"]
impl crate::Readable for DIEPTXF5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf5::W](W) writer structure"]
impl crate::Writable for DIEPTXF5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPTXF5 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
