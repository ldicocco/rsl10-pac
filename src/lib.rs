#![doc = "Peripheral access API for RSL10 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        match *self {}
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
#[doc = "Chip Identification"]
pub struct AHBREGS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AHBREGS {}
impl AHBREGS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ahbregs::RegisterBlock {
        0x1fff_fffc as *const _
    }
}
impl Deref for AHBREGS {
    type Target = ahbregs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AHBREGS::ptr() }
    }
}
#[doc = "Chip Identification"]
pub mod ahbregs;
#[doc = "System Control"]
pub struct SYSCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTRL {}
impl SYSCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctrl::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for SYSCTRL {
    type Target = sysctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCTRL::ptr() }
    }
}
#[doc = "System Control"]
pub mod sysctrl;
#[doc = "Clock Generation"]
pub struct CLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLK {}
impl CLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clk::RegisterBlock {
        0x4000_0100 as *const _
    }
}
impl Deref for CLK {
    type Target = clk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLK::ptr() }
    }
}
#[doc = "Clock Generation"]
pub mod clk;
#[doc = "Reset"]
pub struct DIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DIG {}
impl DIG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dig::RegisterBlock {
        0x4000_0200 as *const _
    }
}
impl Deref for DIG {
    type Target = dig::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DIG::ptr() }
    }
}
#[doc = "Reset"]
pub mod dig;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG {}
impl WATCHDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog::RegisterBlock {
        0x4000_0300 as *const _
    }
}
impl Deref for WATCHDOG {
    type Target = watchdog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog;
#[doc = "General-Purpose Timers 0, 1, 2 and 3"]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER::ptr() }
    }
}
#[doc = "General-Purpose Timers 0, 1, 2 and 3"]
pub mod timer;
#[doc = "Flash Interface Configuration and Control"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4000_0500 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash Interface Configuration and Control"]
pub mod flash;
#[doc = "DMA Controller Configuration and Control"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x4000_0600 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA Controller Configuration and Control"]
pub mod dma;
#[doc = "DIO Interface and Digital Pad control"]
pub struct DIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DIO {}
impl DIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dio::RegisterBlock {
        0x4000_0700 as *const _
    }
}
impl Deref for DIO {
    type Target = dio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DIO::ptr() }
    }
}
#[doc = "DIO Interface and Digital Pad control"]
pub mod dio;
#[doc = "SPI Interface Configuration and Control"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "SPI Interface Configuration and Control"]
pub mod spi0;
#[doc = "SPI Interface Configuration and Control"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_0900 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "SPI Interface Configuration and Control"]
pub mod spi1;
#[doc = "PCM Interface Configuration and Control"]
pub struct PCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCM {}
impl PCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcm::RegisterBlock {
        0x4000_0a00 as *const _
    }
}
impl Deref for PCM {
    type Target = pcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCM::ptr() }
    }
}
#[doc = "PCM Interface Configuration and Control"]
pub mod pcm;
#[doc = "I2C Interface Configuration and Control"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0x4000_0b00 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "I2C Interface Configuration and Control"]
pub mod i2c;
#[doc = "UART Interface Configuration and Control"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "UART Interface Configuration and Control"]
pub mod uart;
#[doc = "PWM 0 and 1 Configuration and Control"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        0x4000_0d00 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "PWM 0 and 1 Configuration and Control"]
pub mod pwm;
#[doc = "DMIC Input and Output Driver Configuration and Control"]
pub struct AUDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUDIO {}
impl AUDIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const audio::RegisterBlock {
        0x4000_0e00 as *const _
    }
}
impl Deref for AUDIO {
    type Target = audio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUDIO::ptr() }
    }
}
#[doc = "DMIC Input and Output Driver Configuration and Control"]
pub mod audio;
#[doc = "CRC Generator Control"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4000_0f00 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC Generator Control"]
pub mod crc;
#[doc = "Audio Sink Clock Counters"]
pub struct AUDIOSINK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUDIOSINK {}
impl AUDIOSINK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const audiosink::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for AUDIOSINK {
    type Target = audiosink::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUDIOSINK::ptr() }
    }
}
#[doc = "Audio Sink Clock Counters"]
pub mod audiosink;
#[doc = "ASRC Configuration and Control"]
pub struct ASRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ASRC {}
impl ASRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const asrc::RegisterBlock {
        0x4000_1100 as *const _
    }
}
impl Deref for ASRC {
    type Target = asrc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ASRC::ptr() }
    }
}
#[doc = "ASRC Configuration and Control"]
pub mod asrc;
#[doc = "Analog-to-Digital Converter and Battery Monitoring"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4000_1200 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter and Battery Monitoring"]
pub mod adc;
#[doc = "ACS domain (Analog Bridge Access)"]
pub struct ACS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACS {}
impl ACS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acs::RegisterBlock {
        0x4000_1300 as *const _
    }
}
impl Deref for ACS {
    type Target = acs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACS::ptr() }
    }
}
#[doc = "ACS domain (Analog Bridge Access)"]
pub mod acs;
#[doc = "Baseband Controller Interface"]
pub struct BBIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BBIF {}
impl BBIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bbif::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for BBIF {
    type Target = bbif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BBIF::ptr() }
    }
}
#[doc = "Baseband Controller Interface"]
pub mod bbif;
#[doc = "Baseband Controller"]
pub struct BB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BB {}
impl BB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bb::RegisterBlock {
        0x4000_1500 as *const _
    }
}
impl Deref for BB {
    type Target = bb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BB::ptr() }
    }
}
#[doc = "Baseband Controller"]
pub mod bb;
#[doc = "RF Front-End 2.4 GHz"]
pub struct RF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RF {}
impl RF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rf::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for RF {
    type Target = rf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RF::ptr() }
    }
}
#[doc = "RF Front-End 2.4 GHz"]
pub mod rf;
#[doc = "SYSTICK Timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "SYSTICK Timer"]
pub mod sys_tick;
#[doc = "System Control and ID register not in the SCB"]
pub struct SCNSCB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCNSCB {}
impl SCNSCB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scn_scb::RegisterBlock {
        0xe000_e004 as *const _
    }
}
impl Deref for SCNSCB {
    type Target = scn_scb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCNSCB::ptr() }
    }
}
#[doc = "System Control and ID register not in the SCB"]
pub mod scn_scb;
#[doc = "Debug Controller"]
pub struct DEBUG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DEBUG {}
impl DEBUG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const debug::RegisterBlock {
        0xe000_edf0 as *const _
    }
}
impl Deref for DEBUG {
    type Target = debug::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DEBUG::ptr() }
    }
}
#[doc = "Debug Controller"]
pub mod debug;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AHBREGS"]
    pub AHBREGS: AHBREGS,
    #[doc = "SYSCTRL"]
    pub SYSCTRL: SYSCTRL,
    #[doc = "CLK"]
    pub CLK: CLK,
    #[doc = "DIG"]
    pub DIG: DIG,
    #[doc = "WATCHDOG"]
    pub WATCHDOG: WATCHDOG,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "DIO"]
    pub DIO: DIO,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "PCM"]
    pub PCM: PCM,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "AUDIO"]
    pub AUDIO: AUDIO,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "AUDIOSINK"]
    pub AUDIOSINK: AUDIOSINK,
    #[doc = "ASRC"]
    pub ASRC: ASRC,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "ACS"]
    pub ACS: ACS,
    #[doc = "BBIF"]
    pub BBIF: BBIF,
    #[doc = "BB"]
    pub BB: BB,
    #[doc = "RF"]
    pub RF: RF,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "SCNSCB"]
    pub SCNSCB: SCNSCB,
    #[doc = "DEBUG"]
    pub DEBUG: DEBUG,
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
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AHBREGS: AHBREGS {
                _marker: PhantomData,
            },
            SYSCTRL: SYSCTRL {
                _marker: PhantomData,
            },
            CLK: CLK {
                _marker: PhantomData,
            },
            DIG: DIG {
                _marker: PhantomData,
            },
            WATCHDOG: WATCHDOG {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            DIO: DIO {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            PCM: PCM {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            AUDIO: AUDIO {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            AUDIOSINK: AUDIOSINK {
                _marker: PhantomData,
            },
            ASRC: ASRC {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            ACS: ACS {
                _marker: PhantomData,
            },
            BBIF: BBIF {
                _marker: PhantomData,
            },
            BB: BB {
                _marker: PhantomData,
            },
            RF: RF {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            SCNSCB: SCNSCB {
                _marker: PhantomData,
            },
            DEBUG: DEBUG {
                _marker: PhantomData,
            },
        }
    }
}
