#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcr0: Mcr0,
    mcr1: Mcr1,
    mcr2: Mcr2,
    ahbcr: Ahbcr,
    inten: Inten,
    intr: Intr,
    lutkey: Lutkey,
    lutcr: Lutcr,
    ahbrxbuf0cr0: Ahbrxbuf0cr0,
    ahbrxbuf1cr0: Ahbrxbuf1cr0,
    ahbrxbuf2cr0: Ahbrxbuf2cr0,
    ahbrxbuf3cr0: Ahbrxbuf3cr0,
    ahbrxbuf4cr0: Ahbrxbuf4cr0,
    ahbrxbuf5cr0: Ahbrxbuf5cr0,
    ahbrxbuf6cr0: Ahbrxbuf6cr0,
    ahbrxbuf7cr0: Ahbrxbuf7cr0,
    _reserved16: [u8; 0x20],
    flsha1cr0: Flsha1cr0,
    flsha2cr0: Flsha2cr0,
    flshb1cr0: Flshb1cr0,
    flshb2cr0: Flshb2cr0,
    flshcr1: [Flshcr1; 4],
    flshcr2: [Flshcr2; 4],
    _reserved22: [u8; 0x04],
    flshcr4: Flshcr4,
    _reserved23: [u8; 0x08],
    ipcr0: Ipcr0,
    ipcr1: Ipcr1,
    _reserved25: [u8; 0x08],
    ipcmd: Ipcmd,
    dlpr: Dlpr,
    iprxfcr: Iprxfcr,
    iptxfcr: Iptxfcr,
    dllcr: [Dllcr; 2],
    _reserved30: [u8; 0x18],
    sts0: Sts0,
    sts1: Sts1,
    sts2: Sts2,
    ahbspndsts: Ahbspndsts,
    iprxfsts: Iprxfsts,
    iptxfsts: Iptxfsts,
    _reserved36: [u8; 0x08],
    rfdr: [Rfdr; 32],
    tfdr: [Tfdr; 32],
    lut: [Lut; 128],
}
impl RegisterBlock {
    #[doc = "0x00 - Module Control Register 0"]
    #[inline(always)]
    pub const fn mcr0(&self) -> &Mcr0 {
        &self.mcr0
    }
    #[doc = "0x04 - Module Control Register 1"]
    #[inline(always)]
    pub const fn mcr1(&self) -> &Mcr1 {
        &self.mcr1
    }
    #[doc = "0x08 - Module Control Register 2"]
    #[inline(always)]
    pub const fn mcr2(&self) -> &Mcr2 {
        &self.mcr2
    }
    #[doc = "0x0c - AHB Bus Control Register"]
    #[inline(always)]
    pub const fn ahbcr(&self) -> &Ahbcr {
        &self.ahbcr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x14 - Interrupt Register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x18 - LUT Key Register"]
    #[inline(always)]
    pub const fn lutkey(&self) -> &Lutkey {
        &self.lutkey
    }
    #[doc = "0x1c - LUT Control Register"]
    #[inline(always)]
    pub const fn lutcr(&self) -> &Lutcr {
        &self.lutcr
    }
    #[doc = "0x20 - AHB RX Buffer 0 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf0cr0(&self) -> &Ahbrxbuf0cr0 {
        &self.ahbrxbuf0cr0
    }
    #[doc = "0x24 - AHB RX Buffer 1 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf1cr0(&self) -> &Ahbrxbuf1cr0 {
        &self.ahbrxbuf1cr0
    }
    #[doc = "0x28 - AHB RX Buffer 2 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf2cr0(&self) -> &Ahbrxbuf2cr0 {
        &self.ahbrxbuf2cr0
    }
    #[doc = "0x2c - AHB RX Buffer 3 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf3cr0(&self) -> &Ahbrxbuf3cr0 {
        &self.ahbrxbuf3cr0
    }
    #[doc = "0x30 - AHB RX Buffer 4 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf4cr0(&self) -> &Ahbrxbuf4cr0 {
        &self.ahbrxbuf4cr0
    }
    #[doc = "0x34 - AHB RX Buffer 5 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf5cr0(&self) -> &Ahbrxbuf5cr0 {
        &self.ahbrxbuf5cr0
    }
    #[doc = "0x38 - AHB RX Buffer 6 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf6cr0(&self) -> &Ahbrxbuf6cr0 {
        &self.ahbrxbuf6cr0
    }
    #[doc = "0x3c - AHB RX Buffer 7 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf7cr0(&self) -> &Ahbrxbuf7cr0 {
        &self.ahbrxbuf7cr0
    }
    #[doc = "0x60 - Flash Control Register 0"]
    #[inline(always)]
    pub const fn flsha1cr0(&self) -> &Flsha1cr0 {
        &self.flsha1cr0
    }
    #[doc = "0x64 - Flash Control Register 0"]
    #[inline(always)]
    pub const fn flsha2cr0(&self) -> &Flsha2cr0 {
        &self.flsha2cr0
    }
    #[doc = "0x68 - Flash Control Register 0"]
    #[inline(always)]
    pub const fn flshb1cr0(&self) -> &Flshb1cr0 {
        &self.flshb1cr0
    }
    #[doc = "0x6c - Flash Control Register 0"]
    #[inline(always)]
    pub const fn flshb2cr0(&self) -> &Flshb2cr0 {
        &self.flshb2cr0
    }
    #[doc = "0x70..0x80 - Flash Control Register 1"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `FLSHCR1A1` register.</div>"]
    #[inline(always)]
    pub const fn flshcr1(&self, n: usize) -> &Flshcr1 {
        &self.flshcr1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Flash Control Register 1"]
    #[inline(always)]
    pub fn flshcr1_iter(&self) -> impl Iterator<Item = &Flshcr1> {
        self.flshcr1.iter()
    }
    #[doc = "0x70 - Flash Control Register 1"]
    #[inline(always)]
    pub const fn flshcr1a1(&self) -> &Flshcr1 {
        self.flshcr1(0)
    }
    #[doc = "0x74 - Flash Control Register 1"]
    #[inline(always)]
    pub const fn flshcr1a2(&self) -> &Flshcr1 {
        self.flshcr1(1)
    }
    #[doc = "0x78 - Flash Control Register 1"]
    #[inline(always)]
    pub const fn flshcr1b1(&self) -> &Flshcr1 {
        self.flshcr1(2)
    }
    #[doc = "0x7c - Flash Control Register 1"]
    #[inline(always)]
    pub const fn flshcr1b2(&self) -> &Flshcr1 {
        self.flshcr1(3)
    }
    #[doc = "0x80..0x90 - Flash Control Register 2"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `FLSHCR2A1` register.</div>"]
    #[inline(always)]
    pub const fn flshcr2(&self, n: usize) -> &Flshcr2 {
        &self.flshcr2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - Flash Control Register 2"]
    #[inline(always)]
    pub fn flshcr2_iter(&self) -> impl Iterator<Item = &Flshcr2> {
        self.flshcr2.iter()
    }
    #[doc = "0x80 - Flash Control Register 2"]
    #[inline(always)]
    pub const fn flshcr2a1(&self) -> &Flshcr2 {
        self.flshcr2(0)
    }
    #[doc = "0x84 - Flash Control Register 2"]
    #[inline(always)]
    pub const fn flshcr2a2(&self) -> &Flshcr2 {
        self.flshcr2(1)
    }
    #[doc = "0x88 - Flash Control Register 2"]
    #[inline(always)]
    pub const fn flshcr2b1(&self) -> &Flshcr2 {
        self.flshcr2(2)
    }
    #[doc = "0x8c - Flash Control Register 2"]
    #[inline(always)]
    pub const fn flshcr2b2(&self) -> &Flshcr2 {
        self.flshcr2(3)
    }
    #[doc = "0x94 - Flash Control Register 4"]
    #[inline(always)]
    pub const fn flshcr4(&self) -> &Flshcr4 {
        &self.flshcr4
    }
    #[doc = "0xa0 - IP Control Register 0"]
    #[inline(always)]
    pub const fn ipcr0(&self) -> &Ipcr0 {
        &self.ipcr0
    }
    #[doc = "0xa4 - IP Control Register 1"]
    #[inline(always)]
    pub const fn ipcr1(&self) -> &Ipcr1 {
        &self.ipcr1
    }
    #[doc = "0xb0 - IP Command Register"]
    #[inline(always)]
    pub const fn ipcmd(&self) -> &Ipcmd {
        &self.ipcmd
    }
    #[doc = "0xb4 - Data Learn Pattern Register"]
    #[inline(always)]
    pub const fn dlpr(&self) -> &Dlpr {
        &self.dlpr
    }
    #[doc = "0xb8 - IP RX FIFO Control Register"]
    #[inline(always)]
    pub const fn iprxfcr(&self) -> &Iprxfcr {
        &self.iprxfcr
    }
    #[doc = "0xbc - IP TX FIFO Control Register"]
    #[inline(always)]
    pub const fn iptxfcr(&self) -> &Iptxfcr {
        &self.iptxfcr
    }
    #[doc = "0xc0..0xc8 - DLL Control Register 0"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `DLLCRA` register.</div>"]
    #[inline(always)]
    pub const fn dllcr(&self, n: usize) -> &Dllcr {
        &self.dllcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0xc8 - DLL Control Register 0"]
    #[inline(always)]
    pub fn dllcr_iter(&self) -> impl Iterator<Item = &Dllcr> {
        self.dllcr.iter()
    }
    #[doc = "0xc0 - DLL Control Register 0"]
    #[inline(always)]
    pub const fn dllcra(&self) -> &Dllcr {
        self.dllcr(0)
    }
    #[doc = "0xc4 - DLL Control Register 0"]
    #[inline(always)]
    pub const fn dllcrb(&self) -> &Dllcr {
        self.dllcr(1)
    }
    #[doc = "0xe0 - Status Register 0"]
    #[inline(always)]
    pub const fn sts0(&self) -> &Sts0 {
        &self.sts0
    }
    #[doc = "0xe4 - Status Register 1"]
    #[inline(always)]
    pub const fn sts1(&self) -> &Sts1 {
        &self.sts1
    }
    #[doc = "0xe8 - Status Register 2"]
    #[inline(always)]
    pub const fn sts2(&self) -> &Sts2 {
        &self.sts2
    }
    #[doc = "0xec - AHB Suspend Status Register"]
    #[inline(always)]
    pub const fn ahbspndsts(&self) -> &Ahbspndsts {
        &self.ahbspndsts
    }
    #[doc = "0xf0 - IP RX FIFO Status Register"]
    #[inline(always)]
    pub const fn iprxfsts(&self) -> &Iprxfsts {
        &self.iprxfsts
    }
    #[doc = "0xf4 - IP TX FIFO Status Register"]
    #[inline(always)]
    pub const fn iptxfsts(&self) -> &Iptxfsts {
        &self.iptxfsts
    }
    #[doc = "0x100..0x180 - IP RX FIFO Data Register x"]
    #[inline(always)]
    pub const fn rfdr(&self, n: usize) -> &Rfdr {
        &self.rfdr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - IP RX FIFO Data Register x"]
    #[inline(always)]
    pub fn rfdr_iter(&self) -> impl Iterator<Item = &Rfdr> {
        self.rfdr.iter()
    }
    #[doc = "0x180..0x200 - IP TX FIFO Data Register x"]
    #[inline(always)]
    pub const fn tfdr(&self, n: usize) -> &Tfdr {
        &self.tfdr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x200 - IP TX FIFO Data Register x"]
    #[inline(always)]
    pub fn tfdr_iter(&self) -> impl Iterator<Item = &Tfdr> {
        self.tfdr.iter()
    }
    #[doc = "0x200..0x400 - LUT x"]
    #[inline(always)]
    pub const fn lut(&self, n: usize) -> &Lut {
        &self.lut[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x400 - LUT x"]
    #[inline(always)]
    pub fn lut_iter(&self) -> impl Iterator<Item = &Lut> {
        self.lut.iter()
    }
}
#[doc = "MCR0 (rw) register accessor: Module Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr0`]
module"]
#[doc(alias = "MCR0")]
pub type Mcr0 = crate::Reg<mcr0::Mcr0Spec>;
#[doc = "Module Control Register 0"]
pub mod mcr0;
#[doc = "MCR1 (rw) register accessor: Module Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr1`]
module"]
#[doc(alias = "MCR1")]
pub type Mcr1 = crate::Reg<mcr1::Mcr1Spec>;
#[doc = "Module Control Register 1"]
pub mod mcr1;
#[doc = "MCR2 (rw) register accessor: Module Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr2`]
module"]
#[doc(alias = "MCR2")]
pub type Mcr2 = crate::Reg<mcr2::Mcr2Spec>;
#[doc = "Module Control Register 2"]
pub mod mcr2;
#[doc = "AHBCR (rw) register accessor: AHB Bus Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbcr`]
module"]
#[doc(alias = "AHBCR")]
pub type Ahbcr = crate::Reg<ahbcr::AhbcrSpec>;
#[doc = "AHB Bus Control Register"]
pub mod ahbcr;
#[doc = "INTEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "INTR (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt Register"]
pub mod intr;
#[doc = "LUTKEY (rw) register accessor: LUT Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lutkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutkey`]
module"]
#[doc(alias = "LUTKEY")]
pub type Lutkey = crate::Reg<lutkey::LutkeySpec>;
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LUTCR (rw) register accessor: LUT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutcr`]
module"]
#[doc(alias = "LUTCR")]
pub type Lutcr = crate::Reg<lutcr::LutcrSpec>;
#[doc = "LUT Control Register"]
pub mod lutcr;
#[doc = "AHBRXBUF0CR0 (rw) register accessor: AHB RX Buffer 0 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf0cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf0cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf0cr0`]
module"]
#[doc(alias = "AHBRXBUF0CR0")]
pub type Ahbrxbuf0cr0 = crate::Reg<ahbrxbuf0cr0::Ahbrxbuf0cr0Spec>;
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub mod ahbrxbuf0cr0;
#[doc = "AHBRXBUF1CR0 (rw) register accessor: AHB RX Buffer 1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf1cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf1cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf1cr0`]
module"]
#[doc(alias = "AHBRXBUF1CR0")]
pub type Ahbrxbuf1cr0 = crate::Reg<ahbrxbuf1cr0::Ahbrxbuf1cr0Spec>;
#[doc = "AHB RX Buffer 1 Control Register 0"]
pub mod ahbrxbuf1cr0;
#[doc = "AHBRXBUF2CR0 (rw) register accessor: AHB RX Buffer 2 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf2cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf2cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf2cr0`]
module"]
#[doc(alias = "AHBRXBUF2CR0")]
pub type Ahbrxbuf2cr0 = crate::Reg<ahbrxbuf2cr0::Ahbrxbuf2cr0Spec>;
#[doc = "AHB RX Buffer 2 Control Register 0"]
pub mod ahbrxbuf2cr0;
#[doc = "AHBRXBUF3CR0 (rw) register accessor: AHB RX Buffer 3 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf3cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf3cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf3cr0`]
module"]
#[doc(alias = "AHBRXBUF3CR0")]
pub type Ahbrxbuf3cr0 = crate::Reg<ahbrxbuf3cr0::Ahbrxbuf3cr0Spec>;
#[doc = "AHB RX Buffer 3 Control Register 0"]
pub mod ahbrxbuf3cr0;
#[doc = "AHBRXBUF4CR0 (rw) register accessor: AHB RX Buffer 4 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf4cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf4cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf4cr0`]
module"]
#[doc(alias = "AHBRXBUF4CR0")]
pub type Ahbrxbuf4cr0 = crate::Reg<ahbrxbuf4cr0::Ahbrxbuf4cr0Spec>;
#[doc = "AHB RX Buffer 4 Control Register 0"]
pub mod ahbrxbuf4cr0;
#[doc = "AHBRXBUF5CR0 (rw) register accessor: AHB RX Buffer 5 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf5cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf5cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf5cr0`]
module"]
#[doc(alias = "AHBRXBUF5CR0")]
pub type Ahbrxbuf5cr0 = crate::Reg<ahbrxbuf5cr0::Ahbrxbuf5cr0Spec>;
#[doc = "AHB RX Buffer 5 Control Register 0"]
pub mod ahbrxbuf5cr0;
#[doc = "AHBRXBUF6CR0 (rw) register accessor: AHB RX Buffer 6 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf6cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf6cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf6cr0`]
module"]
#[doc(alias = "AHBRXBUF6CR0")]
pub type Ahbrxbuf6cr0 = crate::Reg<ahbrxbuf6cr0::Ahbrxbuf6cr0Spec>;
#[doc = "AHB RX Buffer 6 Control Register 0"]
pub mod ahbrxbuf6cr0;
#[doc = "AHBRXBUF7CR0 (rw) register accessor: AHB RX Buffer 7 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf7cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf7cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrxbuf7cr0`]
module"]
#[doc(alias = "AHBRXBUF7CR0")]
pub type Ahbrxbuf7cr0 = crate::Reg<ahbrxbuf7cr0::Ahbrxbuf7cr0Spec>;
#[doc = "AHB RX Buffer 7 Control Register 0"]
pub mod ahbrxbuf7cr0;
#[doc = "FLSHA1CR0 (rw) register accessor: Flash Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flsha1cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flsha1cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flsha1cr0`]
module"]
#[doc(alias = "FLSHA1CR0")]
pub type Flsha1cr0 = crate::Reg<flsha1cr0::Flsha1cr0Spec>;
#[doc = "Flash Control Register 0"]
pub mod flsha1cr0;
#[doc = "FLSHA2CR0 (rw) register accessor: Flash Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flsha2cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flsha2cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flsha2cr0`]
module"]
#[doc(alias = "FLSHA2CR0")]
pub type Flsha2cr0 = crate::Reg<flsha2cr0::Flsha2cr0Spec>;
#[doc = "Flash Control Register 0"]
pub mod flsha2cr0;
#[doc = "FLSHB1CR0 (rw) register accessor: Flash Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flshb1cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshb1cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flshb1cr0`]
module"]
#[doc(alias = "FLSHB1CR0")]
pub type Flshb1cr0 = crate::Reg<flshb1cr0::Flshb1cr0Spec>;
#[doc = "Flash Control Register 0"]
pub mod flshb1cr0;
#[doc = "FLSHB2CR0 (rw) register accessor: Flash Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flshb2cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshb2cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flshb2cr0`]
module"]
#[doc(alias = "FLSHB2CR0")]
pub type Flshb2cr0 = crate::Reg<flshb2cr0::Flshb2cr0Spec>;
#[doc = "Flash Control Register 0"]
pub mod flshb2cr0;
#[doc = "FLSHCR1 (rw) register accessor: Flash Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flshcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flshcr1`]
module"]
#[doc(alias = "FLSHCR1")]
pub type Flshcr1 = crate::Reg<flshcr1::Flshcr1Spec>;
#[doc = "Flash Control Register 1"]
pub mod flshcr1;
#[doc = "FLSHCR2 (rw) register accessor: Flash Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flshcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flshcr2`]
module"]
#[doc(alias = "FLSHCR2")]
pub type Flshcr2 = crate::Reg<flshcr2::Flshcr2Spec>;
#[doc = "Flash Control Register 2"]
pub mod flshcr2;
#[doc = "FLSHCR4 (rw) register accessor: Flash Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flshcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flshcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flshcr4`]
module"]
#[doc(alias = "FLSHCR4")]
pub type Flshcr4 = crate::Reg<flshcr4::Flshcr4Spec>;
#[doc = "Flash Control Register 4"]
pub mod flshcr4;
#[doc = "IPCR0 (rw) register accessor: IP Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcr0`]
module"]
#[doc(alias = "IPCR0")]
pub type Ipcr0 = crate::Reg<ipcr0::Ipcr0Spec>;
#[doc = "IP Control Register 0"]
pub mod ipcr0;
#[doc = "IPCR1 (rw) register accessor: IP Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcr1`]
module"]
#[doc(alias = "IPCR1")]
pub type Ipcr1 = crate::Reg<ipcr1::Ipcr1Spec>;
#[doc = "IP Control Register 1"]
pub mod ipcr1;
#[doc = "IPCMD (rw) register accessor: IP Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcmd`]
module"]
#[doc(alias = "IPCMD")]
pub type Ipcmd = crate::Reg<ipcmd::IpcmdSpec>;
#[doc = "IP Command Register"]
pub mod ipcmd;
#[doc = "DLPR (rw) register accessor: Data Learn Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlpr`]
module"]
#[doc(alias = "DLPR")]
pub type Dlpr = crate::Reg<dlpr::DlprSpec>;
#[doc = "Data Learn Pattern Register"]
pub mod dlpr;
#[doc = "IPRXFCR (rw) register accessor: IP RX FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprxfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprxfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprxfcr`]
module"]
#[doc(alias = "IPRXFCR")]
pub type Iprxfcr = crate::Reg<iprxfcr::IprxfcrSpec>;
#[doc = "IP RX FIFO Control Register"]
pub mod iprxfcr;
#[doc = "IPTXFCR (rw) register accessor: IP TX FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iptxfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iptxfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iptxfcr`]
module"]
#[doc(alias = "IPTXFCR")]
pub type Iptxfcr = crate::Reg<iptxfcr::IptxfcrSpec>;
#[doc = "IP TX FIFO Control Register"]
pub mod iptxfcr;
#[doc = "DLLCR (rw) register accessor: DLL Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dllcr`]
module"]
#[doc(alias = "DLLCR")]
pub type Dllcr = crate::Reg<dllcr::DllcrSpec>;
#[doc = "DLL Control Register 0"]
pub mod dllcr;
#[doc = "STS0 (r) register accessor: Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts0`]
module"]
#[doc(alias = "STS0")]
pub type Sts0 = crate::Reg<sts0::Sts0Spec>;
#[doc = "Status Register 0"]
pub mod sts0;
#[doc = "STS1 (r) register accessor: Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts1`]
module"]
#[doc(alias = "STS1")]
pub type Sts1 = crate::Reg<sts1::Sts1Spec>;
#[doc = "Status Register 1"]
pub mod sts1;
#[doc = "STS2 (r) register accessor: Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`]
module"]
#[doc(alias = "STS2")]
pub type Sts2 = crate::Reg<sts2::Sts2Spec>;
#[doc = "Status Register 2"]
pub mod sts2;
#[doc = "AHBSPNDSTS (r) register accessor: AHB Suspend Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbspndsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbspndsts`]
module"]
#[doc(alias = "AHBSPNDSTS")]
pub type Ahbspndsts = crate::Reg<ahbspndsts::AhbspndstsSpec>;
#[doc = "AHB Suspend Status Register"]
pub mod ahbspndsts;
#[doc = "IPRXFSTS (r) register accessor: IP RX FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprxfsts`]
module"]
#[doc(alias = "IPRXFSTS")]
pub type Iprxfsts = crate::Reg<iprxfsts::IprxfstsSpec>;
#[doc = "IP RX FIFO Status Register"]
pub mod iprxfsts;
#[doc = "IPTXFSTS (r) register accessor: IP TX FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iptxfsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iptxfsts`]
module"]
#[doc(alias = "IPTXFSTS")]
pub type Iptxfsts = crate::Reg<iptxfsts::IptxfstsSpec>;
#[doc = "IP TX FIFO Status Register"]
pub mod iptxfsts;
#[doc = "RFDR (r) register accessor: IP RX FIFO Data Register x\n\nYou can [`read`](crate::Reg::read) this register and get [`rfdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdr`]
module"]
#[doc(alias = "RFDR")]
pub type Rfdr = crate::Reg<rfdr::RfdrSpec>;
#[doc = "IP RX FIFO Data Register x"]
pub mod rfdr;
#[doc = "TFDR (w) register accessor: IP TX FIFO Data Register x\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfdr`]
module"]
#[doc(alias = "TFDR")]
pub type Tfdr = crate::Reg<tfdr::TfdrSpec>;
#[doc = "IP TX FIFO Data Register x"]
pub mod tfdr;
#[doc = "LUT (rw) register accessor: LUT x\n\nYou can [`read`](crate::Reg::read) this register and get [`lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut`]
module"]
#[doc(alias = "LUT")]
pub type Lut = crate::Reg<lut::LutSpec>;
#[doc = "LUT x"]
pub mod lut;
