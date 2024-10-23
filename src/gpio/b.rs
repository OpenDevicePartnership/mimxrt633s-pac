#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Array of registers: B_\\[%s\\]"]
pub struct B {
    b_: [B_; 32],
}
impl B {
    #[doc = "0x00..0x20 - Byte pin registers for all port 0 and 1 GPIO pins"]
    #[inline(always)]
    pub const fn b_(&self, n: usize) -> &B_ {
        &self.b_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Byte pin registers for all port 0 and 1 GPIO pins"]
    #[inline(always)]
    pub fn b__iter(&self) -> impl Iterator<Item = &B_> {
        self.b_.iter()
    }
}
#[doc = "B_ (rw) register accessor: Byte pin registers for all port 0 and 1 GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`b_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_`]
module"]
pub type B_ = crate::Reg<b_::B_Spec>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b_;
