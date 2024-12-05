#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "SPI enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    Disabled = 0,
    #[doc = "1: Enabled. The SPI is enabled for operation."]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - SPI enable."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Enabled. The SPI is enabled for operation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - SPI enable."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Enabled. The SPI is enabled for operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
#[doc = "Master mode select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Master {
    #[doc = "0: Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SlaveMode = 0,
    #[doc = "1: Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MasterMode = 1,
}
impl From<Master> for bool {
    #[inline(always)]
    fn from(variant: Master) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - Master mode select."]
pub type MasterR = crate::BitReader<Master>;
impl MasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Master {
        match self.bits {
            false => Master::SlaveMode,
            true => Master::MasterMode,
        }
    }
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == Master::SlaveMode
    }
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == Master::MasterMode
    }
}
#[doc = "Field `MASTER` writer - Master mode select."]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG, Master>;
impl<'a, REG> MasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Master::SlaveMode)
    }
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Master::MasterMode)
    }
}
#[doc = "LSB First mode enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbf {
    #[doc = "0: Standard. Data is transmitted and received in standard MSB first order."]
    Standard = 0,
    #[doc = "1: Reverse. Data is transmitted and received in reverse order (LSB first)."]
    Reverse = 1,
}
impl From<Lsbf> for bool {
    #[inline(always)]
    fn from(variant: Lsbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBF` reader - LSB First mode enable."]
pub type LsbfR = crate::BitReader<Lsbf>;
impl LsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsbf {
        match self.bits {
            false => Lsbf::Standard,
            true => Lsbf::Reverse,
        }
    }
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Lsbf::Standard
    }
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == Lsbf::Reverse
    }
}
#[doc = "Field `LSBF` writer - LSB First mode enable."]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG, Lsbf>;
impl<'a, REG> LsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::Standard)
    }
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbf::Reverse)
    }
}
#[doc = "Clock Phase select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    Change = 0,
    #[doc = "1: Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    Capture = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase select."]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::Change,
            true => Cpha::Capture,
        }
    }
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == Cpha::Change
    }
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Cpha::Capture
    }
}
#[doc = "Field `CPHA` writer - Clock Phase select."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    #[inline(always)]
    pub fn change(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::Change)
    }
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::Capture)
    }
}
#[doc = "Clock Polarity select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: Low. The rest state of the clock (between transfers) is low."]
    Low = 0,
    #[doc = "1: High. The rest state of the clock (between transfers) is high."]
    High = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity select."]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::Low,
            true => Cpol::High,
        }
    }
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Cpol::Low
    }
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Cpol::High
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity select."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::Low)
    }
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::High)
    }
}
#[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loop {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<Loop> for bool {
    #[inline(always)]
    fn from(variant: Loop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOP` reader - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
pub type LoopR = crate::BitReader<Loop>;
impl LoopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loop {
        match self.bits {
            false => Loop::Disabled,
            true => Loop::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Loop::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Loop::Enabled
    }
}
#[doc = "Field `LOOP` writer - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG, Loop>;
impl<'a, REG> LoopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loop::Enabled)
    }
}
#[doc = "SSEL0 Polarity select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spol0 {
    #[doc = "0: Low. The SSEL0 pin is active low."]
    Low = 0,
    #[doc = "1: High. The SSEL0 pin is active high."]
    High = 1,
}
impl From<Spol0> for bool {
    #[inline(always)]
    fn from(variant: Spol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL0` reader - SSEL0 Polarity select."]
pub type Spol0R = crate::BitReader<Spol0>;
impl Spol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol0 {
        match self.bits {
            false => Spol0::Low,
            true => Spol0::High,
        }
    }
    #[doc = "Low. The SSEL0 pin is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spol0::Low
    }
    #[doc = "High. The SSEL0 pin is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spol0::High
    }
}
#[doc = "Field `SPOL0` writer - SSEL0 Polarity select."]
pub type Spol0W<'a, REG> = crate::BitWriter<'a, REG, Spol0>;
impl<'a, REG> Spol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The SSEL0 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spol0::Low)
    }
    #[doc = "High. The SSEL0 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spol0::High)
    }
}
#[doc = "SSEL1 Polarity select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spol1 {
    #[doc = "0: Low. The SSEL1 pin is active low."]
    Low = 0,
    #[doc = "1: High. The SSEL1 pin is active high."]
    High = 1,
}
impl From<Spol1> for bool {
    #[inline(always)]
    fn from(variant: Spol1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL1` reader - SSEL1 Polarity select."]
pub type Spol1R = crate::BitReader<Spol1>;
impl Spol1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol1 {
        match self.bits {
            false => Spol1::Low,
            true => Spol1::High,
        }
    }
    #[doc = "Low. The SSEL1 pin is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spol1::Low
    }
    #[doc = "High. The SSEL1 pin is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spol1::High
    }
}
#[doc = "Field `SPOL1` writer - SSEL1 Polarity select."]
pub type Spol1W<'a, REG> = crate::BitWriter<'a, REG, Spol1>;
impl<'a, REG> Spol1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The SSEL1 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spol1::Low)
    }
    #[doc = "High. The SSEL1 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spol1::High)
    }
}
#[doc = "SSEL2 Polarity select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spol2 {
    #[doc = "0: Low. The SSEL2 pin is active low."]
    Low = 0,
    #[doc = "1: High. The SSEL2 pin is active high."]
    High = 1,
}
impl From<Spol2> for bool {
    #[inline(always)]
    fn from(variant: Spol2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL2` reader - SSEL2 Polarity select."]
pub type Spol2R = crate::BitReader<Spol2>;
impl Spol2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol2 {
        match self.bits {
            false => Spol2::Low,
            true => Spol2::High,
        }
    }
    #[doc = "Low. The SSEL2 pin is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spol2::Low
    }
    #[doc = "High. The SSEL2 pin is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spol2::High
    }
}
#[doc = "Field `SPOL2` writer - SSEL2 Polarity select."]
pub type Spol2W<'a, REG> = crate::BitWriter<'a, REG, Spol2>;
impl<'a, REG> Spol2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The SSEL2 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spol2::Low)
    }
    #[doc = "High. The SSEL2 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spol2::High)
    }
}
#[doc = "SSEL3 Polarity select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spol3 {
    #[doc = "0: Low. The SSEL3 pin is active low."]
    Low = 0,
    #[doc = "1: High. The SSEL3 pin is active high."]
    High = 1,
}
impl From<Spol3> for bool {
    #[inline(always)]
    fn from(variant: Spol3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL3` reader - SSEL3 Polarity select."]
pub type Spol3R = crate::BitReader<Spol3>;
impl Spol3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spol3 {
        match self.bits {
            false => Spol3::Low,
            true => Spol3::High,
        }
    }
    #[doc = "Low. The SSEL3 pin is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Spol3::Low
    }
    #[doc = "High. The SSEL3 pin is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Spol3::High
    }
}
#[doc = "Field `SPOL3` writer - SSEL3 Polarity select."]
pub type Spol3W<'a, REG> = crate::BitWriter<'a, REG, Spol3>;
impl<'a, REG> Spol3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The SSEL3 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Spol3::Low)
    }
    #[doc = "High. The SSEL3 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Spol3::High)
    }
}
impl R {
    #[doc = "Bit 0 - SPI enable."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Phase select."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SSEL0 Polarity select."]
    #[inline(always)]
    pub fn spol0(&self) -> Spol0R {
        Spol0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SSEL1 Polarity select."]
    #[inline(always)]
    pub fn spol1(&self) -> Spol1R {
        Spol1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SSEL2 Polarity select."]
    #[inline(always)]
    pub fn spol2(&self) -> Spol2R {
        Spol2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SSEL3 Polarity select."]
    #[inline(always)]
    pub fn spol3(&self) -> Spol3R {
        Spol3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("enable", &self.enable())
            .field("master", &self.master())
            .field("lsbf", &self.lsbf())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("loop_", &self.loop_())
            .field("spol0", &self.spol0())
            .field("spol1", &self.spol1())
            .field("spol2", &self.spol2())
            .field("spol3", &self.spol3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<CfgSpec> {
        MasterW::new(self, 2)
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<CfgSpec> {
        LsbfW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock Phase select."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<CfgSpec> {
        CphaW::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<CfgSpec> {
        CpolW::new(self, 5)
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<CfgSpec> {
        LoopW::new(self, 7)
    }
    #[doc = "Bit 8 - SSEL0 Polarity select."]
    #[inline(always)]
    pub fn spol0(&mut self) -> Spol0W<CfgSpec> {
        Spol0W::new(self, 8)
    }
    #[doc = "Bit 9 - SSEL1 Polarity select."]
    #[inline(always)]
    pub fn spol1(&mut self) -> Spol1W<CfgSpec> {
        Spol1W::new(self, 9)
    }
    #[doc = "Bit 10 - SSEL2 Polarity select."]
    #[inline(always)]
    pub fn spol2(&mut self) -> Spol2W<CfgSpec> {
        Spol2W::new(self, 10)
    }
    #[doc = "Bit 11 - SSEL3 Polarity select."]
    #[inline(always)]
    pub fn spol3(&mut self) -> Spol3W<CfgSpec> {
        Spol3W::new(self, 11)
    }
}
#[doc = "SPI Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
