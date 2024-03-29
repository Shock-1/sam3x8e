#[doc = "Reader of register MMR"]
pub type R = crate::R<u32, super::MMR>;
#[doc = "Writer for register MMR"]
pub type W = crate::W<u32, super::MMR>;
#[doc = "Register MMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `IADRSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IADRSZ_A {
    #[doc = "No internal device address"]
    NONE,
    #[doc = "One-byte internal device address"]
    _1_BYTE,
    #[doc = "Two-byte internal device address"]
    _2_BYTE,
    #[doc = "Three-byte internal device address"]
    _3_BYTE,
}
impl crate::ToBits<u8> for IADRSZ_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IADRSZ_A::NONE => 0,
            IADRSZ_A::_1_BYTE => 1,
            IADRSZ_A::_2_BYTE => 2,
            IADRSZ_A::_3_BYTE => 3,
        }
    }
}
#[doc = "Reader of field `IADRSZ`"]
pub type IADRSZ_R = crate::R<u8, IADRSZ_A>;
impl IADRSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IADRSZ_A {
        match self.bits {
            0 => IADRSZ_A::NONE,
            1 => IADRSZ_A::_1_BYTE,
            2 => IADRSZ_A::_2_BYTE,
            3 => IADRSZ_A::_3_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == IADRSZ_A::NONE
    }
    #[doc = "Checks if the value of the field is `_1_BYTE`"]
    #[inline(always)]
    pub fn is_1_byte(&self) -> bool {
        *self == IADRSZ_A::_1_BYTE
    }
    #[doc = "Checks if the value of the field is `_2_BYTE`"]
    #[inline(always)]
    pub fn is_2_byte(&self) -> bool {
        *self == IADRSZ_A::_2_BYTE
    }
    #[doc = "Checks if the value of the field is `_3_BYTE`"]
    #[inline(always)]
    pub fn is_3_byte(&self) -> bool {
        *self == IADRSZ_A::_3_BYTE
    }
}
#[doc = "Write proxy for field `IADRSZ`"]
pub struct IADRSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IADRSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IADRSZ_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(IADRSZ_A::NONE)
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn _1_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_1_BYTE)
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn _2_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_2_BYTE)
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn _3_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_3_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `MREAD`"]
pub type MREAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MREAD`"]
pub struct MREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> MREAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DADR`"]
pub type DADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DADR`"]
pub struct DADR_W<'a> {
    w: &'a mut W,
}
impl<'a> DADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&self) -> IADRSZ_R {
        IADRSZ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&self) -> MREAD_R {
        MREAD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&mut self) -> IADRSZ_W {
        IADRSZ_W { w: self }
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&mut self) -> MREAD_W {
        MREAD_W { w: self }
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&mut self) -> DADR_W {
        DADR_W { w: self }
    }
}
