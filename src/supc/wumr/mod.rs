#[doc = "Reader of register WUMR"]
pub type R = crate::R<u32, super::WUMR>;
#[doc = "Writer for register WUMR"]
pub type W = crate::W<u32, super::WUMR>;
#[doc = "Register WUMR `reset()`'s with value 0"]
impl crate::ResetValue for super::WUMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `FWUPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPEN_A {
    #[doc = "the Force Wake-up pin has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the Force Wake-up pin low forces the wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for FWUPEN_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FWUPEN_A::NOT_ENABLE => false,
            FWUPEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FWUPEN`"]
pub type FWUPEN_R = crate::R<bool, FWUPEN_A>;
impl FWUPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWUPEN_A {
        match self.bits {
            false => FWUPEN_A::NOT_ENABLE,
            true => FWUPEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == FWUPEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FWUPEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `FWUPEN`"]
pub struct FWUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FWUPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWUPEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the Force Wake-up pin has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(FWUPEN_A::NOT_ENABLE)
    }
    #[doc = "the Force Wake-up pin low forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FWUPEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `SMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMEN_A {
    #[doc = "the supply monitor detection has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for SMEN_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMEN_A::NOT_ENABLE => false,
            SMEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SMEN`"]
pub type SMEN_R = crate::R<bool, SMEN_A>;
impl SMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMEN_A {
        match self.bits {
            false => SMEN_A::NOT_ENABLE,
            true => SMEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SMEN`"]
pub struct SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMEN_A::NOT_ENABLE)
    }
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `RTTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTEN_A {
    #[doc = "the RTT alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for RTTEN_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTTEN_A::NOT_ENABLE => false,
            RTTEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RTTEN`"]
pub type RTTEN_R = crate::R<bool, RTTEN_A>;
impl RTTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTTEN_A {
        match self.bits {
            false => RTTEN_A::NOT_ENABLE,
            true => RTTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTTEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `RTTEN`"]
pub struct RTTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTTEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTEN_A::NOT_ENABLE)
    }
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "the RTC alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for RTCEN_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTCEN_A::NOT_ENABLE => false,
            RTCEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RTCEN`"]
pub type RTCEN_R = crate::R<bool, RTCEN_A>;
impl RTCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::NOT_ENABLE,
            true => RTCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `RTCEN`"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCEN_A::NOT_ENABLE)
    }
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `FWUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPDBC_A {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "FWUP shall be low for at least 3 SLCK periods"]
    _3_SCLK,
    #[doc = "FWUP shall be low for at least 32 SLCK periods"]
    _32_SCLK,
    #[doc = "FWUP shall be low for at least 512 SLCK periods"]
    _512_SCLK,
    #[doc = "FWUP shall be low for at least 4,096 SLCK periods"]
    _4096_SCLK,
    #[doc = "FWUP shall be low for at least 32,768 SLCK periods"]
    _32768_SCLK,
}
impl crate::ToBits<u8> for FWUPDBC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FWUPDBC_A::IMMEDIATE => 0,
            FWUPDBC_A::_3_SCLK => 1,
            FWUPDBC_A::_32_SCLK => 2,
            FWUPDBC_A::_512_SCLK => 3,
            FWUPDBC_A::_4096_SCLK => 4,
            FWUPDBC_A::_32768_SCLK => 5,
        }
    }
}
#[doc = "Reader of field `FWUPDBC`"]
pub type FWUPDBC_R = crate::R<u8, FWUPDBC_A>;
impl FWUPDBC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FWUPDBC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FWUPDBC_A::IMMEDIATE),
            1 => Val(FWUPDBC_A::_3_SCLK),
            2 => Val(FWUPDBC_A::_32_SCLK),
            3 => Val(FWUPDBC_A::_512_SCLK),
            4 => Val(FWUPDBC_A::_4096_SCLK),
            5 => Val(FWUPDBC_A::_32768_SCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == FWUPDBC_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SCLK`"]
    #[inline(always)]
    pub fn is_3_sclk(&self) -> bool {
        *self == FWUPDBC_A::_3_SCLK
    }
    #[doc = "Checks if the value of the field is `_32_SCLK`"]
    #[inline(always)]
    pub fn is_32_sclk(&self) -> bool {
        *self == FWUPDBC_A::_32_SCLK
    }
    #[doc = "Checks if the value of the field is `_512_SCLK`"]
    #[inline(always)]
    pub fn is_512_sclk(&self) -> bool {
        *self == FWUPDBC_A::_512_SCLK
    }
    #[doc = "Checks if the value of the field is `_4096_SCLK`"]
    #[inline(always)]
    pub fn is_4096_sclk(&self) -> bool {
        *self == FWUPDBC_A::_4096_SCLK
    }
    #[doc = "Checks if the value of the field is `_32768_SCLK`"]
    #[inline(always)]
    pub fn is_32768_sclk(&self) -> bool {
        *self == FWUPDBC_A::_32768_SCLK
    }
}
#[doc = "Write proxy for field `FWUPDBC`"]
pub struct FWUPDBC_W<'a> {
    w: &'a mut W,
}
impl<'a> FWUPDBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWUPDBC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(FWUPDBC_A::IMMEDIATE)
    }
    #[doc = "FWUP shall be low for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_sclk(self) -> &'a mut W {
        self.variant(FWUPDBC_A::_3_SCLK)
    }
    #[doc = "FWUP shall be low for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_sclk(self) -> &'a mut W {
        self.variant(FWUPDBC_A::_32_SCLK)
    }
    #[doc = "FWUP shall be low for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_sclk(self) -> &'a mut W {
        self.variant(FWUPDBC_A::_512_SCLK)
    }
    #[doc = "FWUP shall be low for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_sclk(self) -> &'a mut W {
        self.variant(FWUPDBC_A::_4096_SCLK)
    }
    #[doc = "FWUP shall be low for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_sclk(self) -> &'a mut W {
        self.variant(FWUPDBC_A::_32768_SCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPDBC_A {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SCLK,
}
impl crate::ToBits<u8> for WKUPDBC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WKUPDBC_A::IMMEDIATE => 0,
            WKUPDBC_A::_3_SCLK => 1,
            WKUPDBC_A::_32_SCLK => 2,
            WKUPDBC_A::_512_SCLK => 3,
            WKUPDBC_A::_4096_SCLK => 4,
            WKUPDBC_A::_32768_SCLK => 5,
        }
    }
}
#[doc = "Reader of field `WKUPDBC`"]
pub type WKUPDBC_R = crate::R<u8, WKUPDBC_A>;
impl WKUPDBC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WKUPDBC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WKUPDBC_A::IMMEDIATE),
            1 => Val(WKUPDBC_A::_3_SCLK),
            2 => Val(WKUPDBC_A::_32_SCLK),
            3 => Val(WKUPDBC_A::_512_SCLK),
            4 => Val(WKUPDBC_A::_4096_SCLK),
            5 => Val(WKUPDBC_A::_32768_SCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBC_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SCLK`"]
    #[inline(always)]
    pub fn is_3_sclk(&self) -> bool {
        *self == WKUPDBC_A::_3_SCLK
    }
    #[doc = "Checks if the value of the field is `_32_SCLK`"]
    #[inline(always)]
    pub fn is_32_sclk(&self) -> bool {
        *self == WKUPDBC_A::_32_SCLK
    }
    #[doc = "Checks if the value of the field is `_512_SCLK`"]
    #[inline(always)]
    pub fn is_512_sclk(&self) -> bool {
        *self == WKUPDBC_A::_512_SCLK
    }
    #[doc = "Checks if the value of the field is `_4096_SCLK`"]
    #[inline(always)]
    pub fn is_4096_sclk(&self) -> bool {
        *self == WKUPDBC_A::_4096_SCLK
    }
    #[doc = "Checks if the value of the field is `_32768_SCLK`"]
    #[inline(always)]
    pub fn is_32768_sclk(&self) -> bool {
        *self == WKUPDBC_A::_32768_SCLK
    }
}
#[doc = "Write proxy for field `WKUPDBC`"]
pub struct WKUPDBC_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPDBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPDBC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBC_A::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_3_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_512_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_4096_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_sclk(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32768_SCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force Wake-up Enable"]
    #[inline(always)]
    pub fn fwupen(&self) -> FWUPEN_R {
        FWUPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RTTEN_R {
        RTTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Force Wake-up Debouncer Period"]
    #[inline(always)]
    pub fn fwupdbc(&self) -> FWUPDBC_R {
        FWUPDBC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WKUPDBC_R {
        WKUPDBC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Wake-up Enable"]
    #[inline(always)]
    pub fn fwupen(&mut self) -> FWUPEN_W {
        FWUPEN_W { w: self }
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W {
        SMEN_W { w: self }
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&mut self) -> RTTEN_W {
        RTTEN_W { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Force Wake-up Debouncer Period"]
    #[inline(always)]
    pub fn fwupdbc(&mut self) -> FWUPDBC_W {
        FWUPDBC_W { w: self }
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&mut self) -> WKUPDBC_W {
        WKUPDBC_W { w: self }
    }
}
