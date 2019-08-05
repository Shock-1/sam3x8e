#[doc = "Reader of register TCMR"]
pub type R = crate::R<u32, super::TCMR>;
#[doc = "Writer for register TCMR"]
pub type W = crate::W<u32, super::TCMR>;
#[doc = "Register TCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCMR {
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
    #[doc = "RK Clock signal"]
    RK,
    #[doc = "TK pin"]
    TK,
}
impl crate::ToBits<u8> for CKS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKS_A::MCK => 0,
            CKS_A::RK => 1,
            CKS_A::TK => 2,
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
            1 => Val(CKS_A::RK),
            2 => Val(CKS_A::TK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CKS_A::MCK
    }
    #[doc = "Checks if the value of the field is `RK`"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == CKS_A::RK
    }
    #[doc = "Checks if the value of the field is `TK`"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == CKS_A::TK
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
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKS_A::RK)
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKS_A::TK)
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
    #[doc = "None, TK pin is an input"]
    NONE,
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    CONTINUOUS,
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
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
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKO_A::NONE)
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKO_A::CONTINUOUS)
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
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
    #[doc = "Transmit Clock enabled only if TF Low"]
    EN_TF_LOW,
    #[doc = "Transmit Clock enabled only if TF High"]
    EN_TF_HIGH,
}
impl crate::ToBits<u8> for CKG_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKG_A::CONTINUOUS => 0,
            CKG_A::EN_TF_LOW => 1,
            CKG_A::EN_TF_HIGH => 2,
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
            1 => Val(CKG_A::EN_TF_LOW),
            2 => Val(CKG_A::EN_TF_HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKG_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `EN_TF_LOW`"]
    #[inline(always)]
    pub fn is_en_tf_low(&self) -> bool {
        *self == CKG_A::EN_TF_LOW
    }
    #[doc = "Checks if the value of the field is `EN_TF_HIGH`"]
    #[inline(always)]
    pub fn is_en_tf_high(&self) -> bool {
        *self == CKG_A::EN_TF_HIGH
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
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn en_tf_low(self) -> &'a mut W {
        self.variant(CKG_A::EN_TF_LOW)
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn en_tf_high(self) -> &'a mut W {
        self.variant(CKG_A::EN_TF_HIGH)
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
    #[doc = "Continuous, as soon as a word is written in the SSC_THR Register (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    CONTINUOUS,
    #[doc = "Receive start"]
    RECEIVE,
    #[doc = "Detection of a low level on TF signal"]
    TF_LOW,
    #[doc = "Detection of a high level on TF signal"]
    TF_HIGH,
    #[doc = "Detection of a falling edge on TF signal"]
    TF_FALLING,
    #[doc = "Detection of a rising edge on TF signal"]
    TF_RISING,
    #[doc = "Detection of any level change on TF signal"]
    TF_LEVEL,
    #[doc = "Detection of any edge on TF signal"]
    TF_EDGE,
}
impl crate::ToBits<u8> for START_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            START_A::CONTINUOUS => 0,
            START_A::RECEIVE => 1,
            START_A::TF_LOW => 2,
            START_A::TF_HIGH => 3,
            START_A::TF_FALLING => 4,
            START_A::TF_RISING => 5,
            START_A::TF_LEVEL => 6,
            START_A::TF_EDGE => 7,
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
            1 => Val(START_A::RECEIVE),
            2 => Val(START_A::TF_LOW),
            3 => Val(START_A::TF_HIGH),
            4 => Val(START_A::TF_FALLING),
            5 => Val(START_A::TF_RISING),
            6 => Val(START_A::TF_LEVEL),
            7 => Val(START_A::TF_EDGE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == START_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == START_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `TF_LOW`"]
    #[inline(always)]
    pub fn is_tf_low(&self) -> bool {
        *self == START_A::TF_LOW
    }
    #[doc = "Checks if the value of the field is `TF_HIGH`"]
    #[inline(always)]
    pub fn is_tf_high(&self) -> bool {
        *self == START_A::TF_HIGH
    }
    #[doc = "Checks if the value of the field is `TF_FALLING`"]
    #[inline(always)]
    pub fn is_tf_falling(&self) -> bool {
        *self == START_A::TF_FALLING
    }
    #[doc = "Checks if the value of the field is `TF_RISING`"]
    #[inline(always)]
    pub fn is_tf_rising(&self) -> bool {
        *self == START_A::TF_RISING
    }
    #[doc = "Checks if the value of the field is `TF_LEVEL`"]
    #[inline(always)]
    pub fn is_tf_level(&self) -> bool {
        *self == START_A::TF_LEVEL
    }
    #[doc = "Checks if the value of the field is `TF_EDGE`"]
    #[inline(always)]
    pub fn is_tf_edge(&self) -> bool {
        *self == START_A::TF_EDGE
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
    #[doc = "Continuous, as soon as a word is written in the SSC_THR Register (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(START_A::CONTINUOUS)
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut W {
        self.variant(START_A::RECEIVE)
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn tf_low(self) -> &'a mut W {
        self.variant(START_A::TF_LOW)
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn tf_high(self) -> &'a mut W {
        self.variant(START_A::TF_HIGH)
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn tf_falling(self) -> &'a mut W {
        self.variant(START_A::TF_FALLING)
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn tf_rising(self) -> &'a mut W {
        self.variant(START_A::TF_RISING)
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn tf_level(self) -> &'a mut W {
        self.variant(START_A::TF_LEVEL)
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn tf_edge(self) -> &'a mut W {
        self.variant(START_A::TF_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
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
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W {
        CKS_W { w: self }
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&mut self) -> CKO_W {
        CKO_W { w: self }
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&mut self) -> CKI_W {
        CKI_W { w: self }
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&mut self) -> CKG_W {
        CKG_W { w: self }
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&mut self) -> STTDLY_W {
        STTDLY_W { w: self }
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
}
