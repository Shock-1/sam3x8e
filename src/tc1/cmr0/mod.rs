#[doc = "Reader of register CMR0"]
pub type R = crate::R<u32, super::CMR0>;
#[doc = "Writer for register CMR0"]
pub type W = crate::W<u32, super::CMR0>;
#[doc = "Register CMR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `TCCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCLKS_A {
    #[doc = "Clock selected: TCLK1"]
    TIMER_CLOCK1,
    #[doc = "Clock selected: TCLK2"]
    TIMER_CLOCK2,
    #[doc = "Clock selected: TCLK3"]
    TIMER_CLOCK3,
    #[doc = "Clock selected: TCLK4"]
    TIMER_CLOCK4,
    #[doc = "Clock selected: TCLK5"]
    TIMER_CLOCK5,
    #[doc = "Clock selected: XC0"]
    XC0,
    #[doc = "Clock selected: XC1"]
    XC1,
    #[doc = "Clock selected: XC2"]
    XC2,
}
impl crate::ToBits<u8> for TCCLKS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TCCLKS_A::TIMER_CLOCK1 => 0,
            TCCLKS_A::TIMER_CLOCK2 => 1,
            TCCLKS_A::TIMER_CLOCK3 => 2,
            TCCLKS_A::TIMER_CLOCK4 => 3,
            TCCLKS_A::TIMER_CLOCK5 => 4,
            TCCLKS_A::XC0 => 5,
            TCCLKS_A::XC1 => 6,
            TCCLKS_A::XC2 => 7,
        }
    }
}
#[doc = "Reader of field `TCCLKS`"]
pub type TCCLKS_R = crate::R<u8, TCCLKS_A>;
impl TCCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCLKS_A {
        match self.bits {
            0 => TCCLKS_A::TIMER_CLOCK1,
            1 => TCCLKS_A::TIMER_CLOCK2,
            2 => TCCLKS_A::TIMER_CLOCK3,
            3 => TCCLKS_A::TIMER_CLOCK4,
            4 => TCCLKS_A::TIMER_CLOCK5,
            5 => TCCLKS_A::XC0,
            6 => TCCLKS_A::XC1,
            7 => TCCLKS_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKS_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKS_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKS_A::XC2
    }
}
#[doc = "Write proxy for field `TCCLKS`"]
pub struct TCCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCLKS_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock selected: TCLK1"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK1)
    }
    #[doc = "Clock selected: TCLK2"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK2)
    }
    #[doc = "Clock selected: TCLK3"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK3)
    }
    #[doc = "Clock selected: TCLK4"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK4)
    }
    #[doc = "Clock selected: TCLK5"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CLKI`"]
pub type CLKI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKI`"]
pub struct CLKI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKI_W<'a> {
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
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_A {
    #[doc = "The clock is not gated by an external signal."]
    NONE,
    #[doc = "XC0 is ANDed with the selected clock."]
    XC0,
    #[doc = "XC1 is ANDed with the selected clock."]
    XC1,
    #[doc = "XC2 is ANDed with the selected clock."]
    XC2,
}
impl crate::ToBits<u8> for BURST_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BURST_A::NONE => 0,
            BURST_A::XC0 => 1,
            BURST_A::XC1 => 2,
            BURST_A::XC2 => 3,
        }
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<u8, BURST_A>;
impl BURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            0 => BURST_A::NONE,
            1 => BURST_A::XC0,
            2 => BURST_A::XC1,
            3 => BURST_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BURST_A::NONE
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == BURST_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == BURST_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == BURST_A::XC2
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BURST_A::NONE)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(BURST_A::XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(BURST_A::XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(BURST_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LDBSTOP`"]
pub type LDBSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDBSTOP`"]
pub struct LDBSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDBSTOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LDBDIS`"]
pub type LDBDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDBDIS`"]
pub struct LDBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDBDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `ETRGEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGEDG_A {
    #[doc = "The clock is not gated by an external signal."]
    NONE,
    #[doc = "Rising edge"]
    RISING,
    #[doc = "Falling edge"]
    FALLING,
    #[doc = "Each edge"]
    EDGE,
}
impl crate::ToBits<u8> for ETRGEDG_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ETRGEDG_A::NONE => 0,
            ETRGEDG_A::RISING => 1,
            ETRGEDG_A::FALLING => 2,
            ETRGEDG_A::EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ETRGEDG`"]
pub type ETRGEDG_R = crate::R<u8, ETRGEDG_A>;
impl ETRGEDG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGEDG_A {
        match self.bits {
            0 => ETRGEDG_A::NONE,
            1 => ETRGEDG_A::RISING,
            2 => ETRGEDG_A::FALLING,
            3 => ETRGEDG_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ETRGEDG_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ETRGEDG_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ETRGEDG_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETRGEDG_A::EDGE
    }
}
#[doc = "Write proxy for field `ETRGEDG`"]
pub struct ETRGEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRGEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETRGEDG_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ETRGEDG_A::NONE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ETRGEDG_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ETRGEDG_A::FALLING)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETRGEDG_A::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ABETRG`"]
pub type ABETRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABETRG`"]
pub struct ABETRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ABETRG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CPCTRG`"]
pub type CPCTRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPCTRG`"]
pub struct CPCTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCTRG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAVE`"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `LDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRA_A {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge of TIOA"]
    RISING,
    #[doc = "Falling edge of TIOA"]
    FALLING,
    #[doc = "Each edge of TIOA"]
    EDGE,
}
impl crate::ToBits<u8> for LDRA_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LDRA_A::NONE => 0,
            LDRA_A::RISING => 1,
            LDRA_A::FALLING => 2,
            LDRA_A::EDGE => 3,
        }
    }
}
#[doc = "Reader of field `LDRA`"]
pub type LDRA_R = crate::R<u8, LDRA_A>;
impl LDRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRA_A {
        match self.bits {
            0 => LDRA_A::NONE,
            1 => LDRA_A::RISING,
            2 => LDRA_A::FALLING,
            3 => LDRA_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRA_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRA_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRA_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRA_A::EDGE
    }
}
#[doc = "Write proxy for field `LDRA`"]
pub struct LDRA_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRA_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRA_A::NONE)
    }
    #[doc = "Rising edge of TIOA"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRA_A::RISING)
    }
    #[doc = "Falling edge of TIOA"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRA_A::FALLING)
    }
    #[doc = "Each edge of TIOA"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRA_A::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `LDRB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRB_A {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge of TIOA"]
    RISING,
    #[doc = "Falling edge of TIOA"]
    FALLING,
    #[doc = "Each edge of TIOA"]
    EDGE,
}
impl crate::ToBits<u8> for LDRB_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LDRB_A::NONE => 0,
            LDRB_A::RISING => 1,
            LDRB_A::FALLING => 2,
            LDRB_A::EDGE => 3,
        }
    }
}
#[doc = "Reader of field `LDRB`"]
pub type LDRB_R = crate::R<u8, LDRB_A>;
impl LDRB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRB_A {
        match self.bits {
            0 => LDRB_A::NONE,
            1 => LDRB_A::RISING,
            2 => LDRB_A::FALLING,
            3 => LDRB_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRB_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRB_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRB_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRB_A::EDGE
    }
}
#[doc = "Write proxy for field `LDRB`"]
pub struct LDRB_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRB_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRB_A::NONE)
    }
    #[doc = "Rising edge of TIOA"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRB_A::RISING)
    }
    #[doc = "Falling edge of TIOA"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRB_A::FALLING)
    }
    #[doc = "Each edge of TIOA"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRB_A::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LDBSTOP_R {
        LDBSTOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LDBDIS_R {
        LDBDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> ETRGEDG_R {
        ETRGEDG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> ABETRG_R {
        ABETRG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CPCTRG_R {
        CPCTRG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LDRA_R {
        LDRA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LDRB_R {
        LDRB_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> TCCLKS_W {
        TCCLKS_W { w: self }
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> CLKI_W {
        CLKI_W { w: self }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&mut self) -> LDBSTOP_W {
        LDBSTOP_W { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&mut self) -> LDBDIS_W {
        LDBDIS_W { w: self }
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&mut self) -> ETRGEDG_W {
        ETRGEDG_W { w: self }
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&mut self) -> ABETRG_W {
        ABETRG_W { w: self }
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&mut self) -> CPCTRG_W {
        CPCTRG_W { w: self }
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&mut self) -> LDRA_W {
        LDRA_W { w: self }
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&mut self) -> LDRB_W {
        LDRB_W { w: self }
    }
}
