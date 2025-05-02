#[doc = "Register `DCFG` reader"]
pub struct R(crate::R<DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG` writer"]
pub struct W(crate::W<DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_SPEC>;
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
impl From<crate::W<DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSPD` reader - Device speed"]
pub type DSPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSPD` writer - Device speed"]
pub type DSPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `NZLSOHSK` reader - Nonzero-length status OUT handshake"]
pub type NZLSOHSK_R = crate::BitReader<bool>;
#[doc = "Field `NZLSOHSK` writer - Nonzero-length status OUT handshake"]
pub type NZLSOHSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_SPEC, bool, O>;
#[doc = "Field `DAD` reader - Device address"]
pub type DAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAD` writer - Device address"]
pub type DAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `PFIVL` reader - Periodic (micro)frame interval"]
pub type PFIVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFIVL` writer - Periodic (micro)frame interval"]
pub type PFIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `PERSCHIVL` reader - Periodic scheduling interval"]
pub type PERSCHIVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERSCHIVL` writer - Periodic scheduling interval"]
pub type PERSCHIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W<0> {
        DSPD_W::new(self)
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<2> {
        NZLSOHSK_W::new(self)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<4> {
        DAD_W::new(self)
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W<11> {
        PFIVL_W::new(self)
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
    #[inline(always)]
    pub fn perschivl(&mut self) -> PERSCHIVL_W<24> {
        PERSCHIVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](index.html) module"]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg::R](R) reader structure"]
impl crate::Readable for DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg::W](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCFG to value 0x0220_0000"]
impl crate::Resettable for DCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0220_0000
    }
}
