#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: OUT_SET, OUT_CLR"]
#[doc(alias = "OUT")]
pub struct Out {
    out_set: OutSet,
    out_clr: OutClr,
}
impl Out {
    #[doc = "0x00 - SCT output 0 set register"]
    #[inline(always)]
    pub const fn out_set(&self) -> &OutSet {
        &self.out_set
    }
    #[doc = "0x04 - SCT output 0 clear register"]
    #[inline(always)]
    pub const fn out_clr(&self) -> &OutClr {
        &self.out_clr
    }
}
#[doc = "OUT_SET (rw) register accessor: SCT output 0 set register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_set`]
module"]
#[doc(alias = "OUT_SET")]
pub type OutSet = crate::Reg<out_set::OutSetSpec>;
#[doc = "SCT output 0 set register"]
pub mod out_set;
#[doc = "OUT_CLR (rw) register accessor: SCT output 0 clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_clr`]
module"]
#[doc(alias = "OUT_CLR")]
pub type OutClr = crate::Reg<out_clr::OutClrSpec>;
#[doc = "SCT output 0 clear register"]
pub mod out_clr;
