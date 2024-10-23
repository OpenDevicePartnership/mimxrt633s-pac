#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: W_\\[%s\\]"]
pub struct W {
    w_: [W_; 32],
}
impl W {
    #[doc = "0x00..0x80 - Word pin registers for all port 0 and 1 GPIO pins"]
    #[inline(always)]
    pub const fn w_(&self, n: usize) -> &W_ {
        &self.w_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Word pin registers for all port 0 and 1 GPIO pins"]
    #[inline(always)]
    pub fn w__iter(&self) -> impl Iterator<Item = &W_> {
        self.w_.iter()
    }
}
#[doc = "W_ (rw) register accessor: Word pin registers for all port 0 and 1 GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w_`]
module"]
pub type W_ = crate::Reg<w_::W_Spec>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w_;
