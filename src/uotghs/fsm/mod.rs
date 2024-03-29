#[doc = "Reader of register FSM"]
pub type R = crate::R<u32, super::FSM>;
#[doc = "Possible values of the field `DRDSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDSTATE_A {
    #[doc = "This is the start state for A-devices (when the ID pin is 0)"]
    A_IDLESTATE,
    #[doc = "In this state, the A-device waits for the voltage on VBus to rise above the A-device VBus Valid threshold (4.4 V)."]
    A_WAIT_VRISE,
    #[doc = "In this state, the A-device waits for the B-device to signal a connection."]
    A_WAIT_BCON,
    #[doc = "In this state, the A-device that operates in Host mode is operational."]
    A_HOST,
    #[doc = "The A-device operating as a host is in the suspend mode."]
    A_SUSPEND,
    #[doc = "The A-device operates as a peripheral."]
    A_PERIPHERAL,
    #[doc = "In this state, the A-device waits for the voltage on VBus to drop below the A-device Session Valid threshold (1.4 V)."]
    A_WAIT_VFALL,
    #[doc = "In this state, the A-device waits for recovery of the over-current condition that caused it to enter this state."]
    A_VBUS_ERR,
    #[doc = "In this state, the A-device waits for the data USB line to discharge (100 us)."]
    A_WAIT_DISCHARGE,
    #[doc = "This is the start state for B-device (when the ID pin is 1)."]
    B_IDLE,
    #[doc = "In this state, the B-device acts as the peripheral."]
    B_PERIPHERAL,
    #[doc = "In this state, the B-device is in suspend mode and waits until 3 ms before initiating the HNP protocol if requested."]
    B_WAIT_BEGIN_HNP,
    #[doc = "In this state, the B-device waits for the data USB line to discharge (100 us) before becoming Host."]
    B_WAIT_DISCHARGE,
    #[doc = "In this state, the B-device waits for the A-device to signal a connect before becoming B-Host."]
    B_WAIT_ACON,
    #[doc = "In this state, the B-device acts as the Host."]
    B_HOST,
    #[doc = "In this state, the B-device attempts to start a session using the SRP protocol."]
    B_SRP_INIT,
}
impl crate::ToBits<u8> for DRDSTATE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DRDSTATE_A::A_IDLESTATE => 0,
            DRDSTATE_A::A_WAIT_VRISE => 1,
            DRDSTATE_A::A_WAIT_BCON => 2,
            DRDSTATE_A::A_HOST => 3,
            DRDSTATE_A::A_SUSPEND => 4,
            DRDSTATE_A::A_PERIPHERAL => 5,
            DRDSTATE_A::A_WAIT_VFALL => 6,
            DRDSTATE_A::A_VBUS_ERR => 7,
            DRDSTATE_A::A_WAIT_DISCHARGE => 8,
            DRDSTATE_A::B_IDLE => 9,
            DRDSTATE_A::B_PERIPHERAL => 10,
            DRDSTATE_A::B_WAIT_BEGIN_HNP => 11,
            DRDSTATE_A::B_WAIT_DISCHARGE => 12,
            DRDSTATE_A::B_WAIT_ACON => 13,
            DRDSTATE_A::B_HOST => 14,
            DRDSTATE_A::B_SRP_INIT => 15,
        }
    }
}
#[doc = "Reader of field `DRDSTATE`"]
pub type DRDSTATE_R = crate::R<u8, DRDSTATE_A>;
impl DRDSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDSTATE_A {
        match self.bits {
            0 => DRDSTATE_A::A_IDLESTATE,
            1 => DRDSTATE_A::A_WAIT_VRISE,
            2 => DRDSTATE_A::A_WAIT_BCON,
            3 => DRDSTATE_A::A_HOST,
            4 => DRDSTATE_A::A_SUSPEND,
            5 => DRDSTATE_A::A_PERIPHERAL,
            6 => DRDSTATE_A::A_WAIT_VFALL,
            7 => DRDSTATE_A::A_VBUS_ERR,
            8 => DRDSTATE_A::A_WAIT_DISCHARGE,
            9 => DRDSTATE_A::B_IDLE,
            10 => DRDSTATE_A::B_PERIPHERAL,
            11 => DRDSTATE_A::B_WAIT_BEGIN_HNP,
            12 => DRDSTATE_A::B_WAIT_DISCHARGE,
            13 => DRDSTATE_A::B_WAIT_ACON,
            14 => DRDSTATE_A::B_HOST,
            15 => DRDSTATE_A::B_SRP_INIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A_IDLESTATE`"]
    #[inline(always)]
    pub fn is_a_idlestate(&self) -> bool {
        *self == DRDSTATE_A::A_IDLESTATE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VRISE`"]
    #[inline(always)]
    pub fn is_a_wait_vrise(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_VRISE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_BCON`"]
    #[inline(always)]
    pub fn is_a_wait_bcon(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_BCON
    }
    #[doc = "Checks if the value of the field is `A_HOST`"]
    #[inline(always)]
    pub fn is_a_host(&self) -> bool {
        *self == DRDSTATE_A::A_HOST
    }
    #[doc = "Checks if the value of the field is `A_SUSPEND`"]
    #[inline(always)]
    pub fn is_a_suspend(&self) -> bool {
        *self == DRDSTATE_A::A_SUSPEND
    }
    #[doc = "Checks if the value of the field is `A_PERIPHERAL`"]
    #[inline(always)]
    pub fn is_a_peripheral(&self) -> bool {
        *self == DRDSTATE_A::A_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VFALL`"]
    #[inline(always)]
    pub fn is_a_wait_vfall(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_VFALL
    }
    #[doc = "Checks if the value of the field is `A_VBUS_ERR`"]
    #[inline(always)]
    pub fn is_a_vbus_err(&self) -> bool {
        *self == DRDSTATE_A::A_VBUS_ERR
    }
    #[doc = "Checks if the value of the field is `A_WAIT_DISCHARGE`"]
    #[inline(always)]
    pub fn is_a_wait_discharge(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_IDLE`"]
    #[inline(always)]
    pub fn is_b_idle(&self) -> bool {
        *self == DRDSTATE_A::B_IDLE
    }
    #[doc = "Checks if the value of the field is `B_PERIPHERAL`"]
    #[inline(always)]
    pub fn is_b_peripheral(&self) -> bool {
        *self == DRDSTATE_A::B_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `B_WAIT_BEGIN_HNP`"]
    #[inline(always)]
    pub fn is_b_wait_begin_hnp(&self) -> bool {
        *self == DRDSTATE_A::B_WAIT_BEGIN_HNP
    }
    #[doc = "Checks if the value of the field is `B_WAIT_DISCHARGE`"]
    #[inline(always)]
    pub fn is_b_wait_discharge(&self) -> bool {
        *self == DRDSTATE_A::B_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_WAIT_ACON`"]
    #[inline(always)]
    pub fn is_b_wait_acon(&self) -> bool {
        *self == DRDSTATE_A::B_WAIT_ACON
    }
    #[doc = "Checks if the value of the field is `B_HOST`"]
    #[inline(always)]
    pub fn is_b_host(&self) -> bool {
        *self == DRDSTATE_A::B_HOST
    }
    #[doc = "Checks if the value of the field is `B_SRP_INIT`"]
    #[inline(always)]
    pub fn is_b_srp_init(&self) -> bool {
        *self == DRDSTATE_A::B_SRP_INIT
    }
}
impl R {
    #[doc = "Bits 0:3 - Dual Role Device State"]
    #[inline(always)]
    pub fn drdstate(&self) -> DRDSTATE_R {
        DRDSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
