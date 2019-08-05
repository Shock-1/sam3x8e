#[doc = "Reader of register SMMR"]
pub type R = crate::R<u32, super::SMMR>;
#[doc = "Writer for register SMMR"]
pub type W = crate::W<u32, super::SMMR>;
#[doc = "Register SMMR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMTH`"]
pub type SMTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMTH`"]
pub struct SMTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `SMSMPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSMPL_A {
    #[doc = "Supply Monitor disabled"]
    SMD,
    #[doc = "Continuous Supply Monitor"]
    CSM,
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK,
}
impl crate::ToBits<u8> for SMSMPL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SMSMPL_A::SMD => 0,
            SMSMPL_A::CSM => 1,
            SMSMPL_A::_32SLCK => 2,
            SMSMPL_A::_256SLCK => 3,
            SMSMPL_A::_2048SLCK => 4,
        }
    }
}
#[doc = "Reader of field `SMSMPL`"]
pub type SMSMPL_R = crate::R<u8, SMSMPL_A>;
impl SMSMPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMSMPL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMSMPL_A::SMD),
            1 => Val(SMSMPL_A::CSM),
            2 => Val(SMSMPL_A::_32SLCK),
            3 => Val(SMSMPL_A::_256SLCK),
            4 => Val(SMSMPL_A::_2048SLCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SMD`"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        *self == SMSMPL_A::SMD
    }
    #[doc = "Checks if the value of the field is `CSM`"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        *self == SMSMPL_A::CSM
    }
    #[doc = "Checks if the value of the field is `_32SLCK`"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        *self == SMSMPL_A::_32SLCK
    }
    #[doc = "Checks if the value of the field is `_256SLCK`"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        *self == SMSMPL_A::_256SLCK
    }
    #[doc = "Checks if the value of the field is `_2048SLCK`"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        *self == SMSMPL_A::_2048SLCK
    }
}
#[doc = "Write proxy for field `SMSMPL`"]
pub struct SMSMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSMPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMSMPL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut W {
        self.variant(SMSMPL_A::SMD)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut W {
        self.variant(SMSMPL_A::CSM)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_32SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_256SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_2048SLCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `SMRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTEN_A {
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl crate::ToBits<bool> for SMRSTEN_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMRSTEN_A::NOT_ENABLE => false,
            SMRSTEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SMRSTEN`"]
pub type SMRSTEN_R = crate::R<bool, SMRSTEN_A>;
impl SMRSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTEN_A {
        match self.bits {
            false => SMRSTEN_A::NOT_ENABLE,
            true => SMRSTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMRSTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMRSTEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SMRSTEN`"]
pub struct SMRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMRSTEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::NOT_ENABLE)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `SMIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMIEN_A {
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl crate::ToBits<bool> for SMIEN_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMIEN_A::NOT_ENABLE => false,
            SMIEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SMIEN`"]
pub type SMIEN_R = crate::R<bool, SMIEN_A>;
impl SMIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMIEN_A {
        match self.bits {
            false => SMIEN_A::NOT_ENABLE,
            true => SMIEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMIEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMIEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SMIEN`"]
pub struct SMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMIEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMIEN_A::NOT_ENABLE)
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMIEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SMTH_R {
        SMTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SMSMPL_R {
        SMSMPL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SMRSTEN_R {
        SMRSTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SMIEN_R {
        SMIEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&mut self) -> SMTH_W {
        SMTH_W { w: self }
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&mut self) -> SMSMPL_W {
        SMSMPL_W { w: self }
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&mut self) -> SMRSTEN_W {
        SMRSTEN_W { w: self }
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&mut self) -> SMIEN_W {
        SMIEN_W { w: self }
    }
}
