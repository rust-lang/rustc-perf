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
pub struct OA2_R(crate::FieldReader<u8, u8>);
impl OA2_R {
    pub(crate) fn new(bits: u8) -> Self {
        OA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OA2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA2` writer - Interface address"]
pub struct OA2_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "Own Address 2 masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA2MSK_A {
    #[doc = "0: No mask"]
    NOMASK = 0,
    #[doc = "1: OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
    MASK1 = 1,
    #[doc = "2: OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
    MASK2 = 2,
    #[doc = "3: OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
    MASK3 = 3,
    #[doc = "4: OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
    MASK4 = 4,
    #[doc = "5: OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
    MASK5 = 5,
    #[doc = "6: OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
    MASK6 = 6,
    #[doc = "7: OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    MASK7 = 7,
}
impl From<OA2MSK_A> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OA2MSK` reader - Own Address 2 masks"]
pub struct OA2MSK_R(crate::FieldReader<u8, OA2MSK_A>);
impl OA2MSK_R {
    pub(crate) fn new(bits: u8) -> Self {
        OA2MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA2MSK_A {
        match self.bits {
            0 => OA2MSK_A::NOMASK,
            1 => OA2MSK_A::MASK1,
            2 => OA2MSK_A::MASK2,
            3 => OA2MSK_A::MASK3,
            4 => OA2MSK_A::MASK4,
            5 => OA2MSK_A::MASK5,
            6 => OA2MSK_A::MASK6,
            7 => OA2MSK_A::MASK7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOMASK`"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        **self == OA2MSK_A::NOMASK
    }
    #[doc = "Checks if the value of the field is `MASK1`"]
    #[inline(always)]
    pub fn is_mask1(&self) -> bool {
        **self == OA2MSK_A::MASK1
    }
    #[doc = "Checks if the value of the field is `MASK2`"]
    #[inline(always)]
    pub fn is_mask2(&self) -> bool {
        **self == OA2MSK_A::MASK2
    }
    #[doc = "Checks if the value of the field is `MASK3`"]
    #[inline(always)]
    pub fn is_mask3(&self) -> bool {
        **self == OA2MSK_A::MASK3
    }
    #[doc = "Checks if the value of the field is `MASK4`"]
    #[inline(always)]
    pub fn is_mask4(&self) -> bool {
        **self == OA2MSK_A::MASK4
    }
    #[doc = "Checks if the value of the field is `MASK5`"]
    #[inline(always)]
    pub fn is_mask5(&self) -> bool {
        **self == OA2MSK_A::MASK5
    }
    #[doc = "Checks if the value of the field is `MASK6`"]
    #[inline(always)]
    pub fn is_mask6(&self) -> bool {
        **self == OA2MSK_A::MASK6
    }
    #[doc = "Checks if the value of the field is `MASK7`"]
    #[inline(always)]
    pub fn is_mask7(&self) -> bool {
        **self == OA2MSK_A::MASK7
    }
}
impl core::ops::Deref for OA2MSK_R {
    type Target = crate::FieldReader<u8, OA2MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA2MSK` writer - Own Address 2 masks"]
pub struct OA2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA2MSK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No mask"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut W {
        self.variant(OA2MSK_A::NOMASK)
    }
    #[doc = "OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
    #[inline(always)]
    pub fn mask1(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK1)
    }
    #[doc = "OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
    #[inline(always)]
    pub fn mask2(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK2)
    }
    #[doc = "OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
    #[inline(always)]
    pub fn mask3(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK3)
    }
    #[doc = "OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
    #[inline(always)]
    pub fn mask4(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK4)
    }
    #[doc = "OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
    #[inline(always)]
    pub fn mask5(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK5)
    }
    #[doc = "OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
    #[inline(always)]
    pub fn mask6(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK6)
    }
    #[doc = "OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    #[inline(always)]
    pub fn mask7(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Own Address 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA2EN_A {
    #[doc = "0: Own address 2 disabled. The received slave address OA2 is NACKed"]
    DISABLED = 0,
    #[doc = "1: Own address 2 enabled. The received slave address OA2 is ACKed"]
    ENABLED = 1,
}
impl From<OA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub struct OA2EN_R(crate::FieldReader<bool, OA2EN_A>);
impl OA2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OA2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA2EN_A {
        match self.bits {
            false => OA2EN_A::DISABLED,
            true => OA2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OA2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OA2EN_A::ENABLED
    }
}
impl core::ops::Deref for OA2EN_R {
    type Target = crate::FieldReader<bool, OA2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub struct OA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OA2EN_A::DISABLED)
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OA2EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
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
        OA2MSK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa2(&mut self) -> OA2_W {
        OA2_W { w: self }
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W {
        OA2MSK_W { w: self }
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W {
        OA2EN_W { w: self }
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
