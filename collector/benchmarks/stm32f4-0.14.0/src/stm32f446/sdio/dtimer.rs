#[doc = "Register `DTIMER` reader"]
pub struct R(crate::R<DTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTIMER` writer"]
pub struct W(crate::W<DTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTIMER_SPEC>;
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
impl From<crate::W<DTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATATIME` reader - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
pub struct DATATIME_R(crate::FieldReader<u32, u32>);
impl DATATIME_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATATIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATATIME_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATATIME` writer - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
pub struct DATATIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn datatime(&mut self) -> DATATIME_W {
        DATATIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtimer](index.html) module"]
pub struct DTIMER_SPEC;
impl crate::RegisterSpec for DTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtimer::R](R) reader structure"]
impl crate::Readable for DTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtimer::W](W) writer structure"]
impl crate::Writable for DTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTIMER to value 0"]
impl crate::Resettable for DTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
