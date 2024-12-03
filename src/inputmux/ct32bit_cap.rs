#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "CT32BITn Counter Timer Capture Trigger Multiplexers"]
#[doc(alias = "CT32BIT_CAP")]
pub struct Ct32bitCap {
    ct32bit_cap_sel: [Ct32bitCapSel; 4],
}
impl Ct32bitCap {
    #[doc = "0x00..0x10 - CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
    #[inline(always)]
    pub const fn ct32bit_cap_sel(&self, n: usize) -> &Ct32bitCapSel {
        &self.ct32bit_cap_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
    #[inline(always)]
    pub fn ct32bit_cap_sel_iter(&self) -> impl Iterator<Item = &Ct32bitCapSel> {
        self.ct32bit_cap_sel.iter()
    }
}
#[doc = "CT32BIT_CAP_SEL (rw) register accessor: CT32BIT N Counter Timer Capture Trigger Multiplexers M\n\nYou can [`read`](crate::Reg::read) this register and get [`ct32bit_cap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ct32bit_cap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ct32bit_cap_sel`]
module"]
#[doc(alias = "CT32BIT_CAP_SEL")]
pub type Ct32bitCapSel = crate::Reg<ct32bit_cap_sel::Ct32bitCapSelSpec>;
#[doc = "CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
pub mod ct32bit_cap_sel;
