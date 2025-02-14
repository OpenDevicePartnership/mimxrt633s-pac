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
    #[doc = "0x08 - CRC data register, 8-bit access"]
    #[inline(always)]
    pub const fn wr_data8(&self) -> &WrData8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - CRC data register, 16-bit access"]
    #[inline(always)]
    pub const fn wr_data16(&self) -> &WrData16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - CRC data register, 32-bit access"]
    #[inline(always)]
    pub const fn wr_data32(&self) -> &WrData32 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - CRC checksum register"]
    #[inline(always)]
    pub const fn sum(&self) -> &Sum {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
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
#[doc = "WR_DATA32 (w) register accessor: CRC data register, 32-bit access\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data32::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_data32`]
module"]
#[doc(alias = "WR_DATA32")]
pub type WrData32 = crate::Reg<wr_data32::WrData32Spec>;
#[doc = "CRC data register, 32-bit access"]
pub mod wr_data32;
#[doc = "WR_DATA16 (w) register accessor: CRC data register, 16-bit access\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data16::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_data16`]
module"]
#[doc(alias = "WR_DATA16")]
pub type WrData16 = crate::Reg<wr_data16::WrData16Spec>;
#[doc = "CRC data register, 16-bit access"]
pub mod wr_data16;
#[doc = "WR_DATA8 (w) register accessor: CRC data register, 8-bit access\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_data8`]
module"]
#[doc(alias = "WR_DATA8")]
pub type WrData8 = crate::Reg<wr_data8::WrData8Spec>;
#[doc = "CRC data register, 8-bit access"]
pub mod wr_data8;
