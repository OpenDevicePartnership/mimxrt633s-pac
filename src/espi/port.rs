#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Port 0-4"]
#[doc(alias = "PORT")]
pub struct Port {
    cfg: Cfg,
    stat: Stat,
    irulestat: Irulestat,
    _reserved_3_addr: [u8; 0x04],
    datain: Datain,
    _reserved_5_ramuse: [u8; 0x04],
}
impl Port {
    #[doc = "0x00 - Port Configuration and Control"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - Port Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - Port set interrupt rules and user status"]
    #[inline(always)]
    pub const fn irulestat(&self) -> &Irulestat {
        &self.irulestat
    }
    #[doc = "0x0c - Port OOB, Mastering and Flash Length (for OOB, Bus Mastering, and Flash)"]
    #[inline(always)]
    pub const fn omflen(&self) -> &Omflen {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Port Address offset to host"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Port Data from Host"]
    #[inline(always)]
    pub const fn datain(&self) -> &Datain {
        &self.datain
    }
    #[doc = "0x14 - Port RAM base and size (for Mailbox and Bus Master)"]
    #[inline(always)]
    pub const fn ramuse(&self) -> &Ramuse {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Port Data to Host (for Endpoint and Index/Data)"]
    #[inline(always)]
    pub const fn dataout(&self) -> &Dataout {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
}
#[doc = "CFG (rw) register accessor: Port Configuration and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Port Configuration and Control"]
pub mod cfg;
#[doc = "STAT (rw) register accessor: Port Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Port Status"]
pub mod stat;
#[doc = "IRULESTAT (rw) register accessor: Port set interrupt rules and user status\n\nYou can [`read`](crate::Reg::read) this register and get [`irulestat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irulestat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irulestat`]
module"]
#[doc(alias = "IRULESTAT")]
pub type Irulestat = crate::Reg<irulestat::IrulestatSpec>;
#[doc = "Port set interrupt rules and user status"]
pub mod irulestat;
#[doc = "ADDR (rw) register accessor: Port Address offset to host\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Port Address offset to host"]
pub mod addr;
#[doc = "OMFLEN (rw) register accessor: Port OOB, Mastering and Flash Length (for OOB, Bus Mastering, and Flash)\n\nYou can [`read`](crate::Reg::read) this register and get [`omflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omflen`]
module"]
#[doc(alias = "OMFLEN")]
pub type Omflen = crate::Reg<omflen::OmflenSpec>;
#[doc = "Port OOB, Mastering and Flash Length (for OOB, Bus Mastering, and Flash)"]
pub mod omflen;
#[doc = "DATAIN (r) register accessor: Port Data from Host\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datain`]
module"]
#[doc(alias = "DATAIN")]
pub type Datain = crate::Reg<datain::DatainSpec>;
#[doc = "Port Data from Host"]
pub mod datain;
#[doc = "DATAOUT (rw) register accessor: Port Data to Host (for Endpoint and Index/Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataout`]
module"]
#[doc(alias = "DATAOUT")]
pub type Dataout = crate::Reg<dataout::DataoutSpec>;
#[doc = "Port Data to Host (for Endpoint and Index/Data)"]
pub mod dataout;
#[doc = "RAMUSE (rw) register accessor: Port RAM base and size (for Mailbox and Bus Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`ramuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramuse`]
module"]
#[doc(alias = "RAMUSE")]
pub type Ramuse = crate::Reg<ramuse::RamuseSpec>;
#[doc = "Port RAM base and size (for Mailbox and Bus Master)"]
pub mod ramuse;
