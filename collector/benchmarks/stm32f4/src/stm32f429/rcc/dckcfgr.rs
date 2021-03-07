#[doc = "Reader of register DCKCFGR"]
pub type R = crate::R<u32, super::DCKCFGR>;
#[doc = "Writer for register DCKCFGR"]
pub type W = crate::W<u32, super::DCKCFGR>;
#[doc = "Register DCKCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCKCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PLLI2S division factor for SAI1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLI2SDIVQ_A {
    #[doc = "0: PLLI2SDIVQ = /1"]
    DIV1 = 0,
    #[doc = "1: PLLI2SDIVQ = /2"]
    DIV2 = 1,
    #[doc = "2: PLLI2SDIVQ = /3"]
    DIV3 = 2,
    #[doc = "3: PLLI2SDIVQ = /4"]
    DIV4 = 3,
    #[doc = "4: PLLI2SDIVQ = /5"]
    DIV5 = 4,
    #[doc = "5: PLLI2SDIVQ = /6"]
    DIV6 = 5,
    #[doc = "6: PLLI2SDIVQ = /7"]
    DIV7 = 6,
    #[doc = "7: PLLI2SDIVQ = /8"]
    DIV8 = 7,
    #[doc = "8: PLLI2SDIVQ = /9"]
    DIV9 = 8,
    #[doc = "9: PLLI2SDIVQ = /10"]
    DIV10 = 9,
    #[doc = "10: PLLI2SDIVQ = /11"]
    DIV11 = 10,
    #[doc = "11: PLLI2SDIVQ = /12"]
    DIV12 = 11,
    #[doc = "12: PLLI2SDIVQ = /13"]
    DIV13 = 12,
    #[doc = "13: PLLI2SDIVQ = /14"]
    DIV14 = 13,
    #[doc = "14: PLLI2SDIVQ = /15"]
    DIV15 = 14,
    #[doc = "15: PLLI2SDIVQ = /16"]
    DIV16 = 15,
    #[doc = "16: PLLI2SDIVQ = /17"]
    DIV17 = 16,
    #[doc = "17: PLLI2SDIVQ = /18"]
    DIV18 = 17,
    #[doc = "18: PLLI2SDIVQ = /19"]
    DIV19 = 18,
    #[doc = "19: PLLI2SDIVQ = /20"]
    DIV20 = 19,
    #[doc = "20: PLLI2SDIVQ = /21"]
    DIV21 = 20,
    #[doc = "21: PLLI2SDIVQ = /22"]
    DIV22 = 21,
    #[doc = "22: PLLI2SDIVQ = /23"]
    DIV23 = 22,
    #[doc = "23: PLLI2SDIVQ = /24"]
    DIV24 = 23,
    #[doc = "24: PLLI2SDIVQ = /25"]
    DIV25 = 24,
    #[doc = "25: PLLI2SDIVQ = /26"]
    DIV26 = 25,
    #[doc = "26: PLLI2SDIVQ = /27"]
    DIV27 = 26,
    #[doc = "27: PLLI2SDIVQ = /28"]
    DIV28 = 27,
    #[doc = "28: PLLI2SDIVQ = /29"]
    DIV29 = 28,
    #[doc = "29: PLLI2SDIVQ = /30"]
    DIV30 = 29,
    #[doc = "30: PLLI2SDIVQ = /31"]
    DIV31 = 30,
    #[doc = "31: PLLI2SDIVQ = /32"]
    DIV32 = 31,
}
impl From<PLLI2SDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SDIVQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLI2SDIVQ`"]
pub type PLLI2SDIVQ_R = crate::R<u8, PLLI2SDIVQ_A>;
impl PLLI2SDIVQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SDIVQ_A {
        match self.bits {
            0 => PLLI2SDIVQ_A::DIV1,
            1 => PLLI2SDIVQ_A::DIV2,
            2 => PLLI2SDIVQ_A::DIV3,
            3 => PLLI2SDIVQ_A::DIV4,
            4 => PLLI2SDIVQ_A::DIV5,
            5 => PLLI2SDIVQ_A::DIV6,
            6 => PLLI2SDIVQ_A::DIV7,
            7 => PLLI2SDIVQ_A::DIV8,
            8 => PLLI2SDIVQ_A::DIV9,
            9 => PLLI2SDIVQ_A::DIV10,
            10 => PLLI2SDIVQ_A::DIV11,
            11 => PLLI2SDIVQ_A::DIV12,
            12 => PLLI2SDIVQ_A::DIV13,
            13 => PLLI2SDIVQ_A::DIV14,
            14 => PLLI2SDIVQ_A::DIV15,
            15 => PLLI2SDIVQ_A::DIV16,
            16 => PLLI2SDIVQ_A::DIV17,
            17 => PLLI2SDIVQ_A::DIV18,
            18 => PLLI2SDIVQ_A::DIV19,
            19 => PLLI2SDIVQ_A::DIV20,
            20 => PLLI2SDIVQ_A::DIV21,
            21 => PLLI2SDIVQ_A::DIV22,
            22 => PLLI2SDIVQ_A::DIV23,
            23 => PLLI2SDIVQ_A::DIV24,
            24 => PLLI2SDIVQ_A::DIV25,
            25 => PLLI2SDIVQ_A::DIV26,
            26 => PLLI2SDIVQ_A::DIV27,
            27 => PLLI2SDIVQ_A::DIV28,
            28 => PLLI2SDIVQ_A::DIV29,
            29 => PLLI2SDIVQ_A::DIV30,
            30 => PLLI2SDIVQ_A::DIV31,
            31 => PLLI2SDIVQ_A::DIV32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV23
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV25
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV27
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV29
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV31
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLI2SDIVQ_A::DIV32
    }
}
#[doc = "Write proxy for field `PLLI2SDIVQ`"]
pub struct PLLI2SDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SDIVQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SDIVQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLLI2SDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV1)
    }
    #[doc = "PLLI2SDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV2)
    }
    #[doc = "PLLI2SDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV3)
    }
    #[doc = "PLLI2SDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV4)
    }
    #[doc = "PLLI2SDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV5)
    }
    #[doc = "PLLI2SDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV6)
    }
    #[doc = "PLLI2SDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV7)
    }
    #[doc = "PLLI2SDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV8)
    }
    #[doc = "PLLI2SDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV9)
    }
    #[doc = "PLLI2SDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV10)
    }
    #[doc = "PLLI2SDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV11)
    }
    #[doc = "PLLI2SDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV12)
    }
    #[doc = "PLLI2SDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV13)
    }
    #[doc = "PLLI2SDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV14)
    }
    #[doc = "PLLI2SDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV15)
    }
    #[doc = "PLLI2SDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV16)
    }
    #[doc = "PLLI2SDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV17)
    }
    #[doc = "PLLI2SDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV18)
    }
    #[doc = "PLLI2SDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV19)
    }
    #[doc = "PLLI2SDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV20)
    }
    #[doc = "PLLI2SDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV21)
    }
    #[doc = "PLLI2SDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV22)
    }
    #[doc = "PLLI2SDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV23)
    }
    #[doc = "PLLI2SDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV24)
    }
    #[doc = "PLLI2SDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV25)
    }
    #[doc = "PLLI2SDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV26)
    }
    #[doc = "PLLI2SDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV27)
    }
    #[doc = "PLLI2SDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV28)
    }
    #[doc = "PLLI2SDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV29)
    }
    #[doc = "PLLI2SDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV30)
    }
    #[doc = "PLLI2SDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV31)
    }
    #[doc = "PLLI2SDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLI2SDIVQ_A::DIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "PLLSAI division factor for SAI1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAIDIVQ_A {
    #[doc = "0: PLLSAIDIVQ = /1"]
    DIV1 = 0,
    #[doc = "1: PLLSAIDIVQ = /2"]
    DIV2 = 1,
    #[doc = "2: PLLSAIDIVQ = /3"]
    DIV3 = 2,
    #[doc = "3: PLLSAIDIVQ = /4"]
    DIV4 = 3,
    #[doc = "4: PLLSAIDIVQ = /5"]
    DIV5 = 4,
    #[doc = "5: PLLSAIDIVQ = /6"]
    DIV6 = 5,
    #[doc = "6: PLLSAIDIVQ = /7"]
    DIV7 = 6,
    #[doc = "7: PLLSAIDIVQ = /8"]
    DIV8 = 7,
    #[doc = "8: PLLSAIDIVQ = /9"]
    DIV9 = 8,
    #[doc = "9: PLLSAIDIVQ = /10"]
    DIV10 = 9,
    #[doc = "10: PLLSAIDIVQ = /11"]
    DIV11 = 10,
    #[doc = "11: PLLSAIDIVQ = /12"]
    DIV12 = 11,
    #[doc = "12: PLLSAIDIVQ = /13"]
    DIV13 = 12,
    #[doc = "13: PLLSAIDIVQ = /14"]
    DIV14 = 13,
    #[doc = "14: PLLSAIDIVQ = /15"]
    DIV15 = 14,
    #[doc = "15: PLLSAIDIVQ = /16"]
    DIV16 = 15,
    #[doc = "16: PLLSAIDIVQ = /17"]
    DIV17 = 16,
    #[doc = "17: PLLSAIDIVQ = /18"]
    DIV18 = 17,
    #[doc = "18: PLLSAIDIVQ = /19"]
    DIV19 = 18,
    #[doc = "19: PLLSAIDIVQ = /20"]
    DIV20 = 19,
    #[doc = "20: PLLSAIDIVQ = /21"]
    DIV21 = 20,
    #[doc = "21: PLLSAIDIVQ = /22"]
    DIV22 = 21,
    #[doc = "22: PLLSAIDIVQ = /23"]
    DIV23 = 22,
    #[doc = "23: PLLSAIDIVQ = /24"]
    DIV24 = 23,
    #[doc = "24: PLLSAIDIVQ = /25"]
    DIV25 = 24,
    #[doc = "25: PLLSAIDIVQ = /26"]
    DIV26 = 25,
    #[doc = "26: PLLSAIDIVQ = /27"]
    DIV27 = 26,
    #[doc = "27: PLLSAIDIVQ = /28"]
    DIV28 = 27,
    #[doc = "28: PLLSAIDIVQ = /29"]
    DIV29 = 28,
    #[doc = "29: PLLSAIDIVQ = /30"]
    DIV30 = 29,
    #[doc = "30: PLLSAIDIVQ = /31"]
    DIV31 = 30,
    #[doc = "31: PLLSAIDIVQ = /32"]
    DIV32 = 31,
}
impl From<PLLSAIDIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLSAIDIVQ`"]
pub type PLLSAIDIVQ_R = crate::R<u8, PLLSAIDIVQ_A>;
impl PLLSAIDIVQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVQ_A {
        match self.bits {
            0 => PLLSAIDIVQ_A::DIV1,
            1 => PLLSAIDIVQ_A::DIV2,
            2 => PLLSAIDIVQ_A::DIV3,
            3 => PLLSAIDIVQ_A::DIV4,
            4 => PLLSAIDIVQ_A::DIV5,
            5 => PLLSAIDIVQ_A::DIV6,
            6 => PLLSAIDIVQ_A::DIV7,
            7 => PLLSAIDIVQ_A::DIV8,
            8 => PLLSAIDIVQ_A::DIV9,
            9 => PLLSAIDIVQ_A::DIV10,
            10 => PLLSAIDIVQ_A::DIV11,
            11 => PLLSAIDIVQ_A::DIV12,
            12 => PLLSAIDIVQ_A::DIV13,
            13 => PLLSAIDIVQ_A::DIV14,
            14 => PLLSAIDIVQ_A::DIV15,
            15 => PLLSAIDIVQ_A::DIV16,
            16 => PLLSAIDIVQ_A::DIV17,
            17 => PLLSAIDIVQ_A::DIV18,
            18 => PLLSAIDIVQ_A::DIV19,
            19 => PLLSAIDIVQ_A::DIV20,
            20 => PLLSAIDIVQ_A::DIV21,
            21 => PLLSAIDIVQ_A::DIV22,
            22 => PLLSAIDIVQ_A::DIV23,
            23 => PLLSAIDIVQ_A::DIV24,
            24 => PLLSAIDIVQ_A::DIV25,
            25 => PLLSAIDIVQ_A::DIV26,
            26 => PLLSAIDIVQ_A::DIV27,
            27 => PLLSAIDIVQ_A::DIV28,
            28 => PLLSAIDIVQ_A::DIV29,
            29 => PLLSAIDIVQ_A::DIV30,
            30 => PLLSAIDIVQ_A::DIV31,
            31 => PLLSAIDIVQ_A::DIV32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV23
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV25
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV27
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV29
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV31
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLSAIDIVQ_A::DIV32
    }
}
#[doc = "Write proxy for field `PLLSAIDIVQ`"]
pub struct PLLSAIDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAIDIVQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLLSAIDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV1)
    }
    #[doc = "PLLSAIDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV2)
    }
    #[doc = "PLLSAIDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV3)
    }
    #[doc = "PLLSAIDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV4)
    }
    #[doc = "PLLSAIDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV5)
    }
    #[doc = "PLLSAIDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV6)
    }
    #[doc = "PLLSAIDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV7)
    }
    #[doc = "PLLSAIDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV8)
    }
    #[doc = "PLLSAIDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV9)
    }
    #[doc = "PLLSAIDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV10)
    }
    #[doc = "PLLSAIDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV11)
    }
    #[doc = "PLLSAIDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV12)
    }
    #[doc = "PLLSAIDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV13)
    }
    #[doc = "PLLSAIDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV14)
    }
    #[doc = "PLLSAIDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV15)
    }
    #[doc = "PLLSAIDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV16)
    }
    #[doc = "PLLSAIDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV17)
    }
    #[doc = "PLLSAIDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV18)
    }
    #[doc = "PLLSAIDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV19)
    }
    #[doc = "PLLSAIDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV20)
    }
    #[doc = "PLLSAIDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV21)
    }
    #[doc = "PLLSAIDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV22)
    }
    #[doc = "PLLSAIDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV23)
    }
    #[doc = "PLLSAIDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV24)
    }
    #[doc = "PLLSAIDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV25)
    }
    #[doc = "PLLSAIDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV26)
    }
    #[doc = "PLLSAIDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV27)
    }
    #[doc = "PLLSAIDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV28)
    }
    #[doc = "PLLSAIDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV29)
    }
    #[doc = "PLLSAIDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV30)
    }
    #[doc = "PLLSAIDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV31)
    }
    #[doc = "PLLSAIDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLSAIDIVQ_A::DIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "division factor for LCD_CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAIDIVR_A {
    #[doc = "0: PLLSAIDIVR = /2"]
    DIV2 = 0,
    #[doc = "1: PLLSAIDIVR = /4"]
    DIV4 = 1,
    #[doc = "2: PLLSAIDIVR = /8"]
    DIV8 = 2,
    #[doc = "3: PLLSAIDIVR = /16"]
    DIV16 = 3,
}
impl From<PLLSAIDIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIDIVR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLSAIDIVR`"]
pub type PLLSAIDIVR_R = crate::R<u8, PLLSAIDIVR_A>;
impl PLLSAIDIVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIDIVR_A {
        match self.bits {
            0 => PLLSAIDIVR_A::DIV2,
            1 => PLLSAIDIVR_A::DIV4,
            2 => PLLSAIDIVR_A::DIV8,
            3 => PLLSAIDIVR_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLSAIDIVR_A::DIV16
    }
}
#[doc = "Write proxy for field `PLLSAIDIVR`"]
pub struct PLLSAIDIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAIDIVR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLLSAIDIVR = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV2)
    }
    #[doc = "PLLSAIDIVR = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV4)
    }
    #[doc = "PLLSAIDIVR = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV8)
    }
    #[doc = "PLLSAIDIVR = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLSAIDIVR_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "SAI1-A clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1ASRC_A {
    #[doc = "0: SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    PLLSAI = 0,
    #[doc = "1: SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    PLLI2S = 1,
    #[doc = "2: SAI1-A clock frequency = Alternate function input frequency"]
    I2S_CKIN = 2,
}
impl From<SAI1ASRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1ASRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1ASRC`"]
pub type SAI1ASRC_R = crate::R<u8, SAI1ASRC_A>;
impl SAI1ASRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1ASRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1ASRC_A::PLLSAI),
            1 => Val(SAI1ASRC_A::PLLI2S),
            2 => Val(SAI1ASRC_A::I2S_CKIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1ASRC_A::PLLSAI
    }
    #[doc = "Checks if the value of the field is `PLLI2S`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1ASRC_A::PLLI2S
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1ASRC_A::I2S_CKIN
    }
}
#[doc = "Write proxy for field `SAI1ASRC`"]
pub struct SAI1ASRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1ASRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1ASRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::PLLSAI)
    }
    #[doc = "SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::PLLI2S)
    }
    #[doc = "SAI1-A clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::I2S_CKIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "SAI1-B clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1BSRC_A {
    #[doc = "0: SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    PLLSAI = 0,
    #[doc = "1: SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    PLLI2S = 1,
    #[doc = "2: SAI1-B clock frequency = Alternate function input frequency"]
    I2S_CKIN = 2,
}
impl From<SAI1BSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1BSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1BSRC`"]
pub type SAI1BSRC_R = crate::R<u8, SAI1BSRC_A>;
impl SAI1BSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1BSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1BSRC_A::PLLSAI),
            1 => Val(SAI1BSRC_A::PLLI2S),
            2 => Val(SAI1BSRC_A::I2S_CKIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1BSRC_A::PLLSAI
    }
    #[doc = "Checks if the value of the field is `PLLI2S`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1BSRC_A::PLLI2S
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1BSRC_A::I2S_CKIN
    }
}
#[doc = "Write proxy for field `SAI1BSRC`"]
pub struct SAI1BSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1BSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1BSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::PLLSAI)
    }
    #[doc = "SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::PLLI2S)
    }
    #[doc = "SAI1-B clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::I2S_CKIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Timers clocks prescalers selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPRE_A {
    #[doc = "0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    MUL2 = 0,
    #[doc = "1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    MUL4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMPRE`"]
pub type TIMPRE_R = crate::R<bool, TIMPRE_A>;
impl TIMPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::MUL2,
            true => TIMPRE_A::MUL4,
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == TIMPRE_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == TIMPRE_A::MUL4
    }
}
#[doc = "Write proxy for field `TIMPRE`"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL2)
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL4)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdivq(&self) -> PLLI2SDIVQ_R {
        PLLI2SDIVQ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PLLSAIDIVR_R {
        PLLSAIDIVR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - SAI1-A clock source selection"]
    #[inline(always)]
    pub fn sai1asrc(&self) -> SAI1ASRC_R {
        SAI1ASRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - SAI1-B clock source selection"]
    #[inline(always)]
    pub fn sai1bsrc(&self) -> SAI1BSRC_R {
        SAI1BSRC_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdivq(&mut self) -> PLLI2SDIVQ_W {
        PLLI2SDIVQ_W { w: self }
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W {
        PLLSAIDIVQ_W { w: self }
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&mut self) -> PLLSAIDIVR_W {
        PLLSAIDIVR_W { w: self }
    }
    #[doc = "Bits 20:21 - SAI1-A clock source selection"]
    #[inline(always)]
    pub fn sai1asrc(&mut self) -> SAI1ASRC_W {
        SAI1ASRC_W { w: self }
    }
    #[doc = "Bits 22:23 - SAI1-B clock source selection"]
    #[inline(always)]
    pub fn sai1bsrc(&mut self) -> SAI1BSRC_W {
        SAI1BSRC_W { w: self }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
}
