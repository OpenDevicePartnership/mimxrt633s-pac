#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    outbase: Outbase,
    outformat: Outformat,
    tmpbase: Tmpbase,
    tmpformat: Tmpformat,
    inabase: Inabase,
    inaformat: Inaformat,
    inbbase: Inbbase,
    inbformat: Inbformat,
    _reserved8: [u8; 0xe0],
    control: Control,
    length: Length,
    cppre: Cppre,
    misc: Misc,
    cursory: Cursory,
    _reserved13: [u8; 0x6c],
    cordic_x: CordicX,
    cordic_y: CordicY,
    cordic_z: CordicZ,
    errstat: Errstat,
    intren: Intren,
    eventen: Eventen,
    intrstat: Intrstat,
    _reserved20: [u8; 0x64],
    gpreg: [Gpreg; 16],
    compreg: [Compreg; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Base address register for output region"]
    #[inline(always)]
    pub const fn outbase(&self) -> &Outbase {
        &self.outbase
    }
    #[doc = "0x04 - Output format"]
    #[inline(always)]
    pub const fn outformat(&self) -> &Outformat {
        &self.outformat
    }
    #[doc = "0x08 - Base address register for temp region"]
    #[inline(always)]
    pub const fn tmpbase(&self) -> &Tmpbase {
        &self.tmpbase
    }
    #[doc = "0x0c - Temp format"]
    #[inline(always)]
    pub const fn tmpformat(&self) -> &Tmpformat {
        &self.tmpformat
    }
    #[doc = "0x10 - Base address register for input A region"]
    #[inline(always)]
    pub const fn inabase(&self) -> &Inabase {
        &self.inabase
    }
    #[doc = "0x14 - Input A format"]
    #[inline(always)]
    pub const fn inaformat(&self) -> &Inaformat {
        &self.inaformat
    }
    #[doc = "0x18 - Base address register for input B region"]
    #[inline(always)]
    pub const fn inbbase(&self) -> &Inbbase {
        &self.inbbase
    }
    #[doc = "0x1c - Input B format"]
    #[inline(always)]
    pub const fn inbformat(&self) -> &Inbformat {
        &self.inbformat
    }
    #[doc = "0x100 - PowerQuad Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x104 - Length register"]
    #[inline(always)]
    pub const fn length(&self) -> &Length {
        &self.length
    }
    #[doc = "0x108 - Pre-scale register"]
    #[inline(always)]
    pub const fn cppre(&self) -> &Cppre {
        &self.cppre
    }
    #[doc = "0x10c - Misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &Misc {
        &self.misc
    }
    #[doc = "0x110 - Cursory register"]
    #[inline(always)]
    pub const fn cursory(&self) -> &Cursory {
        &self.cursory
    }
    #[doc = "0x180 - Cordic input X register"]
    #[inline(always)]
    pub const fn cordic_x(&self) -> &CordicX {
        &self.cordic_x
    }
    #[doc = "0x184 - Cordic input Y register"]
    #[inline(always)]
    pub const fn cordic_y(&self) -> &CordicY {
        &self.cordic_y
    }
    #[doc = "0x188 - Cordic input Z register"]
    #[inline(always)]
    pub const fn cordic_z(&self) -> &CordicZ {
        &self.cordic_z
    }
    #[doc = "0x18c - Read/Write register where error statuses are captured (sticky)"]
    #[inline(always)]
    pub const fn errstat(&self) -> &Errstat {
        &self.errstat
    }
    #[doc = "0x190 - INTERRUPT enable register"]
    #[inline(always)]
    pub const fn intren(&self) -> &Intren {
        &self.intren
    }
    #[doc = "0x194 - Event Enable register"]
    #[inline(always)]
    pub const fn eventen(&self) -> &Eventen {
        &self.eventen
    }
    #[doc = "0x198 - INTERRUPT STATUS register"]
    #[inline(always)]
    pub const fn intrstat(&self) -> &Intrstat {
        &self.intrstat
    }
    #[doc = "0x200..0x240 - General purpose register bank N."]
    #[inline(always)]
    pub const fn gpreg(&self, n: usize) -> &Gpreg {
        &self.gpreg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x240 - General purpose register bank N."]
    #[inline(always)]
    pub fn gpreg_iter(&self) -> impl Iterator<Item = &Gpreg> {
        self.gpreg.iter()
    }
    #[doc = "0x240..0x260 - Compute register bank"]
    #[inline(always)]
    pub const fn compreg(&self, n: usize) -> &Compreg {
        &self.compreg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x260 - Compute register bank"]
    #[inline(always)]
    pub fn compreg_iter(&self) -> impl Iterator<Item = &Compreg> {
        self.compreg.iter()
    }
}
#[doc = "OUTBASE (rw) register accessor: Base address register for output region\n\nYou can [`read`](crate::Reg::read) this register and get [`outbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbase`]
module"]
#[doc(alias = "OUTBASE")]
pub type Outbase = crate::Reg<outbase::OutbaseSpec>;
#[doc = "Base address register for output region"]
pub mod outbase;
#[doc = "OUTFORMAT (rw) register accessor: Output format\n\nYou can [`read`](crate::Reg::read) this register and get [`outformat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outformat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outformat`]
module"]
#[doc(alias = "OUTFORMAT")]
pub type Outformat = crate::Reg<outformat::OutformatSpec>;
#[doc = "Output format"]
pub mod outformat;
#[doc = "TMPBASE (rw) register accessor: Base address register for temp region\n\nYou can [`read`](crate::Reg::read) this register and get [`tmpbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmpbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmpbase`]
module"]
#[doc(alias = "TMPBASE")]
pub type Tmpbase = crate::Reg<tmpbase::TmpbaseSpec>;
#[doc = "Base address register for temp region"]
pub mod tmpbase;
#[doc = "TMPFORMAT (rw) register accessor: Temp format\n\nYou can [`read`](crate::Reg::read) this register and get [`tmpformat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmpformat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmpformat`]
module"]
#[doc(alias = "TMPFORMAT")]
pub type Tmpformat = crate::Reg<tmpformat::TmpformatSpec>;
#[doc = "Temp format"]
pub mod tmpformat;
#[doc = "INABASE (rw) register accessor: Base address register for input A region\n\nYou can [`read`](crate::Reg::read) this register and get [`inabase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inabase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inabase`]
module"]
#[doc(alias = "INABASE")]
pub type Inabase = crate::Reg<inabase::InabaseSpec>;
#[doc = "Base address register for input A region"]
pub mod inabase;
#[doc = "INAFORMAT (rw) register accessor: Input A format\n\nYou can [`read`](crate::Reg::read) this register and get [`inaformat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inaformat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inaformat`]
module"]
#[doc(alias = "INAFORMAT")]
pub type Inaformat = crate::Reg<inaformat::InaformatSpec>;
#[doc = "Input A format"]
pub mod inaformat;
#[doc = "INBBASE (rw) register accessor: Base address register for input B region\n\nYou can [`read`](crate::Reg::read) this register and get [`inbbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inbbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inbbase`]
module"]
#[doc(alias = "INBBASE")]
pub type Inbbase = crate::Reg<inbbase::InbbaseSpec>;
#[doc = "Base address register for input B region"]
pub mod inbbase;
#[doc = "INBFORMAT (rw) register accessor: Input B format\n\nYou can [`read`](crate::Reg::read) this register and get [`inbformat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inbformat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inbformat`]
module"]
#[doc(alias = "INBFORMAT")]
pub type Inbformat = crate::Reg<inbformat::InbformatSpec>;
#[doc = "Input B format"]
pub mod inbformat;
#[doc = "CONTROL (rw) register accessor: PowerQuad Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "PowerQuad Control register"]
pub mod control;
#[doc = "LENGTH (rw) register accessor: Length register\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@length`]
module"]
#[doc(alias = "LENGTH")]
pub type Length = crate::Reg<length::LengthSpec>;
#[doc = "Length register"]
pub mod length;
#[doc = "CPPRE (rw) register accessor: Pre-scale register\n\nYou can [`read`](crate::Reg::read) this register and get [`cppre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cppre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cppre`]
module"]
#[doc(alias = "CPPRE")]
pub type Cppre = crate::Reg<cppre::CppreSpec>;
#[doc = "Pre-scale register"]
pub mod cppre;
#[doc = "MISC (rw) register accessor: Misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`]
module"]
#[doc(alias = "MISC")]
pub type Misc = crate::Reg<misc::MiscSpec>;
#[doc = "Misc register"]
pub mod misc;
#[doc = "CURSORY (rw) register accessor: Cursory register\n\nYou can [`read`](crate::Reg::read) this register and get [`cursory::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursory::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cursory`]
module"]
#[doc(alias = "CURSORY")]
pub type Cursory = crate::Reg<cursory::CursorySpec>;
#[doc = "Cursory register"]
pub mod cursory;
#[doc = "CORDIC_X (rw) register accessor: Cordic input X register\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_x::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_x::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_x`]
module"]
#[doc(alias = "CORDIC_X")]
pub type CordicX = crate::Reg<cordic_x::CordicXSpec>;
#[doc = "Cordic input X register"]
pub mod cordic_x;
#[doc = "CORDIC_Y (rw) register accessor: Cordic input Y register\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_y`]
module"]
#[doc(alias = "CORDIC_Y")]
pub type CordicY = crate::Reg<cordic_y::CordicYSpec>;
#[doc = "Cordic input Y register"]
pub mod cordic_y;
#[doc = "CORDIC_Z (rw) register accessor: Cordic input Z register\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_z::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_z::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_z`]
module"]
#[doc(alias = "CORDIC_Z")]
pub type CordicZ = crate::Reg<cordic_z::CordicZSpec>;
#[doc = "Cordic input Z register"]
pub mod cordic_z;
#[doc = "ERRSTAT (rw) register accessor: Read/Write register where error statuses are captured (sticky)\n\nYou can [`read`](crate::Reg::read) this register and get [`errstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errstat`]
module"]
#[doc(alias = "ERRSTAT")]
pub type Errstat = crate::Reg<errstat::ErrstatSpec>;
#[doc = "Read/Write register where error statuses are captured (sticky)"]
pub mod errstat;
#[doc = "INTREN (rw) register accessor: INTERRUPT enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intren`]
module"]
#[doc(alias = "INTREN")]
pub type Intren = crate::Reg<intren::IntrenSpec>;
#[doc = "INTERRUPT enable register"]
pub mod intren;
#[doc = "EVENTEN (rw) register accessor: Event Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`eventen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventen`]
module"]
#[doc(alias = "EVENTEN")]
pub type Eventen = crate::Reg<eventen::EventenSpec>;
#[doc = "Event Enable register"]
pub mod eventen;
#[doc = "INTRSTAT (rw) register accessor: INTERRUPT STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`intrstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrstat`]
module"]
#[doc(alias = "INTRSTAT")]
pub type Intrstat = crate::Reg<intrstat::IntrstatSpec>;
#[doc = "INTERRUPT STATUS register"]
pub mod intrstat;
#[doc = "gpreg (rw) register accessor: General purpose register bank N.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpreg`]
module"]
#[doc(alias = "gpreg")]
pub type Gpreg = crate::Reg<gpreg::GpregSpec>;
#[doc = "General purpose register bank N."]
pub mod gpreg;
#[doc = "compreg (rw) register accessor: Compute register bank\n\nYou can [`read`](crate::Reg::read) this register and get [`compreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compreg`]
module"]
#[doc(alias = "compreg")]
pub type Compreg = crate::Reg<compreg::CompregSpec>;
#[doc = "Compute register bank"]
pub mod compreg;
