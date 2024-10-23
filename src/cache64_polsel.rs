#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    reg0_top: Reg0Top,
    reg1_top: Reg1Top,
    polsel: Polsel,
}
impl RegisterBlock {
    #[doc = "0x14 - Region 0 Top Boundary"]
    #[inline(always)]
    pub const fn reg0_top(&self) -> &Reg0Top {
        &self.reg0_top
    }
    #[doc = "0x18 - Region 1 Top Boundary"]
    #[inline(always)]
    pub const fn reg1_top(&self) -> &Reg1Top {
        &self.reg1_top
    }
    #[doc = "0x1c - Policy Select"]
    #[inline(always)]
    pub const fn polsel(&self) -> &Polsel {
        &self.polsel
    }
}
#[doc = "REG0_TOP (rw) register accessor: Region 0 Top Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0_top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0_top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg0_top`]
module"]
#[doc(alias = "REG0_TOP")]
pub type Reg0Top = crate::Reg<reg0_top::Reg0TopSpec>;
#[doc = "Region 0 Top Boundary"]
pub mod reg0_top;
#[doc = "REG1_TOP (rw) register accessor: Region 1 Top Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1_top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg1_top`]
module"]
#[doc(alias = "REG1_TOP")]
pub type Reg1Top = crate::Reg<reg1_top::Reg1TopSpec>;
#[doc = "Region 1 Top Boundary"]
pub mod reg1_top;
#[doc = "POLSEL (rw) register accessor: Policy Select\n\nYou can [`read`](crate::Reg::read) this register and get [`polsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polsel`]
module"]
#[doc(alias = "POLSEL")]
pub type Polsel = crate::Reg<polsel::PolselSpec>;
#[doc = "Policy Select"]
pub mod polsel;
