#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mctrl: Mctrl,
    mstat: Mstat,
    intenset: Intenset,
    intenclr: Intenclr,
    intstat: Intstat,
    dmactrl: Dmactrl,
    rambase: Rambase,
    mapbase: Mapbase,
    irqpush: Irqpush,
    wirewo: Wirewo,
    wirero: Wirero,
    _reserved11: [u8; 0x04],
    p80stat: P80stat,
    stataddr: Stataddr,
    _reserved13: [u8; 0x08],
    espicap: Espicap,
    espicfg: Espicfg,
    espimisc: Espimisc,
    _reserved16: [u8; 0xb4],
    port: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Master Control for whole peripheral"]
    #[inline(always)]
    pub const fn mctrl(&self) -> &Mctrl {
        &self.mctrl
    }
    #[doc = "0x04 - Master Status of whole peripheral"]
    #[inline(always)]
    pub const fn mstat(&self) -> &Mstat {
        &self.mstat
    }
    #[doc = "0x08 - Interrupt Set (enable)"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - Interrupt Clear (disable)"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - Masked interrupt status (causes)"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x14 - Selects DMA for Ports (if used)"]
    #[inline(always)]
    pub const fn dmactrl(&self) -> &Dmactrl {
        &self.dmactrl
    }
    #[doc = "0x18 - Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable)."]
    #[inline(always)]
    pub const fn rambase(&self) -> &Rambase {
        &self.rambase
    }
    #[doc = "0x1c - Base0 and Base1 mapped offsets for ports"]
    #[inline(always)]
    pub const fn mapbase(&self) -> &Mapbase {
        &self.mapbase
    }
    #[doc = "0x20 - IRQ to drive into Host (with eSPI)"]
    #[inline(always)]
    pub const fn irqpush(&self) -> &Irqpush {
        &self.irqpush
    }
    #[doc = "0x24 - Wire states for Host to see; if LPC, this is the IRQ states."]
    #[inline(always)]
    pub const fn wirewo(&self) -> &Wirewo {
        &self.wirewo
    }
    #[doc = "0x28 - Wire states from Host"]
    #[inline(always)]
    pub const fn wirero(&self) -> &Wirero {
        &self.wirero
    }
    #[doc = "0x30 - Port 80 Status (byte and prev byte)"]
    #[inline(always)]
    pub const fn p80stat(&self) -> &P80stat {
        &self.p80stat
    }
    #[doc = "0x34 - Location of Status block in memory space, if enabled."]
    #[inline(always)]
    pub const fn stataddr(&self) -> &Stataddr {
        &self.stataddr
    }
    #[doc = "0x40 - eSPI Capabilities of MCU to send to Host"]
    #[inline(always)]
    pub const fn espicap(&self) -> &Espicap {
        &self.espicap
    }
    #[doc = "0x44 - eSPI Configuration settings from eSPI"]
    #[inline(always)]
    pub const fn espicfg(&self) -> &Espicfg {
        &self.espicfg
    }
    #[doc = "0x48 - Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert)."]
    #[inline(always)]
    pub const fn espimisc(&self) -> &Espimisc {
        &self.espimisc
    }
    #[doc = "0x100..0x178 - Port 0-4"]
    #[inline(always)]
    pub const fn port(&self, n: usize) -> &Port {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x178 - Port 0-4"]
    #[inline(always)]
    pub fn port_iter(&self) -> impl Iterator<Item = &Port> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        })
    }
}
#[doc = "MCTRL (rw) register accessor: Master Control for whole peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`]
module"]
#[doc(alias = "MCTRL")]
pub type Mctrl = crate::Reg<mctrl::MctrlSpec>;
#[doc = "Master Control for whole peripheral"]
pub mod mctrl;
#[doc = "MSTAT (rw) register accessor: Master Status of whole peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`mstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstat`]
module"]
#[doc(alias = "MSTAT")]
pub type Mstat = crate::Reg<mstat::MstatSpec>;
#[doc = "Master Status of whole peripheral"]
pub mod mstat;
#[doc = "INTENSET (rw) register accessor: Interrupt Set (enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Set (enable)"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt Clear (disable)\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Clear (disable)"]
pub mod intenclr;
#[doc = "INTSTAT (rw) register accessor: Masked interrupt status (causes)\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Masked interrupt status (causes)"]
pub mod intstat;
#[doc = "DMACTRL (rw) register accessor: Selects DMA for Ports (if used)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl`]
module"]
#[doc(alias = "DMACTRL")]
pub type Dmactrl = crate::Reg<dmactrl::DmactrlSpec>;
#[doc = "Selects DMA for Ports (if used)"]
pub mod dmactrl;
#[doc = "RAMBASE (rw) register accessor: Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable).\n\nYou can [`read`](crate::Reg::read) this register and get [`rambase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rambase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rambase`]
module"]
#[doc(alias = "RAMBASE")]
pub type Rambase = crate::Reg<rambase::RambaseSpec>;
#[doc = "Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable)."]
pub mod rambase;
#[doc = "MAPBASE (rw) register accessor: Base0 and Base1 mapped offsets for ports\n\nYou can [`read`](crate::Reg::read) this register and get [`mapbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mapbase`]
module"]
#[doc(alias = "MAPBASE")]
pub type Mapbase = crate::Reg<mapbase::MapbaseSpec>;
#[doc = "Base0 and Base1 mapped offsets for ports"]
pub mod mapbase;
#[doc = "IRQPUSH (rw) register accessor: IRQ to drive into Host (with eSPI)\n\nYou can [`read`](crate::Reg::read) this register and get [`irqpush::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqpush::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqpush`]
module"]
#[doc(alias = "IRQPUSH")]
pub type Irqpush = crate::Reg<irqpush::IrqpushSpec>;
#[doc = "IRQ to drive into Host (with eSPI)"]
pub mod irqpush;
#[doc = "WIREWO (rw) register accessor: Wire states for Host to see; if LPC, this is the IRQ states.\n\nYou can [`read`](crate::Reg::read) this register and get [`wirewo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wirewo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wirewo`]
module"]
#[doc(alias = "WIREWO")]
pub type Wirewo = crate::Reg<wirewo::WirewoSpec>;
#[doc = "Wire states for Host to see; if LPC, this is the IRQ states."]
pub mod wirewo;
#[doc = "WIRERO (r) register accessor: Wire states from Host\n\nYou can [`read`](crate::Reg::read) this register and get [`wirero::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wirero`]
module"]
#[doc(alias = "WIRERO")]
pub type Wirero = crate::Reg<wirero::WireroSpec>;
#[doc = "Wire states from Host"]
pub mod wirero;
#[doc = "P80STAT (rw) register accessor: Port 80 Status (byte and prev byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`p80stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p80stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p80stat`]
module"]
#[doc(alias = "P80STAT")]
pub type P80stat = crate::Reg<p80stat::P80statSpec>;
#[doc = "Port 80 Status (byte and prev byte)"]
pub mod p80stat;
#[doc = "STATADDR (rw) register accessor: Location of Status block in memory space, if enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`stataddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stataddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stataddr`]
module"]
#[doc(alias = "STATADDR")]
pub type Stataddr = crate::Reg<stataddr::StataddrSpec>;
#[doc = "Location of Status block in memory space, if enabled."]
pub mod stataddr;
#[doc = "ESPICAP (rw) register accessor: eSPI Capabilities of MCU to send to Host\n\nYou can [`read`](crate::Reg::read) this register and get [`espicap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espicap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espicap`]
module"]
#[doc(alias = "ESPICAP")]
pub type Espicap = crate::Reg<espicap::EspicapSpec>;
#[doc = "eSPI Capabilities of MCU to send to Host"]
pub mod espicap;
#[doc = "ESPICFG (r) register accessor: eSPI Configuration settings from eSPI\n\nYou can [`read`](crate::Reg::read) this register and get [`espicfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espicfg`]
module"]
#[doc(alias = "ESPICFG")]
pub type Espicfg = crate::Reg<espicfg::EspicfgSpec>;
#[doc = "eSPI Configuration settings from eSPI"]
pub mod espicfg;
#[doc = "ESPIMISC (rw) register accessor: Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert).\n\nYou can [`read`](crate::Reg::read) this register and get [`espimisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espimisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espimisc`]
module"]
#[doc(alias = "ESPIMISC")]
pub type Espimisc = crate::Reg<espimisc::EspimiscSpec>;
#[doc = "Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert)."]
pub mod espimisc;
#[doc = "Port 0-4"]
pub use self::port::Port;
#[doc = r"Cluster"]
#[doc = "Port 0-4"]
pub mod port;
