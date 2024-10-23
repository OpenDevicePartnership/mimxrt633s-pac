#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: CTX_KEY%s, CTX_CTR%s, CTX_RGD_W0, CTX_RGD_W1"]
#[doc(alias = "CTX")]
pub struct Ctx {
    ctx_key: [CtxKey; 4],
    ctx_ctr: [CtxCtr; 2],
    ctx_rgd_w0: CtxRgdW0,
    ctx_rgd_w1: CtxRgdW1,
}
impl Ctx {
    #[doc = "0x00..0x10 - AES Key Word"]
    #[inline(always)]
    pub const fn ctx_key(&self, n: usize) -> &CtxKey {
        &self.ctx_key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - AES Key Word"]
    #[inline(always)]
    pub fn ctx_key_iter(&self) -> impl Iterator<Item = &CtxKey> {
        self.ctx_key.iter()
    }
    #[doc = "0x10..0x18 - AES Counter Word"]
    #[inline(always)]
    pub const fn ctx_ctr(&self, n: usize) -> &CtxCtr {
        &self.ctx_ctr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - AES Counter Word"]
    #[inline(always)]
    pub fn ctx_ctr_iter(&self) -> impl Iterator<Item = &CtxCtr> {
        self.ctx_ctr.iter()
    }
    #[doc = "0x18 - AES Region Descriptor Word0"]
    #[inline(always)]
    pub const fn ctx_rgd_w0(&self) -> &CtxRgdW0 {
        &self.ctx_rgd_w0
    }
    #[doc = "0x1c - AES Region Descriptor Word1"]
    #[inline(always)]
    pub const fn ctx_rgd_w1(&self) -> &CtxRgdW1 {
        &self.ctx_rgd_w1
    }
}
#[doc = "CTX_KEY (rw) register accessor: AES Key Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_key`]
module"]
#[doc(alias = "CTX_KEY")]
pub type CtxKey = crate::Reg<ctx_key::CtxKeySpec>;
#[doc = "AES Key Word"]
pub mod ctx_key;
#[doc = "CTX_CTR (rw) register accessor: AES Counter Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_ctr`]
module"]
#[doc(alias = "CTX_CTR")]
pub type CtxCtr = crate::Reg<ctx_ctr::CtxCtrSpec>;
#[doc = "AES Counter Word"]
pub mod ctx_ctr;
#[doc = "CTX_RGD_W0 (rw) register accessor: AES Region Descriptor Word0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_rgd_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_rgd_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_rgd_w0`]
module"]
#[doc(alias = "CTX_RGD_W0")]
pub type CtxRgdW0 = crate::Reg<ctx_rgd_w0::CtxRgdW0Spec>;
#[doc = "AES Region Descriptor Word0"]
pub mod ctx_rgd_w0;
#[doc = "CTX_RGD_W1 (rw) register accessor: AES Region Descriptor Word1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctx_rgd_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctx_rgd_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctx_rgd_w1`]
module"]
#[doc(alias = "CTX_RGD_W1")]
pub type CtxRgdW1 = crate::Reg<ctx_rgd_w1::CtxRgdW1Spec>;
#[doc = "AES Region Descriptor Word1"]
pub mod ctx_rgd_w1;
