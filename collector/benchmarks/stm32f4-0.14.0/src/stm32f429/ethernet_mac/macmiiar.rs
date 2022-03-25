#[doc = "Register `MACMIIAR` reader"]
pub struct R(crate::R<MACMIIAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMIIAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMIIAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMIIAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACMIIAR` writer"]
pub struct W(crate::W<MACMIIAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMIIAR_SPEC>;
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
impl From<crate::W<MACMIIAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMIIAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MII busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MB_A {
    #[doc = "1: This bit is set to 1 by the application to indicate that a read or write access is in progress"]
    BUSY = 1,
}
impl From<MB_A> for bool {
    #[inline(always)]
    fn from(variant: MB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB` reader - MII busy"]
pub struct MB_R(crate::FieldReader<bool, MB_A>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MB_A> {
        match self.bits {
            true => Some(MB_A::BUSY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == MB_A::BUSY
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, MB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB` writer - MII busy"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This bit is set to 1 by the application to indicate that a read or write access is in progress"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(MB_A::BUSY)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "MII write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MW_A {
    #[doc = "0: Read operation"]
    READ = 0,
    #[doc = "1: Write operation"]
    WRITE = 1,
}
impl From<MW_A> for bool {
    #[inline(always)]
    fn from(variant: MW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MW` reader - MII write"]
pub struct MW_R(crate::FieldReader<bool, MW_A>);
impl MW_R {
    pub(crate) fn new(bits: bool) -> Self {
        MW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MW_A {
        match self.bits {
            false => MW_A::READ,
            true => MW_A::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == MW_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == MW_A::WRITE
    }
}
impl core::ops::Deref for MW_R {
    type Target = crate::FieldReader<bool, MW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MW` writer - MII write"]
pub struct MW_W<'a> {
    w: &'a mut W,
}
impl<'a> MW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(MW_A::READ)
    }
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(MW_A::WRITE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Clock range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CR_A {
    #[doc = "0: 60-100MHz HCLK/42"]
    CR_60_100 = 0,
    #[doc = "1: 100-150 MHz HCLK/62"]
    CR_100_150 = 1,
    #[doc = "2: 20-35MHz HCLK/16"]
    CR_20_35 = 2,
    #[doc = "3: 35-60MHz HCLK/16"]
    CR_35_60 = 3,
    #[doc = "4: 150-168MHz HCLK/102"]
    CR_150_168 = 4,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CR` reader - Clock range"]
pub struct CR_R(crate::FieldReader<u8, CR_A>);
impl CR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CR_A> {
        match self.bits {
            0 => Some(CR_A::CR_60_100),
            1 => Some(CR_A::CR_100_150),
            2 => Some(CR_A::CR_20_35),
            3 => Some(CR_A::CR_35_60),
            4 => Some(CR_A::CR_150_168),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CR_60_100`"]
    #[inline(always)]
    pub fn is_cr_60_100(&self) -> bool {
        **self == CR_A::CR_60_100
    }
    #[doc = "Checks if the value of the field is `CR_100_150`"]
    #[inline(always)]
    pub fn is_cr_100_150(&self) -> bool {
        **self == CR_A::CR_100_150
    }
    #[doc = "Checks if the value of the field is `CR_20_35`"]
    #[inline(always)]
    pub fn is_cr_20_35(&self) -> bool {
        **self == CR_A::CR_20_35
    }
    #[doc = "Checks if the value of the field is `CR_35_60`"]
    #[inline(always)]
    pub fn is_cr_35_60(&self) -> bool {
        **self == CR_A::CR_35_60
    }
    #[doc = "Checks if the value of the field is `CR_150_168`"]
    #[inline(always)]
    pub fn is_cr_150_168(&self) -> bool {
        **self == CR_A::CR_150_168
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<u8, CR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR` writer - Clock range"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "60-100MHz HCLK/42"]
    #[inline(always)]
    pub fn cr_60_100(self) -> &'a mut W {
        self.variant(CR_A::CR_60_100)
    }
    #[doc = "100-150 MHz HCLK/62"]
    #[inline(always)]
    pub fn cr_100_150(self) -> &'a mut W {
        self.variant(CR_A::CR_100_150)
    }
    #[doc = "20-35MHz HCLK/16"]
    #[inline(always)]
    pub fn cr_20_35(self) -> &'a mut W {
        self.variant(CR_A::CR_20_35)
    }
    #[doc = "35-60MHz HCLK/16"]
    #[inline(always)]
    pub fn cr_35_60(self) -> &'a mut W {
        self.variant(CR_A::CR_35_60)
    }
    #[doc = "150-168MHz HCLK/102"]
    #[inline(always)]
    pub fn cr_150_168(self) -> &'a mut W {
        self.variant(CR_A::CR_150_168)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `MR` reader - MII register - select the desired MII register in the PHY device"]
pub struct MR_R(crate::FieldReader<u8, u8>);
impl MR_R {
    pub(crate) fn new(bits: u8) -> Self {
        MR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR` writer - MII register - select the desired MII register in the PHY device"]
pub struct MR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `PA` reader - PHY address - select which of possible 32 PHYs is being accessed"]
pub struct PA_R(crate::FieldReader<u8, u8>);
impl PA_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA` writer - PHY address - select which of possible 32 PHYs is being accessed"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 6:10 - MII register - select the desired MII register in the PHY device"]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W {
        MW_W { w: self }
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bits 6:10 - MII register - select the desired MII register in the PHY device"]
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W {
        MR_W { w: self }
    }
    #[doc = "Bits 11:15 - PHY address - select which of possible 32 PHYs is being accessed"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MII address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmiiar](index.html) module"]
pub struct MACMIIAR_SPEC;
impl crate::RegisterSpec for MACMIIAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macmiiar::R](R) reader structure"]
impl crate::Readable for MACMIIAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macmiiar::W](W) writer structure"]
impl crate::Writable for MACMIIAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACMIIAR to value 0"]
impl crate::Resettable for MACMIIAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
