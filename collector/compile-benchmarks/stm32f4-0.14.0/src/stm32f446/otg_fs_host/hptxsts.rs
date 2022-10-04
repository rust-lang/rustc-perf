#[doc = "Register `HPTXSTS` reader"]
pub struct R(crate::R<HPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPTXSTS` writer"]
pub struct W(crate::W<HPTXSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPTXSTS_SPEC>;
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
impl From<crate::W<HPTXSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPTXSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTXFSAVL` reader - Periodic transmit data FIFO space available"]
pub struct PTXFSAVL_R(crate::FieldReader<u16, u16>);
impl PTXFSAVL_R {
    pub(crate) fn new(bits: u16) -> Self {
        PTXFSAVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFSAVL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFSAVL` writer - Periodic transmit data FIFO space available"]
pub struct PTXFSAVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSAVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PTXQSAV` reader - Periodic transmit request queue space available"]
pub struct PTXQSAV_R(crate::FieldReader<u8, u8>);
impl PTXQSAV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTXQSAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXQSAV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXQTOP` reader - Top of the periodic transmit request queue"]
pub struct PTXQTOP_R(crate::FieldReader<u8, u8>);
impl PTXQTOP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTXQTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXQTOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfsavl(&mut self) -> PTXFSAVL_W {
        PTXFSAVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxsts](index.html) module"]
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptxsts::R](R) reader structure"]
impl crate::Readable for HPTXSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hptxsts::W](W) writer structure"]
impl crate::Writable for HPTXSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HPTXSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0100
    }
}
