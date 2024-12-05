#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    gate: [Gate; 16],
    _reserved1: [u8; 0x32],
    _reserved_1_rstgt: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - Semphores2 Gate n"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `GATE3` register.</div>"]
    #[inline(always)]
    pub const fn gate(&self, n: usize) -> &Gate {
        &self.gate[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Semphores2 Gate n"]
    #[inline(always)]
    pub fn gate_iter(&self) -> impl Iterator<Item = &Gate> {
        self.gate.iter()
    }
    #[doc = "0x00 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate3(&self) -> &Gate {
        self.gate(0)
    }
    #[doc = "0x01 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate2(&self) -> &Gate {
        self.gate(1)
    }
    #[doc = "0x02 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate1(&self) -> &Gate {
        self.gate(2)
    }
    #[doc = "0x03 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate0(&self) -> &Gate {
        self.gate(3)
    }
    #[doc = "0x04 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate7(&self) -> &Gate {
        self.gate(4)
    }
    #[doc = "0x05 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate6(&self) -> &Gate {
        self.gate(5)
    }
    #[doc = "0x06 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate5(&self) -> &Gate {
        self.gate(6)
    }
    #[doc = "0x07 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate4(&self) -> &Gate {
        self.gate(7)
    }
    #[doc = "0x08 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate11(&self) -> &Gate {
        self.gate(8)
    }
    #[doc = "0x09 - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate10(&self) -> &Gate {
        self.gate(9)
    }
    #[doc = "0x0a - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate9(&self) -> &Gate {
        self.gate(10)
    }
    #[doc = "0x0b - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate8(&self) -> &Gate {
        self.gate(11)
    }
    #[doc = "0x0c - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate15(&self) -> &Gate {
        self.gate(12)
    }
    #[doc = "0x0d - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate14(&self) -> &Gate {
        self.gate(13)
    }
    #[doc = "0x0e - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate13(&self) -> &Gate {
        self.gate(14)
    }
    #[doc = "0x0f - Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate12(&self) -> &Gate {
        self.gate(15)
    }
    #[doc = "0x42 - Reset Gate Write"]
    #[inline(always)]
    pub const fn rstgt_w(&self) -> &RstgtW {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x42 - Reset Gate Read"]
    #[inline(always)]
    pub const fn rstgt_r(&self) -> &RstgtR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
}
#[doc = "GATE (rw) register accessor: Semphores2 Gate n\n\nYou can [`read`](crate::Reg::read) this register and get [`gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gate`]
module"]
#[doc(alias = "GATE")]
pub type Gate = crate::Reg<gate::GateSpec>;
#[doc = "Semphores2 Gate n"]
pub mod gate;
#[doc = "RSTGT_R (r) register accessor: Reset Gate Read\n\nYou can [`read`](crate::Reg::read) this register and get [`rstgt_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstgt_r`]
module"]
#[doc(alias = "RSTGT_R")]
pub type RstgtR = crate::Reg<rstgt_r::RstgtRSpec>;
#[doc = "Reset Gate Read"]
pub mod rstgt_r;
#[doc = "RSTGT_W (w) register accessor: Reset Gate Write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstgt_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstgt_w`]
module"]
#[doc(alias = "RSTGT_W")]
pub type RstgtW = crate::Reg<rstgt_w::RstgtWSpec>;
#[doc = "Reset Gate Write"]
pub mod rstgt_w;
