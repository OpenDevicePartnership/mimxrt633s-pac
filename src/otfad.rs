#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    cr: Cr,
    sr: Sr,
    _reserved2: [u8; 0xf8],
    ctx: (),
}
impl RegisterBlock {
    #[doc = "0xc00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0xc04 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0xd00..0xd80 - Array of registers: CTX_KEY%s, CTX_CTR%s, CTX_RGD_W0, CTX_RGD_W1"]
    #[inline(always)]
    pub const fn ctx(&self, n: usize) -> &Ctx {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(3328)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd00..0xd80 - Array of registers: CTX_KEY%s, CTX_CTR%s, CTX_RGD_W0, CTX_RGD_W1"]
    #[inline(always)]
    pub fn ctx_iter(&self) -> impl Iterator<Item = &Ctx> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(3328)
                .add(64 * n)
                .cast()
        })
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "Array of registers: CTX_KEY%s, CTX_CTR%s, CTX_RGD_W0, CTX_RGD_W1"]
pub use self::ctx::Ctx;
#[doc = r"Cluster"]
#[doc = "Array of registers: CTX_KEY%s, CTX_CTR%s, CTX_RGD_W0, CTX_RGD_W1"]
pub mod ctx;
