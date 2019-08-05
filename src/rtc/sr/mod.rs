#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Possible values of the field `ACKUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKUPD_A {
    #[doc = "Time and calendar registers cannot be updated."]
    FREERUN,
    #[doc = "Time and calendar registers can be updated."]
    UPDATE,
}
impl crate::ToBits<bool> for ACKUPD_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ACKUPD_A::FREERUN => false,
            ACKUPD_A::UPDATE => true,
        }
    }
}
#[doc = "Reader of field `ACKUPD`"]
pub type ACKUPD_R = crate::R<bool, ACKUPD_A>;
impl ACKUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKUPD_A {
        match self.bits {
            false => ACKUPD_A::FREERUN,
            true => ACKUPD_A::UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `FREERUN`"]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == ACKUPD_A::FREERUN
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ACKUPD_A::UPDATE
    }
}
#[doc = "Possible values of the field `ALARM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM_A {
    #[doc = "No alarm matching condition occurred."]
    NO_ALARMEVENT,
    #[doc = "An alarm matching condition has occurred."]
    ALARMEVENT,
}
impl crate::ToBits<bool> for ALARM_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ALARM_A::NO_ALARMEVENT => false,
            ALARM_A::ALARMEVENT => true,
        }
    }
}
#[doc = "Reader of field `ALARM`"]
pub type ALARM_R = crate::R<bool, ALARM_A>;
impl ALARM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM_A {
        match self.bits {
            false => ALARM_A::NO_ALARMEVENT,
            true => ALARM_A::ALARMEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ALARMEVENT`"]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == ALARM_A::NO_ALARMEVENT
    }
    #[doc = "Checks if the value of the field is `ALARMEVENT`"]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == ALARM_A::ALARMEVENT
    }
}
#[doc = "Possible values of the field `SEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_A {
    #[doc = "No second event has occurred since the last clear."]
    NO_SECEVENT,
    #[doc = "At least one second event has occurred since the last clear."]
    SECEVENT,
}
impl crate::ToBits<bool> for SEC_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SEC_A::NO_SECEVENT => false,
            SEC_A::SECEVENT => true,
        }
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<bool, SEC_A>;
impl SEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A {
        match self.bits {
            false => SEC_A::NO_SECEVENT,
            true => SEC_A::SECEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SECEVENT`"]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == SEC_A::NO_SECEVENT
    }
    #[doc = "Checks if the value of the field is `SECEVENT`"]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == SEC_A::SECEVENT
    }
}
#[doc = "Possible values of the field `TIMEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEV_A {
    #[doc = "No time event has occurred since the last clear."]
    NO_TIMEVENT,
    #[doc = "At least one time event has occurred since the last clear."]
    TIMEVENT,
}
impl crate::ToBits<bool> for TIMEV_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TIMEV_A::NO_TIMEVENT => false,
            TIMEV_A::TIMEVENT => true,
        }
    }
}
#[doc = "Reader of field `TIMEV`"]
pub type TIMEV_R = crate::R<bool, TIMEV_A>;
impl TIMEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEV_A {
        match self.bits {
            false => TIMEV_A::NO_TIMEVENT,
            true => TIMEV_A::TIMEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEVENT`"]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == TIMEV_A::NO_TIMEVENT
    }
    #[doc = "Checks if the value of the field is `TIMEVENT`"]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == TIMEV_A::TIMEVENT
    }
}
#[doc = "Possible values of the field `CALEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEV_A {
    #[doc = "No calendar event has occurred since the last clear."]
    NO_CALEVENT,
    #[doc = "At least one calendar event has occurred since the last clear."]
    CALEVENT,
}
impl crate::ToBits<bool> for CALEV_A {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CALEV_A::NO_CALEVENT => false,
            CALEV_A::CALEVENT => true,
        }
    }
}
#[doc = "Reader of field `CALEV`"]
pub type CALEV_R = crate::R<bool, CALEV_A>;
impl CALEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALEV_A {
        match self.bits {
            false => CALEV_A::NO_CALEVENT,
            true => CALEV_A::CALEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CALEVENT`"]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == CALEV_A::NO_CALEVENT
    }
    #[doc = "Checks if the value of the field is `CALEVENT`"]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == CALEV_A::CALEVENT
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R {
        ACKUPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R {
        TIMEV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R {
        CALEV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
