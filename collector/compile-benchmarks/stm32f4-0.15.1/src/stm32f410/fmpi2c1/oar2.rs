#[doc = "Register `OAR2` reader"]
pub struct R(crate::R<OAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OAR2` writer"]
pub struct W(crate::W<OAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR2_SPEC>;
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
impl From<crate::W<OAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA2` reader - Interface address"]
pub type OA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA2` writer - Interface address"]
pub type OA2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OAR2_SPEC, u8, u8, 7, O>;
#[doc = "Own Address 2 masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA2MSK_A {
    #[doc = "0: No mask"]
    NoMask = 0,
    #[doc = "1: OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
    Mask1 = 1,
    #[doc = "2: OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
    Mask2 = 2,
    #[doc = "3: OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
    Mask3 = 3,
    #[doc = "4: OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
    Mask4 = 4,
    #[doc = "5: OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
    Mask5 = 5,
    #[doc = "6: OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
    Mask6 = 6,
    #[doc = "7: OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    Mask7 = 7,
}
impl From<OA2MSK_A> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OA2MSK` reader - Own Address 2 masks"]
pub type OA2MSK_R = crate::FieldReader<u8, OA2MSK_A>;
impl OA2MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA2MSK_A {
        match self.bits {
            0 => OA2MSK_A::NoMask,
            1 => OA2MSK_A::Mask1,
            2 => OA2MSK_A::Mask2,
            3 => OA2MSK_A::Mask3,
            4 => OA2MSK_A::Mask4,
            5 => OA2MSK_A::Mask5,
            6 => OA2MSK_A::Mask6,
            7 => OA2MSK_A::Mask7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoMask`"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == OA2MSK_A::NoMask
    }
    #[doc = "Checks if the value of the field is `Mask1`"]
    #[inline(always)]
    pub fn is_mask1(&self) -> bool {
        *self == OA2MSK_A::Mask1
    }
    #[doc = "Checks if the value of the field is `Mask2`"]
    #[inline(always)]
    pub fn is_mask2(&self) -> bool {
        *self == OA2MSK_A::Mask2
    }
    #[doc = "Checks if the value of the field is `Mask3`"]
    #[inline(always)]
    pub fn is_mask3(&self) -> bool {
        *self == OA2MSK_A::Mask3
    }
    #[doc = "Checks if the value of the field is `Mask4`"]
    #[inline(always)]
    pub fn is_mask4(&self) -> bool {
        *self == OA2MSK_A::Mask4
    }
    #[doc = "Checks if the value of the field is `Mask5`"]
    #[inline(always)]
    pub fn is_mask5(&self) -> bool {
        *self == OA2MSK_A::Mask5
    }
    #[doc = "Checks if the value of the field is `Mask6`"]
    #[inline(always)]
    pub fn is_mask6(&self) -> bool {
        *self == OA2MSK_A::Mask6
    }
    #[doc = "Checks if the value of the field is `Mask7`"]
    #[inline(always)]
    pub fn is_mask7(&self) -> bool {
        *self == OA2MSK_A::Mask7
    }
}
#[doc = "Field `OA2MSK` writer - Own Address 2 masks"]
pub type OA2MSK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OAR2_SPEC, u8, OA2MSK_A, 3, O>;
impl<'a, const O: u8> OA2MSK_W<'a, O> {
    #[doc = "No mask"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut W {
        self.variant(OA2MSK_A::NoMask)
    }
    #[doc = "OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
    #[inline(always)]
    pub fn mask1(self) -> &'a mut W {
        self.variant(OA2MSK_A::Mask1)
    }
    #[doc = "OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
    #[inline(always)]
    pub fn mask2(self) -> &'a mut W {
        self.variant(OA2MSK_A::Mask2)
    }
    #[doc = "OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
    #[inline(always)]
    pub fn mask3(self) -> &'a mut W {
        self.variant(OA2MSK_A::Mask3)
    }
    #[doc = "OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
    #[inline(always)]
    pub fn mask4(self) -> &'a mut W {
        self.variant(OA2MSK_A::Mask4)
    }
    #[doc = "OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
    #[inline(always)]
    pub fn mask5(self) -> &'a mut W {
        self.variant(OA2MSK_A::Mask5)
    }
    #[doc = "OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
    #[inline(always)]
    pub fn mask6(self) -> &'a mut W {
        self.variant(OA2MSK_A::Mask6)
    }
    #[doc = "OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    #[inline(always)]
    pub fn mask7(self) -> &'a mut W {
        self.variant(OA2MSK_A::Mask7)
    }
}
#[doc = "Own Address 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA2EN_A {
    #[doc = "0: Own address 2 disabled. The received slave address OA2 is NACKed"]
    Disabled = 0,
    #[doc = "1: Own address 2 enabled. The received slave address OA2 is ACKed"]
    Enabled = 1,
}
impl From<OA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub type OA2EN_R = crate::BitReader<OA2EN_A>;
impl OA2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA2EN_A {
        match self.bits {
            false => OA2EN_A::Disabled,
            true => OA2EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OA2EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OA2EN_A::Enabled
    }
}
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub type OA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR2_SPEC, OA2EN_A, O>;
impl<'a, const O: u8> OA2EN_W<'a, O> {
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OA2EN_A::Disabled)
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OA2EN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa2(&mut self) -> OA2_W<1> {
        OA2_W::new(self)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W<8> {
        OA2MSK_W::new(self)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W<15> {
        OA2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar2](index.html) module"]
pub struct OAR2_SPEC;
impl crate::RegisterSpec for OAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oar2::R](R) reader structure"]
impl crate::Readable for OAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oar2::W](W) writer structure"]
impl crate::Writable for OAR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for OAR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
