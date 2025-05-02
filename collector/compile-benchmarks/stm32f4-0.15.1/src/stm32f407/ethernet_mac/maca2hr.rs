#[doc = "Register `MACA2HR` reader"]
pub struct R(crate::R<MACA2HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA2HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA2HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA2HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA2HR` writer"]
pub struct W(crate::W<MACA2HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA2HR_SPEC>;
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
impl From<crate::W<MACA2HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA2HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA2H` reader - MAC2AH"]
pub type MACA2H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MACA2H` writer - MAC2AH"]
pub type MACA2H_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACA2HR_SPEC, u16, u16, 16, O>;
#[doc = "Field `MBC` reader - MBC"]
pub type MBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MBC` writer - MBC"]
pub type MBC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACA2HR_SPEC, u8, u8, 6, O>;
#[doc = "SA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_A {
    #[doc = "0: This address is used for comparison with DA fields of the received frame"]
    Destination = 0,
    #[doc = "1: This address is used for comparison with SA fields of received frames"]
    Source = 1,
}
impl From<SA_A> for bool {
    #[inline(always)]
    fn from(variant: SA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SA` reader - SA"]
pub type SA_R = crate::BitReader<SA_A>;
impl SA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SA_A {
        match self.bits {
            false => SA_A::Destination,
            true => SA_A::Source,
        }
    }
    #[doc = "Checks if the value of the field is `Destination`"]
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        *self == SA_A::Destination
    }
    #[doc = "Checks if the value of the field is `Source`"]
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == SA_A::Source
    }
}
#[doc = "Field `SA` writer - SA"]
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA2HR_SPEC, SA_A, O>;
impl<'a, const O: u8> SA_W<'a, O> {
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    #[inline(always)]
    pub fn destination(self) -> &'a mut W {
        self.variant(SA_A::Destination)
    }
    #[doc = "This address is used for comparison with SA fields of received frames"]
    #[inline(always)]
    pub fn source(self) -> &'a mut W {
        self.variant(SA_A::Source)
    }
}
#[doc = "AE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AE_A {
    #[doc = "0: Address filters ignore this address"]
    Disabled = 0,
    #[doc = "1: Address filters use this address"]
    Enabled = 1,
}
impl From<AE_A> for bool {
    #[inline(always)]
    fn from(variant: AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AE` reader - AE"]
pub type AE_R = crate::BitReader<AE_A>;
impl AE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AE_A {
        match self.bits {
            false => AE_A::Disabled,
            true => AE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AE_A::Enabled
    }
}
#[doc = "Field `AE` writer - AE"]
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA2HR_SPEC, AE_A, O>;
impl<'a, const O: u8> AE_W<'a, O> {
    #[doc = "Address filters ignore this address"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AE_A::Disabled)
    }
    #[doc = "Address filters use this address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AE_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:15 - MAC2AH"]
    #[inline(always)]
    pub fn maca2h(&self) -> MACA2H_R {
        MACA2H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC2AH"]
    #[inline(always)]
    pub fn maca2h(&mut self) -> MACA2H_W<0> {
        MACA2H_W::new(self)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W<24> {
        MBC_W::new(self)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W<30> {
        SA_W::new(self)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 2 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca2hr](index.html) module"]
pub struct MACA2HR_SPEC;
impl crate::RegisterSpec for MACA2HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca2hr::R](R) reader structure"]
impl crate::Readable for MACA2HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca2hr::W](W) writer structure"]
impl crate::Writable for MACA2HR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA2HR to value 0xffff"]
impl crate::Resettable for MACA2HR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
