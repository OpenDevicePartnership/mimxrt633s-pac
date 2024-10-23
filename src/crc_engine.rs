#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: Mode,
    seed: Seed,
    _reserved_2_sum: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - CRC seed register"]
    #[inline(always)]
    pub const fn seed(&self) -> &Seed {
        &self.seed
    }
    #[doc = "0x08 - CRC data register"]
    #[inline(always)]
    pub const fn wr_data(&self) -> &WrData {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - CRC checksum register"]
    #[inline(always)]
    pub const fn sum(&self) -> &Sum {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
}
#[doc = "MODE (rw) register accessor: CRC mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "CRC mode register"]
pub mod mode;
#[doc = "SEED (rw) register accessor: CRC seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`seed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seed`]
module"]
#[doc(alias = "SEED")]
pub type Seed = crate::Reg<seed::SeedSpec>;
#[doc = "CRC seed register"]
pub mod seed;
#[doc = "SUM (r) register accessor: CRC checksum register\n\nYou can [`read`](crate::Reg::read) this register and get [`sum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sum`]
module"]
#[doc(alias = "SUM")]
pub type Sum = crate::Reg<sum::SumSpec>;
#[doc = "CRC checksum register"]
pub mod sum;
#[doc = "WR_DATA (w) register accessor: CRC data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_data`]
module"]
#[doc(alias = "WR_DATA")]
pub type WrData = crate::Reg<wr_data::WrDataSpec>;
#[doc = "CRC data register"]
pub mod wr_data;
