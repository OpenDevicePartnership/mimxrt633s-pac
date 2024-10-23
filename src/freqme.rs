#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_freqmectrl: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Freqeuncy Measurement (in Write mode)"]
    #[inline(always)]
    pub const fn freqmectrl_w(&self) -> &FreqmectrlW {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Frequency Measurement (in Read mode)"]
    #[inline(always)]
    pub const fn freqmectrl_r(&self) -> &FreqmectrlR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
}
#[doc = "FREQMECTRL_R (r) register accessor: Frequency Measurement (in Read mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`freqmectrl_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqmectrl_r`]
module"]
#[doc(alias = "FREQMECTRL_R")]
pub type FreqmectrlR = crate::Reg<freqmectrl_r::FreqmectrlRSpec>;
#[doc = "Frequency Measurement (in Read mode)"]
pub mod freqmectrl_r;
#[doc = "FREQMECTRL_W (w) register accessor: Freqeuncy Measurement (in Write mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqmectrl_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqmectrl_w`]
module"]
#[doc(alias = "FREQMECTRL_W")]
pub type FreqmectrlW = crate::Reg<freqmectrl_w::FreqmectrlWSpec>;
#[doc = "Freqeuncy Measurement (in Write mode)"]
pub mod freqmectrl_w;
