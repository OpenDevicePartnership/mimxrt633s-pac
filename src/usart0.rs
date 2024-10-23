#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    ctl: Ctl,
    stat: Stat,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved5: [u8; 0x0c],
    brg: Brg,
    intstat: Intstat,
    osr: Osr,
    addr: Addr,
    _reserved9: [u8; 0x0dd0],
    fifocfg: Fifocfg,
    fifostat: Fifostat,
    fifotrig: Fifotrig,
    _reserved12: [u8; 0x04],
    fifointenset: Fifointenset,
    fifointenclr: Fifointenclr,
    fifointstat: Fifointstat,
    _reserved15: [u8; 0x04],
    fifowr: Fifowr,
    _reserved16: [u8; 0x0c],
    fiford: Fiford,
    _reserved17: [u8; 0x0c],
    fifordnopop: Fifordnopop,
    _reserved18: [u8; 0x04],
    fifosize: Fifosize,
    _reserved19: [u8; 0x01b0],
    id: Id,
}
impl RegisterBlock {
    #[doc = "0x00 - USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - USART Control register. USART control settings that are more likely to change during operation."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x08 - USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x0c - Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x20 - Baud Rate Generator register. 16-bit integer baud rate divisor value."]
    #[inline(always)]
    pub const fn brg(&self) -> &Brg {
        &self.brg
    }
    #[doc = "0x24 - Interrupt status register. Reflects interrupts that are currently enabled."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x28 - Oversample selection register for asynchronous communication."]
    #[inline(always)]
    pub const fn osr(&self) -> &Osr {
        &self.osr
    }
    #[doc = "0x2c - Address register for automatic address matching."]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0xe00 - FIFO configuration and enable register."]
    #[inline(always)]
    pub const fn fifocfg(&self) -> &Fifocfg {
        &self.fifocfg
    }
    #[doc = "0xe04 - FIFO status register."]
    #[inline(always)]
    pub const fn fifostat(&self) -> &Fifostat {
        &self.fifostat
    }
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    #[inline(always)]
    pub const fn fifotrig(&self) -> &Fifotrig {
        &self.fifotrig
    }
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    #[inline(always)]
    pub const fn fifointenset(&self) -> &Fifointenset {
        &self.fifointenset
    }
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    #[inline(always)]
    pub const fn fifointenclr(&self) -> &Fifointenclr {
        &self.fifointenclr
    }
    #[doc = "0xe18 - FIFO interrupt status register."]
    #[inline(always)]
    pub const fn fifointstat(&self) -> &Fifointstat {
        &self.fifointstat
    }
    #[doc = "0xe20 - FIFO write data."]
    #[inline(always)]
    pub const fn fifowr(&self) -> &Fifowr {
        &self.fifowr
    }
    #[doc = "0xe30 - FIFO read data."]
    #[inline(always)]
    pub const fn fiford(&self) -> &Fiford {
        &self.fiford
    }
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    #[inline(always)]
    pub const fn fifordnopop(&self) -> &Fifordnopop {
        &self.fifordnopop
    }
    #[doc = "0xe48 - FIFO size register"]
    #[inline(always)]
    pub const fn fifosize(&self) -> &Fifosize {
        &self.fifosize
    }
    #[doc = "0xffc - Peripheral identification register."]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
}
#[doc = "CFG (rw) register accessor: USART Configuration register. Basic USART configuration settings that typically are not changed during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
pub mod cfg;
#[doc = "CTL (rw) register accessor: USART Control register. USART control settings that are more likely to change during operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
pub mod ctl;
#[doc = "STAT (rw) register accessor: USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
pub mod intenclr;
#[doc = "BRG (rw) register accessor: Baud Rate Generator register. 16-bit integer baud rate divisor value.\n\nYou can [`read`](crate::Reg::read) this register and get [`brg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brg`]
module"]
#[doc(alias = "BRG")]
pub type Brg = crate::Reg<brg::BrgSpec>;
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
pub mod brg;
#[doc = "INTSTAT (r) register accessor: Interrupt status register. Reflects interrupts that are currently enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
pub mod intstat;
#[doc = "OSR (rw) register accessor: Oversample selection register for asynchronous communication.\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osr`]
module"]
#[doc(alias = "OSR")]
pub type Osr = crate::Reg<osr::OsrSpec>;
#[doc = "Oversample selection register for asynchronous communication."]
pub mod osr;
#[doc = "ADDR (rw) register accessor: Address register for automatic address matching.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address register for automatic address matching."]
pub mod addr;
#[doc = "FIFOCFG (rw) register accessor: FIFO configuration and enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocfg`]
module"]
#[doc(alias = "FIFOCFG")]
pub type Fifocfg = crate::Reg<fifocfg::FifocfgSpec>;
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFOSTAT (rw) register accessor: FIFO status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifostat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifostat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifostat`]
module"]
#[doc(alias = "FIFOSTAT")]
pub type Fifostat = crate::Reg<fifostat::FifostatSpec>;
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFOTRIG (rw) register accessor: FIFO trigger settings for interrupt and DMA request.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifotrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifotrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifotrig`]
module"]
#[doc(alias = "FIFOTRIG")]
pub type Fifotrig = crate::Reg<fifotrig::FifotrigSpec>;
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFOINTENSET (rw) register accessor: FIFO interrupt enable set (enable) and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifointenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifointenset`]
module"]
#[doc(alias = "FIFOINTENSET")]
pub type Fifointenset = crate::Reg<fifointenset::FifointensetSpec>;
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFOINTENCLR (rw) register accessor: FIFO interrupt enable clear (disable) and read register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifointenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifointenclr`]
module"]
#[doc(alias = "FIFOINTENCLR")]
pub type Fifointenclr = crate::Reg<fifointenclr::FifointenclrSpec>;
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFOINTSTAT (r) register accessor: FIFO interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifointstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifointstat`]
module"]
#[doc(alias = "FIFOINTSTAT")]
pub type Fifointstat = crate::Reg<fifointstat::FifointstatSpec>;
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFOWR (w) register accessor: FIFO write data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifowr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifowr`]
module"]
#[doc(alias = "FIFOWR")]
pub type Fifowr = crate::Reg<fifowr::FifowrSpec>;
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFORD (r) register accessor: FIFO read data.\n\nYou can [`read`](crate::Reg::read) this register and get [`fiford::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiford`]
module"]
#[doc(alias = "FIFORD")]
pub type Fiford = crate::Reg<fiford::FifordSpec>;
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFORDNOPOP (r) register accessor: FIFO data read with no FIFO pop.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifordnopop::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifordnopop`]
module"]
#[doc(alias = "FIFORDNOPOP")]
pub type Fifordnopop = crate::Reg<fifordnopop::FifordnopopSpec>;
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "FIFOSIZE (rw) register accessor: FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifosize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifosize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifosize`]
module"]
#[doc(alias = "FIFOSIZE")]
pub type Fifosize = crate::Reg<fifosize::FifosizeSpec>;
#[doc = "FIFO size register"]
pub mod fifosize;
#[doc = "ID (r) register accessor: Peripheral identification register.\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Peripheral identification register."]
pub mod id;
