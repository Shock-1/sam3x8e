#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UPDTIM`"]
pub type UPDTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPDTIM`"]
pub struct UPDTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDTIM_W<'a> {
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
#[doc = "Reader of field `UPDCAL`"]
pub type UPDCAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPDCAL`"]
pub struct UPDCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDCAL_W<'a> {
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
#[doc = "Possible values of the field `TIMEVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEVSEL_A {
    #[doc = "Minute change"]
    MINUTE,
    #[doc = "Hour change"]
    HOUR,
    #[doc = "Every day at midnight"]
    MIDNIGHT,
    #[doc = "Every day at noon"]
    NOON,
}
impl crate::ToBits<u8> for TIMEVSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TIMEVSEL_A::MINUTE => 0,
            TIMEVSEL_A::HOUR => 1,
            TIMEVSEL_A::MIDNIGHT => 2,
            TIMEVSEL_A::NOON => 3,
        }
    }
}
#[doc = "Reader of field `TIMEVSEL`"]
pub type TIMEVSEL_R = crate::R<u8, TIMEVSEL_A>;
impl TIMEVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVSEL_A {
        match self.bits {
            0 => TIMEVSEL_A::MINUTE,
            1 => TIMEVSEL_A::HOUR,
            2 => TIMEVSEL_A::MIDNIGHT,
            3 => TIMEVSEL_A::NOON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MINUTE`"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == TIMEVSEL_A::MINUTE
    }
    #[doc = "Checks if the value of the field is `HOUR`"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == TIMEVSEL_A::HOUR
    }
    #[doc = "Checks if the value of the field is `MIDNIGHT`"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == TIMEVSEL_A::MIDNIGHT
    }
    #[doc = "Checks if the value of the field is `NOON`"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == TIMEVSEL_A::NOON
    }
}
#[doc = "Write proxy for field `TIMEVSEL`"]
pub struct TIMEVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVSEL_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::MINUTE)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::HOUR)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::MIDNIGHT)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut W {
        self.variant(TIMEVSEL_A::NOON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `CALEVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEVSEL_A {
    #[doc = "Week change (every Monday at time 00:00:00)"]
    WEEK,
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    MONTH,
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    YEAR,
}
impl crate::ToBits<u8> for CALEVSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CALEVSEL_A::WEEK => 0,
            CALEVSEL_A::MONTH => 1,
            CALEVSEL_A::YEAR => 2,
        }
    }
}
#[doc = "Reader of field `CALEVSEL`"]
pub type CALEVSEL_R = crate::R<u8, CALEVSEL_A>;
impl CALEVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CALEVSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CALEVSEL_A::WEEK),
            1 => Val(CALEVSEL_A::MONTH),
            2 => Val(CALEVSEL_A::YEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WEEK`"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == CALEVSEL_A::WEEK
    }
    #[doc = "Checks if the value of the field is `MONTH`"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == CALEVSEL_A::MONTH
    }
    #[doc = "Checks if the value of the field is `YEAR`"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == CALEVSEL_A::YEAR
    }
}
#[doc = "Write proxy for field `CALEVSEL`"]
pub struct CALEVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALEVSEL_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut W {
        self.variant(CALEVSEL_A::WEEK)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut W {
        self.variant(CALEVSEL_A::MONTH)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut W {
        self.variant(CALEVSEL_A::YEAR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&self) -> UPDTIM_R {
        UPDTIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&self) -> UPDCAL_R {
        UPDCAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&self) -> TIMEVSEL_R {
        TIMEVSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&self) -> CALEVSEL_R {
        CALEVSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&mut self) -> UPDTIM_W {
        UPDTIM_W { w: self }
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&mut self) -> UPDCAL_W {
        UPDCAL_W { w: self }
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&mut self) -> TIMEVSEL_W {
        TIMEVSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&mut self) -> CALEVSEL_W {
        CALEVSEL_W { w: self }
    }
}
