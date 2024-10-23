#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "flexcomm clock controller"]
#[doc(alias = "FLEXCOMM")]
pub struct Flexcomm {
    frgclksel: Frgclksel,
    frgctl: Frgctl,
    fcfclksel: Fcfclksel,
}
impl Flexcomm {
    #[doc = "0x00 - FRG clock selection register N"]
    #[inline(always)]
    pub const fn frgclksel(&self) -> &Frgclksel {
        &self.frgclksel
    }
    #[doc = "0x04 - FRG clock controller"]
    #[inline(always)]
    pub const fn frgctl(&self) -> &Frgctl {
        &self.frgctl
    }
    #[doc = "0x08 - flexcomm clock selection"]
    #[inline(always)]
    pub const fn fcfclksel(&self) -> &Fcfclksel {
        &self.fcfclksel
    }
}
#[doc = "FRGCLKSEL (rw) register accessor: FRG clock selection register N\n\nYou can [`read`](crate::Reg::read) this register and get [`frgclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frgclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frgclksel`]
module"]
#[doc(alias = "FRGCLKSEL")]
pub type Frgclksel = crate::Reg<frgclksel::FrgclkselSpec>;
#[doc = "FRG clock selection register N"]
pub mod frgclksel;
#[doc = "FRGCTL (rw) register accessor: FRG clock controller\n\nYou can [`read`](crate::Reg::read) this register and get [`frgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frgctl`]
module"]
#[doc(alias = "FRGCTL")]
pub type Frgctl = crate::Reg<frgctl::FrgctlSpec>;
#[doc = "FRG clock controller"]
pub mod frgctl;
#[doc = "FCFCLKSEL (rw) register accessor: flexcomm clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`fcfclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcfclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfclksel`]
module"]
#[doc(alias = "FCFCLKSEL")]
pub type Fcfclksel = crate::Reg<fcfclksel::FcfclkselSpec>;
#[doc = "flexcomm clock selection"]
pub mod fcfclksel;
