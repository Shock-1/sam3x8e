#![doc = "Peripheral access API for ATSAM3X8E microcontrollers (generated using svd2rust v0.16.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn PMC();
    fn EFC0();
    fn EFC1();
    fn UART();
    fn PIOA();
    fn PIOB();
    fn PIOC();
    fn PIOD();
    fn USART0();
    fn USART1();
    fn USART2();
    fn USART3();
    fn HSMCI();
    fn TWI0();
    fn TWI1();
    fn SPI0();
    fn SSC();
    fn TC0();
    fn TC1();
    fn TC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn TC6();
    fn TC7();
    fn TC8();
    fn PWM();
    fn ADC();
    fn DACC();
    fn DMAC();
    fn UOTGHS();
    fn TRNG();
    fn EMAC();
    fn CAN0();
    fn CAN1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 45] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PMC },
    Vector { _handler: EFC0 },
    Vector { _handler: EFC1 },
    Vector { _handler: UART },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PIOA },
    Vector { _handler: PIOB },
    Vector { _handler: PIOC },
    Vector { _handler: PIOD },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: HSMCI },
    Vector { _handler: TWI0 },
    Vector { _handler: TWI1 },
    Vector { _handler: SPI0 },
    Vector { _reserved: 0 },
    Vector { _handler: SSC },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: TC5 },
    Vector { _handler: TC6 },
    Vector { _handler: TC7 },
    Vector { _handler: TC8 },
    Vector { _handler: PWM },
    Vector { _handler: ADC },
    Vector { _handler: DACC },
    Vector { _handler: DMAC },
    Vector { _handler: UOTGHS },
    Vector { _handler: TRNG },
    Vector { _handler: EMAC },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "5 - PMC"]
    PMC,
    #[doc = "6 - EFC0"]
    EFC0,
    #[doc = "7 - EFC1"]
    EFC1,
    #[doc = "8 - UART"]
    UART,
    #[doc = "11 - PIOA"]
    PIOA,
    #[doc = "12 - PIOB"]
    PIOB,
    #[doc = "13 - PIOC"]
    PIOC,
    #[doc = "14 - PIOD"]
    PIOD,
    #[doc = "17 - USART0"]
    USART0,
    #[doc = "18 - USART1"]
    USART1,
    #[doc = "19 - USART2"]
    USART2,
    #[doc = "20 - USART3"]
    USART3,
    #[doc = "21 - HSMCI"]
    HSMCI,
    #[doc = "22 - TWI0"]
    TWI0,
    #[doc = "23 - TWI1"]
    TWI1,
    #[doc = "24 - SPI0"]
    SPI0,
    #[doc = "26 - SSC"]
    SSC,
    #[doc = "27 - TC0"]
    TC0,
    #[doc = "28 - TC1"]
    TC1,
    #[doc = "29 - TC2"]
    TC2,
    #[doc = "30 - TC3"]
    TC3,
    #[doc = "31 - TC4"]
    TC4,
    #[doc = "32 - TC5"]
    TC5,
    #[doc = "33 - TC6"]
    TC6,
    #[doc = "34 - TC7"]
    TC7,
    #[doc = "35 - TC8"]
    TC8,
    #[doc = "36 - PWM"]
    PWM,
    #[doc = "37 - ADC"]
    ADC,
    #[doc = "38 - DACC"]
    DACC,
    #[doc = "39 - DMAC"]
    DMAC,
    #[doc = "40 - UOTGHS"]
    UOTGHS,
    #[doc = "41 - TRNG"]
    TRNG,
    #[doc = "42 - EMAC"]
    EMAC,
    #[doc = "43 - CAN0"]
    CAN0,
    #[doc = "44 - CAN1"]
    CAN1,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PMC => 5,
            Interrupt::EFC0 => 6,
            Interrupt::EFC1 => 7,
            Interrupt::UART => 8,
            Interrupt::PIOA => 11,
            Interrupt::PIOB => 12,
            Interrupt::PIOC => 13,
            Interrupt::PIOD => 14,
            Interrupt::USART0 => 17,
            Interrupt::USART1 => 18,
            Interrupt::USART2 => 19,
            Interrupt::USART3 => 20,
            Interrupt::HSMCI => 21,
            Interrupt::TWI0 => 22,
            Interrupt::TWI1 => 23,
            Interrupt::SPI0 => 24,
            Interrupt::SSC => 26,
            Interrupt::TC0 => 27,
            Interrupt::TC1 => 28,
            Interrupt::TC2 => 29,
            Interrupt::TC3 => 30,
            Interrupt::TC4 => 31,
            Interrupt::TC5 => 32,
            Interrupt::TC6 => 33,
            Interrupt::TC7 => 34,
            Interrupt::TC8 => 35,
            Interrupt::PWM => 36,
            Interrupt::ADC => 37,
            Interrupt::DACC => 38,
            Interrupt::DMAC => 39,
            Interrupt::UOTGHS => 40,
            Interrupt::TRNG => 41,
            Interrupt::EMAC => 42,
            Interrupt::CAN0 => 43,
            Interrupt::CAN1 => 44,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "High Speed MultiMedia Card Interface"]
pub struct HSMCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSMCI {}
impl HSMCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsmci::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for HSMCI {
    type Target = hsmci::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSMCI::ptr() }
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub mod hsmci;
#[doc = "Synchronous Serial Controller"]
pub struct SSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSC {}
impl SSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssc::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SSC {
    type Target = ssc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSC::ptr() }
    }
}
#[doc = "Synchronous Serial Controller"]
pub mod ssc;
#[doc = "Serial Peripheral Interface 0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 0"]
pub mod spi0;
#[doc = "Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer Counter 0"]
pub mod tc0;
#[doc = "Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc1::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Timer Counter 1"]
pub mod tc1;
#[doc = "Timer Counter 2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc2::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Timer Counter 2"]
pub mod tc2;
#[doc = "Two-wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        0x4008_c000 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "Two-wire Interface 0"]
pub mod twi0;
#[doc = "Two-wire Interface 1"]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi1::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "Two-wire Interface 1"]
pub mod twi1;
#[doc = "Pulse Width Modulation Controller"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller"]
pub mod pwm;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4009_c000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub mod usart1;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart2::RegisterBlock {
        0x400a_0000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub mod usart2;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart3::RegisterBlock {
        0x400a_4000 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 3"]
pub mod usart3;
#[doc = "USB On-The-Go Interface"]
pub struct UOTGHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UOTGHS {}
impl UOTGHS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uotghs::RegisterBlock {
        0x400a_c000 as *const _
    }
}
impl Deref for UOTGHS {
    type Target = uotghs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UOTGHS::ptr() }
    }
}
#[doc = "USB On-The-Go Interface"]
pub mod uotghs;
#[doc = "Ethernet MAC 10/100"]
pub struct EMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC {}
impl EMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emac::RegisterBlock {
        0x400b_0000 as *const _
    }
}
impl Deref for EMAC {
    type Target = emac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EMAC::ptr() }
    }
}
#[doc = "Ethernet MAC 10/100"]
pub mod emac;
#[doc = "Controller Area Network 0"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x400b_4000 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Controller Area Network 0"]
pub mod can0;
#[doc = "Controller Area Network 1"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x400b_8000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Controller Area Network 1"]
pub mod can1;
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x400b_c000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x400c_0000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc;
#[doc = "DMA Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        0x400c_4000 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "DMA Controller"]
pub mod dmac;
#[doc = "Digital-to-Analog Converter Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dacc::RegisterBlock {
        0x400c_8000 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter Controller"]
pub mod dacc;
#[doc = "Static Memory Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smc::RegisterBlock {
        0x400e_0000 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "Static Memory Controller"]
pub mod smc;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const matrix::RegisterBlock {
        0x400e_0400 as *const _
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MATRIX::ptr() }
    }
}
#[doc = "AHB Bus Matrix"]
pub mod matrix;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x400e_0600 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x400e_0800 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub mod uart;
#[doc = "Chip Identifier"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const chipid::RegisterBlock {
        0x400e_0940 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub mod chipid;
#[doc = "Embedded Flash Controller 0"]
pub struct EFC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC0 {}
impl EFC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efc0::RegisterBlock {
        0x400e_0a00 as *const _
    }
}
impl Deref for EFC0 {
    type Target = efc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFC0::ptr() }
    }
}
#[doc = "Embedded Flash Controller 0"]
pub mod efc0;
#[doc = "Embedded Flash Controller 1"]
pub struct EFC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC1 {}
impl EFC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efc1::RegisterBlock {
        0x400e_0c00 as *const _
    }
}
impl Deref for EFC1 {
    type Target = efc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFC1::ptr() }
    }
}
#[doc = "Embedded Flash Controller 1"]
pub mod efc1;
#[doc = "Parallel Input/Output Controller A"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        0x400e_0e00 as *const _
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOA::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller A"]
pub mod pioa;
#[doc = "Parallel Input/Output Controller B"]
pub struct PIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOB {}
impl PIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piob::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for PIOB {
    type Target = piob::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOB::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller B"]
pub mod piob;
#[doc = "Parallel Input/Output Controller C"]
pub struct PIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOC {}
impl PIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioc::RegisterBlock {
        0x400e_1200 as *const _
    }
}
impl Deref for PIOC {
    type Target = pioc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOC::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller C"]
pub mod pioc;
#[doc = "Parallel Input/Output Controller D"]
pub struct PIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOD {}
impl PIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piod::RegisterBlock {
        0x400e_1400 as *const _
    }
}
impl Deref for PIOD {
    type Target = piod::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOD::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller D"]
pub mod piod;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x400e_1a00 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x400e_1a10 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Real-time Timer"]
pub struct RTT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTT {}
impl RTT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtt::RegisterBlock {
        0x400e_1a30 as *const _
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTT::ptr() }
    }
}
#[doc = "Real-time Timer"]
pub mod rtt;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x400e_1a50 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real-time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x400e_1a60 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time Clock"]
pub mod rtc;
#[doc = "General Purpose Backup Registers"]
pub struct GPBR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPBR {}
impl GPBR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpbr::RegisterBlock {
        0x400e_1a90 as *const _
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPBR::ptr() }
    }
}
#[doc = "General Purpose Backup Registers"]
pub mod gpbr;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "HSMCI"]
    pub HSMCI: HSMCI,
    #[doc = "SSC"]
    pub SSC: SSC,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UOTGHS"]
    pub UOTGHS: UOTGHS,
    #[doc = "EMAC"]
    pub EMAC: EMAC,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "MATRIX"]
    pub MATRIX: MATRIX,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "EFC0"]
    pub EFC0: EFC0,
    #[doc = "EFC1"]
    pub EFC1: EFC1,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "PIOC"]
    pub PIOC: PIOC,
    #[doc = "PIOD"]
    pub PIOD: PIOD,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "RTT"]
    pub RTT: RTT,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPBR"]
    pub GPBR: GPBR,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            HSMCI: HSMCI {
                _marker: PhantomData,
            },
            SSC: SSC {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UOTGHS: UOTGHS {
                _marker: PhantomData,
            },
            EMAC: EMAC {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            MATRIX: MATRIX {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            EFC0: EFC0 {
                _marker: PhantomData,
            },
            EFC1: EFC1 {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            PIOC: PIOC {
                _marker: PhantomData,
            },
            PIOD: PIOD {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            RTT: RTT {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPBR: GPBR {
                _marker: PhantomData,
            },
        }
    }
}
