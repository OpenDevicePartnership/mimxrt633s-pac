#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "CT32BITn Counter Timer Capture Trigger Multiplexers"]
#[doc(alias = "CT32BIT_CAP_SEL")]
pub struct Ct32bitCapSel {
    cap_sel: [CapSel; 4],
}
impl Ct32bitCapSel {
    #[doc = "0x00..0x10 - CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
    #[inline(always)]
    pub const fn cap_sel(&self, n: usize) -> &CapSel {
        &self.cap_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
    #[inline(always)]
    pub fn cap_sel_iter(&self) -> impl Iterator<Item = &CapSel> {
        self.cap_sel.iter()
    }
}
#[doc = "CAP_SEL (rw) register accessor: CT32BIT N Counter Timer Capture Trigger Multiplexers M\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_sel`]
module"]
#[doc(alias = "CAP_SEL")]
pub type CapSel = crate::Reg<cap_sel::CapSelSpec>;
#[doc = "CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
pub mod cap_sel;
