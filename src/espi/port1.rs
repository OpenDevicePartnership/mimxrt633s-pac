#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Cluster PORT1, containing P1CFG, P1STAT, P1IRULESTAT, P1ADDR, P1OMFLEN, P1DATAIN, P1DATAOUT, P1RAMUSE"]
#[doc(alias = "PORT1")]
pub struct Port1 {
    cfg: Cfg,
    stat: Stat,
    irulestat: Irulestat,
    _reserved_3_addr: [u8; 0x04],
    datain: Datain,
    _reserved_5_ramuse: [u8; 0x04],
}
impl Port1 {
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
    #[doc = "0x08 - Port set Interrupt rules and user status"]
    #[inline(always)]
    pub const fn irulestat(&self) -> &Irulestat {
        &self.irulestat
    }
    #[doc = "0x0c - Port OOB, Mastering and Flash Length (for OOB, Bus Mastering, and Flash)"]
    #[inline(always)]
    pub const fn omflen(&self) -> &Omflen {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Port address offset to host"]
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
pub use crate::espi::port0::addr;
pub use crate::espi::port0::cfg;
pub use crate::espi::port0::datain;
pub use crate::espi::port0::dataout;
pub use crate::espi::port0::irulestat;
pub use crate::espi::port0::omflen;
pub use crate::espi::port0::ramuse;
pub use crate::espi::port0::stat;
pub use crate::espi::port0::Addr;
pub use crate::espi::port0::Cfg;
pub use crate::espi::port0::Datain;
pub use crate::espi::port0::Dataout;
pub use crate::espi::port0::Irulestat;
pub use crate::espi::port0::Omflen;
pub use crate::espi::port0::Ramuse;
pub use crate::espi::port0::Stat;
