#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    intstat: Intstat,
    srambase: Srambase,
    _reserved3: [u8; 0x14],
    enableset0: Enableset0,
    enableset1: Enableset1,
    enableclr0: Enableclr0,
    enableclr1: Enableclr1,
    active0: Active0,
    active1: Active1,
    busy0: Busy0,
    busy1: Busy1,
    errint0: Errint0,
    errint1: Errint1,
    intenset0: Intenset0,
    intenset1: Intenset1,
    intenclr0: Intenclr0,
    intenclr1: Intenclr1,
    inta0: Inta0,
    inta1: Inta1,
    intb0: Intb0,
    intb1: Intb1,
    setvalid0: Setvalid0,
    setvalid1: Setvalid1,
    settrig0: Settrig0,
    settrig1: Settrig1,
    abort0: Abort0,
    abort1: Abort1,
    _reserved27: [u8; 0x0380],
    channel: (),
}
impl RegisterBlock {
    #[doc = "0x00 - DMA control."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Interrupt status."]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x08 - SRAM address of the channel configuration table."]
    #[inline(always)]
    pub const fn srambase(&self) -> &Srambase {
        &self.srambase
    }
    #[doc = "0x20 - Channel Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn enableset0(&self) -> &Enableset0 {
        &self.enableset0
    }
    #[doc = "0x24 - Channel Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn enableset1(&self) -> &Enableset1 {
        &self.enableset1
    }
    #[doc = "0x28 - Channel Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn enableclr0(&self) -> &Enableclr0 {
        &self.enableclr0
    }
    #[doc = "0x2c - Channel Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn enableclr1(&self) -> &Enableclr1 {
        &self.enableclr1
    }
    #[doc = "0x30 - Channel Active status for all DMA channels."]
    #[inline(always)]
    pub const fn active0(&self) -> &Active0 {
        &self.active0
    }
    #[doc = "0x34 - Channel Active status for all DMA channels."]
    #[inline(always)]
    pub const fn active1(&self) -> &Active1 {
        &self.active1
    }
    #[doc = "0x38 - Channel Busy status for all DMA channels."]
    #[inline(always)]
    pub const fn busy0(&self) -> &Busy0 {
        &self.busy0
    }
    #[doc = "0x3c - Channel Busy status for all DMA channels."]
    #[inline(always)]
    pub const fn busy1(&self) -> &Busy1 {
        &self.busy1
    }
    #[doc = "0x40 - Error Interrupt status for all DMA channels."]
    #[inline(always)]
    pub const fn errint0(&self) -> &Errint0 {
        &self.errint0
    }
    #[doc = "0x44 - Error Interrupt status for all DMA channels."]
    #[inline(always)]
    pub const fn errint1(&self) -> &Errint1 {
        &self.errint1
    }
    #[doc = "0x48 - Interrupt Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn intenset0(&self) -> &Intenset0 {
        &self.intenset0
    }
    #[doc = "0x4c - Interrupt Enable read and Set for all DMA channels."]
    #[inline(always)]
    pub const fn intenset1(&self) -> &Intenset1 {
        &self.intenset1
    }
    #[doc = "0x50 - Interrupt Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn intenclr0(&self) -> &Intenclr0 {
        &self.intenclr0
    }
    #[doc = "0x54 - Interrupt Enable Clear for all DMA channels."]
    #[inline(always)]
    pub const fn intenclr1(&self) -> &Intenclr1 {
        &self.intenclr1
    }
    #[doc = "0x58 - Interrupt A status for all DMA channels."]
    #[inline(always)]
    pub const fn inta0(&self) -> &Inta0 {
        &self.inta0
    }
    #[doc = "0x5c - Interrupt A status for all DMA channels."]
    #[inline(always)]
    pub const fn inta1(&self) -> &Inta1 {
        &self.inta1
    }
    #[doc = "0x60 - Interrupt B status for all DMA channels."]
    #[inline(always)]
    pub const fn intb0(&self) -> &Intb0 {
        &self.intb0
    }
    #[doc = "0x64 - Interrupt B status for all DMA channels."]
    #[inline(always)]
    pub const fn intb1(&self) -> &Intb1 {
        &self.intb1
    }
    #[doc = "0x68 - Set ValidPending control bits for all DMA channels."]
    #[inline(always)]
    pub const fn setvalid0(&self) -> &Setvalid0 {
        &self.setvalid0
    }
    #[doc = "0x6c - Set ValidPending control bits for all DMA channels."]
    #[inline(always)]
    pub const fn setvalid1(&self) -> &Setvalid1 {
        &self.setvalid1
    }
    #[doc = "0x70 - Set Trigger control bits for all DMA channels."]
    #[inline(always)]
    pub const fn settrig0(&self) -> &Settrig0 {
        &self.settrig0
    }
    #[doc = "0x74 - Set Trigger control bits for all DMA channels."]
    #[inline(always)]
    pub const fn settrig1(&self) -> &Settrig1 {
        &self.settrig1
    }
    #[doc = "0x78 - Channel Abort control for all DMA channels."]
    #[inline(always)]
    pub const fn abort0(&self) -> &Abort0 {
        &self.abort0
    }
    #[doc = "0x7c - Channel Abort control for all DMA channels."]
    #[inline(always)]
    pub const fn abort1(&self) -> &Abort1 {
        &self.abort1
    }
    #[doc = "0x400..0x58c - Array of registers: CFG, CTLSTAT, XFERCFG"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        #[allow(clippy::no_effect)]
        [(); 33][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x58c - Array of registers: CFG, CTLSTAT, XFERCFG"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        (0..33).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "CTRL (rw) register accessor: DMA control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DMA control."]
pub mod ctrl;
#[doc = "INTSTAT (r) register accessor: Interrupt status.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt status."]
pub mod intstat;
#[doc = "SRAMBASE (rw) register accessor: SRAM address of the channel configuration table.\n\nYou can [`read`](crate::Reg::read) this register and get [`srambase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srambase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srambase`]
module"]
#[doc(alias = "SRAMBASE")]
pub type Srambase = crate::Reg<srambase::SrambaseSpec>;
#[doc = "SRAM address of the channel configuration table."]
pub mod srambase;
#[doc = "ENABLESET0 (rw) register accessor: Channel Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`enableset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enableset0`]
module"]
#[doc(alias = "ENABLESET0")]
pub type Enableset0 = crate::Reg<enableset0::Enableset0Spec>;
#[doc = "Channel Enable read and Set for all DMA channels."]
pub mod enableset0;
#[doc = "ENABLESET1 (rw) register accessor: Channel Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`enableset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enableset1`]
module"]
#[doc(alias = "ENABLESET1")]
pub type Enableset1 = crate::Reg<enableset1::Enableset1Spec>;
#[doc = "Channel Enable read and Set for all DMA channels."]
pub mod enableset1;
#[doc = "ENABLECLR0 (w) register accessor: Channel Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enableclr0`]
module"]
#[doc(alias = "ENABLECLR0")]
pub type Enableclr0 = crate::Reg<enableclr0::Enableclr0Spec>;
#[doc = "Channel Enable Clear for all DMA channels."]
pub mod enableclr0;
#[doc = "ENABLECLR1 (w) register accessor: Channel Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enableclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enableclr1`]
module"]
#[doc(alias = "ENABLECLR1")]
pub type Enableclr1 = crate::Reg<enableclr1::Enableclr1Spec>;
#[doc = "Channel Enable Clear for all DMA channels."]
pub mod enableclr1;
#[doc = "ACTIVE0 (r) register accessor: Channel Active status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`active0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active0`]
module"]
#[doc(alias = "ACTIVE0")]
pub type Active0 = crate::Reg<active0::Active0Spec>;
#[doc = "Channel Active status for all DMA channels."]
pub mod active0;
#[doc = "ACTIVE1 (r) register accessor: Channel Active status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`active1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active1`]
module"]
#[doc(alias = "ACTIVE1")]
pub type Active1 = crate::Reg<active1::Active1Spec>;
#[doc = "Channel Active status for all DMA channels."]
pub mod active1;
#[doc = "BUSY0 (r) register accessor: Channel Busy status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`busy0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busy0`]
module"]
#[doc(alias = "BUSY0")]
pub type Busy0 = crate::Reg<busy0::Busy0Spec>;
#[doc = "Channel Busy status for all DMA channels."]
pub mod busy0;
#[doc = "BUSY1 (r) register accessor: Channel Busy status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`busy1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busy1`]
module"]
#[doc(alias = "BUSY1")]
pub type Busy1 = crate::Reg<busy1::Busy1Spec>;
#[doc = "Channel Busy status for all DMA channels."]
pub mod busy1;
#[doc = "ERRINT0 (rw) register accessor: Error Interrupt status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`errint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errint0`]
module"]
#[doc(alias = "ERRINT0")]
pub type Errint0 = crate::Reg<errint0::Errint0Spec>;
#[doc = "Error Interrupt status for all DMA channels."]
pub mod errint0;
#[doc = "ERRINT1 (rw) register accessor: Error Interrupt status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`errint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errint1`]
module"]
#[doc(alias = "ERRINT1")]
pub type Errint1 = crate::Reg<errint1::Errint1Spec>;
#[doc = "Error Interrupt status for all DMA channels."]
pub mod errint1;
#[doc = "INTENSET0 (rw) register accessor: Interrupt Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset0`]
module"]
#[doc(alias = "INTENSET0")]
pub type Intenset0 = crate::Reg<intenset0::Intenset0Spec>;
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub mod intenset0;
#[doc = "INTENSET1 (rw) register accessor: Interrupt Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset1`]
module"]
#[doc(alias = "INTENSET1")]
pub type Intenset1 = crate::Reg<intenset1::Intenset1Spec>;
#[doc = "Interrupt Enable read and Set for all DMA channels."]
pub mod intenset1;
#[doc = "INTENCLR0 (w) register accessor: Interrupt Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr0`]
module"]
#[doc(alias = "INTENCLR0")]
pub type Intenclr0 = crate::Reg<intenclr0::Intenclr0Spec>;
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub mod intenclr0;
#[doc = "INTENCLR1 (w) register accessor: Interrupt Enable Clear for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr1`]
module"]
#[doc(alias = "INTENCLR1")]
pub type Intenclr1 = crate::Reg<intenclr1::Intenclr1Spec>;
#[doc = "Interrupt Enable Clear for all DMA channels."]
pub mod intenclr1;
#[doc = "INTA0 (rw) register accessor: Interrupt A status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`inta0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inta0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inta0`]
module"]
#[doc(alias = "INTA0")]
pub type Inta0 = crate::Reg<inta0::Inta0Spec>;
#[doc = "Interrupt A status for all DMA channels."]
pub mod inta0;
#[doc = "INTA1 (rw) register accessor: Interrupt A status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`inta1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inta1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inta1`]
module"]
#[doc(alias = "INTA1")]
pub type Inta1 = crate::Reg<inta1::Inta1Spec>;
#[doc = "Interrupt A status for all DMA channels."]
pub mod inta1;
#[doc = "INTB0 (rw) register accessor: Interrupt B status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intb0`]
module"]
#[doc(alias = "INTB0")]
pub type Intb0 = crate::Reg<intb0::Intb0Spec>;
#[doc = "Interrupt B status for all DMA channels."]
pub mod intb0;
#[doc = "INTB1 (rw) register accessor: Interrupt B status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intb1`]
module"]
#[doc(alias = "INTB1")]
pub type Intb1 = crate::Reg<intb1::Intb1Spec>;
#[doc = "Interrupt B status for all DMA channels."]
pub mod intb1;
#[doc = "SETVALID0 (w) register accessor: Set ValidPending control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setvalid0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setvalid0`]
module"]
#[doc(alias = "SETVALID0")]
pub type Setvalid0 = crate::Reg<setvalid0::Setvalid0Spec>;
#[doc = "Set ValidPending control bits for all DMA channels."]
pub mod setvalid0;
#[doc = "SETVALID1 (w) register accessor: Set ValidPending control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setvalid1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setvalid1`]
module"]
#[doc(alias = "SETVALID1")]
pub type Setvalid1 = crate::Reg<setvalid1::Setvalid1Spec>;
#[doc = "Set ValidPending control bits for all DMA channels."]
pub mod setvalid1;
#[doc = "SETTRIG0 (w) register accessor: Set Trigger control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`settrig0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@settrig0`]
module"]
#[doc(alias = "SETTRIG0")]
pub type Settrig0 = crate::Reg<settrig0::Settrig0Spec>;
#[doc = "Set Trigger control bits for all DMA channels."]
pub mod settrig0;
#[doc = "SETTRIG1 (w) register accessor: Set Trigger control bits for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`settrig1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@settrig1`]
module"]
#[doc(alias = "SETTRIG1")]
pub type Settrig1 = crate::Reg<settrig1::Settrig1Spec>;
#[doc = "Set Trigger control bits for all DMA channels."]
pub mod settrig1;
#[doc = "ABORT0 (w) register accessor: Channel Abort control for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abort0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abort0`]
module"]
#[doc(alias = "ABORT0")]
pub type Abort0 = crate::Reg<abort0::Abort0Spec>;
#[doc = "Channel Abort control for all DMA channels."]
pub mod abort0;
#[doc = "ABORT1 (w) register accessor: Channel Abort control for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abort1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abort1`]
module"]
#[doc(alias = "ABORT1")]
pub type Abort1 = crate::Reg<abort1::Abort1Spec>;
#[doc = "Channel Abort control for all DMA channels."]
pub mod abort1;
#[doc = "Array of registers: CFG, CTLSTAT, XFERCFG"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Array of registers: CFG, CTLSTAT, XFERCFG"]
pub mod channel;
