#[doc = "Reader of register RCMR"]
pub type R = crate::R<u32, super::RCMR>;
#[doc = "Writer for register RCMR"]
pub type W = crate::W<u32, super::RCMR>;
#[doc = "Register RCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKS_A {
    #[doc = "Divided Clock"]
    MCK,
    #[doc = "TK Clock signal"]
    TK,
    #[doc = "RK pin"]
    RK,
}
impl crate::ToBits<u8> for CKS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKS_A::MCK => 0,
            CKS_A::TK => 1,
            CKS_A::RK => 2,
        }
    }
}
#[doc = "Reader of field `CKS`"]
pub type CKS_R = crate::R<u8, CKS_A>;
impl CKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKS_A::MCK),
            1 => Val(CKS_A::TK),
            2 => Val(CKS_A::RK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CKS_A::MCK
    }
    #[doc = "Checks if the value of the field is `TK`"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == CKS_A::TK
    }
    #[doc = "Checks if the value of the field is `RK`"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == CKS_A::RK
    }
}
#[doc = "Write proxy for field `CKS`"]
pub struct CKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKS_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CKS_A::MCK)
    }
    #[doc = "TK Clock signal"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKS_A::TK)
    }
    #[doc = "RK pin"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKS_A::RK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `CKO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKO_A {
    #[doc = "None, RK pin is an input"]
    NONE,
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    CONTINUOUS,
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    TRANSFER,
}
impl crate::ToBits<u8> for CKO_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKO_A::NONE => 0,
            CKO_A::CONTINUOUS => 1,
            CKO_A::TRANSFER => 2,
        }
    }
}
#[doc = "Reader of field `CKO`"]
pub type CKO_R = crate::R<u8, CKO_A>;
impl CKO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKO_A::NONE),
            1 => Val(CKO_A::CONTINUOUS),
            2 => Val(CKO_A::TRANSFER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKO_A::NONE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKO_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == CKO_A::TRANSFER
    }
}
#[doc = "Write proxy for field `CKO`"]
pub struct CKO_W<'a> {
    w: &'a mut W,
}
impl<'a> CKO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKO_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None, RK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKO_A::NONE)
    }
    #[doc = "Continuous Receive Clock, RK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKO_A::CONTINUOUS)
    }
    #[doc = "Receive Clock only during data transfers, RK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(CKO_A::TRANSFER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `CKI`"]
pub type CKI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKI`"]
pub struct CKI_W<'a> {
    w: &'a mut W,
}
impl<'a> CKI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `CKG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKG_A {
    #[doc = "None"]
    CONTINUOUS,
    #[doc = "Receive Clock enabled only if RF Low"]
    EN_RF_LOW,
    #[doc = "Receive Clock enabled only if RF High"]
    EN_RF_HIGH,
}
impl crate::ToBits<u8> for CKG_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKG_A::CONTINUOUS => 0,
            CKG_A::EN_RF_LOW => 1,
            CKG_A::EN_RF_HIGH => 2,
        }
    }
}
#[doc = "Reader of field `CKG`"]
pub type CKG_R = crate::R<u8, CKG_A>;
impl CKG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKG_A::CONTINUOUS),
            1 => Val(CKG_A::EN_RF_LOW),
            2 => Val(CKG_A::EN_RF_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKG_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `EN_RF_LOW`"]
    #[inline(always)]
    pub fn is_en_rf_low(&self) -> bool {
        *self == CKG_A::EN_RF_LOW
    }
    #[doc = "Checks if the value of the field is `EN_RF_HIGH`"]
    #[inline(always)]
    pub fn is_en_rf_high(&self) -> bool {
        *self == CKG_A::EN_RF_HIGH
    }
}
#[doc = "Write proxy for field `CKG`"]
pub struct CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKG_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKG_A::CONTINUOUS)
    }
    #[doc = "Receive Clock enabled only if RF Low"]
    #[inline(always)]
    pub fn en_rf_low(self) -> &'a mut W {
        self.variant(CKG_A::EN_RF_LOW)
    }
    #[doc = "Receive Clock enabled only if RF High"]
    #[inline(always)]
    pub fn en_rf_high(self) -> &'a mut W {
        self.variant(CKG_A::EN_RF_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    CONTINUOUS,
    #[doc = "Transmit start"]
    TRANSMIT,
    #[doc = "Detection of a low level on RF signal"]
    RF_LOW,
    #[doc = "Detection of a high level on RF signal"]
    RF_HIGH,
    #[doc = "Detection of a falling edge on RF signal"]
    RF_FALLING,
    #[doc = "Detection of a rising edge on RF signal"]
    RF_RISING,
    #[doc = "Detection of any level change on RF signal"]
    RF_LEVEL,
    #[doc = "Detection of any edge on RF signal"]
    RF_EDGE,
    #[doc = "Compare 0"]
    CMP_0,
}
impl crate::ToBits<u8> for START_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            START_A::CONTINUOUS => 0,
            START_A::TRANSMIT => 1,
            START_A::RF_LOW => 2,
            START_A::RF_HIGH => 3,
            START_A::RF_FALLING => 4,
            START_A::RF_RISING => 5,
            START_A::RF_LEVEL => 6,
            START_A::RF_EDGE => 7,
            START_A::CMP_0 => 8,
        }
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<u8, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, START_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(START_A::CONTINUOUS),
            1 => Val(START_A::TRANSMIT),
            2 => Val(START_A::RF_LOW),
            3 => Val(START_A::RF_HIGH),
            4 => Val(START_A::RF_FALLING),
            5 => Val(START_A::RF_RISING),
            6 => Val(START_A::RF_LEVEL),
            7 => Val(START_A::RF_EDGE),
            8 => Val(START_A::CMP_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == START_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == START_A::TRANSMIT
    }
    #[doc = "Checks if the value of the field is `RF_LOW`"]
    #[inline(always)]
    pub fn is_rf_low(&self) -> bool {
        *self == START_A::RF_LOW
    }
    #[doc = "Checks if the value of the field is `RF_HIGH`"]
    #[inline(always)]
    pub fn is_rf_high(&self) -> bool {
        *self == START_A::RF_HIGH
    }
    #[doc = "Checks if the value of the field is `RF_FALLING`"]
    #[inline(always)]
    pub fn is_rf_falling(&self) -> bool {
        *self == START_A::RF_FALLING
    }
    #[doc = "Checks if the value of the field is `RF_RISING`"]
    #[inline(always)]
    pub fn is_rf_rising(&self) -> bool {
        *self == START_A::RF_RISING
    }
    #[doc = "Checks if the value of the field is `RF_LEVEL`"]
    #[inline(always)]
    pub fn is_rf_level(&self) -> bool {
        *self == START_A::RF_LEVEL
    }
    #[doc = "Checks if the value of the field is `RF_EDGE`"]
    #[inline(always)]
    pub fn is_rf_edge(&self) -> bool {
        *self == START_A::RF_EDGE
    }
    #[doc = "Checks if the value of the field is `CMP_0`"]
    #[inline(always)]
    pub fn is_cmp_0(&self) -> bool {
        *self == START_A::CMP_0
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Continuous, as soon as the receiver is enabled, and immediately after the end of transfer of the previous data."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(START_A::CONTINUOUS)
    }
    #[doc = "Transmit start"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut W {
        self.variant(START_A::TRANSMIT)
    }
    #[doc = "Detection of a low level on RF signal"]
    #[inline(always)]
    pub fn rf_low(self) -> &'a mut W {
        self.variant(START_A::RF_LOW)
    }
    #[doc = "Detection of a high level on RF signal"]
    #[inline(always)]
    pub fn rf_high(self) -> &'a mut W {
        self.variant(START_A::RF_HIGH)
    }
    #[doc = "Detection of a falling edge on RF signal"]
    #[inline(always)]
    pub fn rf_falling(self) -> &'a mut W {
        self.variant(START_A::RF_FALLING)
    }
    #[doc = "Detection of a rising edge on RF signal"]
    #[inline(always)]
    pub fn rf_rising(self) -> &'a mut W {
        self.variant(START_A::RF_RISING)
    }
    #[doc = "Detection of any level change on RF signal"]
    #[inline(always)]
    pub fn rf_level(self) -> &'a mut W {
        self.variant(START_A::RF_LEVEL)
    }
    #[doc = "Detection of any edge on RF signal"]
    #[inline(always)]
    pub fn rf_edge(self) -> &'a mut W {
        self.variant(START_A::RF_EDGE)
    }
    #[doc = "Compare 0"]
    #[inline(always)]
    pub fn cmp_0(self) -> &'a mut W {
        self.variant(START_A::CMP_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Reader of field `STTDLY`"]
pub type STTDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STTDLY`"]
pub struct STTDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> STTDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PERIOD`"]
pub type PERIOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERIOD`"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Clock Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W {
        CKS_W { w: self }
    }
    #[doc = "Bits 2:4 - Receive Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&mut self) -> CKO_W {
        CKO_W { w: self }
    }
    #[doc = "Bit 5 - Receive Clock Inversion"]
    #[inline(always)]
    pub fn cki(&mut self) -> CKI_W {
        CKI_W { w: self }
    }
    #[doc = "Bits 6:7 - Receive Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&mut self) -> CKG_W {
        CKG_W { w: self }
    }
    #[doc = "Bits 8:11 - Receive Start Selection"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 12 - Receive Stop Selection"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bits 16:23 - Receive Start Delay"]
    #[inline(always)]
    pub fn sttdly(&mut self) -> STTDLY_W {
        STTDLY_W { w: self }
    }
    #[doc = "Bits 24:31 - Receive Period Divider Selection"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
}
