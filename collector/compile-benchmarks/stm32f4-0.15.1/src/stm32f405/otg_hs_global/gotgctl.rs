#[doc = "Register `GOTGCTL` reader"]
pub struct R(crate::R<GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGCTL` writer"]
pub struct W(crate::W<GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGCTL_SPEC>;
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
impl From<crate::W<GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRQSCS` reader - Session request success"]
pub type SRQSCS_R = crate::BitReader<bool>;
#[doc = "Field `SRQ` reader - Session request"]
pub type SRQ_R = crate::BitReader<bool>;
#[doc = "Field `SRQ` writer - Session request"]
pub type SRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `HNGSCS` reader - Host negotiation success"]
pub type HNGSCS_R = crate::BitReader<bool>;
#[doc = "Field `HNPRQ` reader - HNP request"]
pub type HNPRQ_R = crate::BitReader<bool>;
#[doc = "Field `HNPRQ` writer - HNP request"]
pub type HNPRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `HSHNPEN` reader - Host set HNP enable"]
pub type HSHNPEN_R = crate::BitReader<bool>;
#[doc = "Field `HSHNPEN` writer - Host set HNP enable"]
pub type HSHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DHNPEN_R = crate::BitReader<bool>;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GOTGCTL_SPEC, bool, O>;
#[doc = "Field `CIDSTS` reader - Connector ID status"]
pub type CIDSTS_R = crate::BitReader<bool>;
#[doc = "Field `DBCT` reader - Long/short debounce time"]
pub type DBCT_R = crate::BitReader<bool>;
#[doc = "Field `ASVLD` reader - A-session valid"]
pub type ASVLD_R = crate::BitReader<bool>;
#[doc = "Field `BSVLD` reader - B-session valid"]
pub type BSVLD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W<1> {
        SRQ_W::new(self)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W<9> {
        HNPRQ_W::new(self)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<10> {
        HSHNPEN_W::new(self)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W<11> {
        DHNPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](index.html) module"]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgctl::R](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x0800"]
impl crate::Resettable for GOTGCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
