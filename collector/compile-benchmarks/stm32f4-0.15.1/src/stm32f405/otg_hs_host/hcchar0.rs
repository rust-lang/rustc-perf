#[doc = "Register `HCCHAR0` reader"]
pub struct R(crate::R<HCCHAR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR0` writer"]
pub struct W(crate::W<HCCHAR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR0_SPEC>;
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
impl From<crate::W<HCCHAR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MPSIZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPSIZ` writer - Maximum packet size"]
pub type MPSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR0_SPEC, u16, u16, 11, O>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub type EPNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub type EPDIR_R = crate::BitReader<bool>;
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR0_SPEC, bool, O>;
#[doc = "Field `LSDEV` reader - Low-speed device"]
pub type LSDEV_R = crate::BitReader<bool>;
#[doc = "Field `LSDEV` writer - Low-speed device"]
pub type LSDEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR0_SPEC, bool, O>;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTYP` writer - Endpoint type"]
pub type EPTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `MC` reader - Multi Count (MC) / Error Count (EC)"]
pub type MC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MC` writer - Multi Count (MC) / Error Count (EC)"]
pub type MC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `DAD` reader - Device address"]
pub type DAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAD` writer - Device address"]
pub type DAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR0_SPEC, u8, u8, 7, O>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type ODDFRM_R = crate::BitReader<bool>;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type ODDFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR0_SPEC, bool, O>;
#[doc = "Field `CHDIS` reader - Channel disable"]
pub type CHDIS_R = crate::BitReader<bool>;
#[doc = "Field `CHDIS` writer - Channel disable"]
pub type CHDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR0_SPEC, bool, O>;
#[doc = "Field `CHENA` reader - Channel enable"]
pub type CHENA_R = crate::BitReader<bool>;
#[doc = "Field `CHENA` writer - Channel enable"]
pub type CHENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<0> {
        MPSIZ_W::new(self)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W<11> {
        EPNUM_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W<15> {
        EPDIR_W::new(self)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsdev(&mut self) -> LSDEV_W<17> {
        LSDEV_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<18> {
        EPTYP_W::new(self)
    }
    #[doc = "Bits 20:21 - Multi Count (MC) / Error Count (EC)"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W<20> {
        MC_W::new(self)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<22> {
        DAD_W::new(self)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> ODDFRM_W<29> {
        ODDFRM_W::new(self)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W<30> {
        CHDIS_W::new(self)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W<31> {
        CHENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host channel-0 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar0](index.html) module"]
pub struct HCCHAR0_SPEC;
impl crate::RegisterSpec for HCCHAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar0::R](R) reader structure"]
impl crate::Readable for HCCHAR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar0::W](W) writer structure"]
impl crate::Writable for HCCHAR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCHAR0 to value 0"]
impl crate::Resettable for HCCHAR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
