#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    channel: [Channel; 4],
    _reserved1: [u8; 0xb0],
    modcfg: Modcfg,
    idle_ch: IdleCh,
    irq_flag: IrqFlag,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Array of registers: INTVAL, TIMER, CTRL, STAT"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Array of registers: INTVAL, TIMER, CTRL, STAT"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
    #[doc = "0xf0 - Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
    #[inline(always)]
    pub const fn modcfg(&self) -> &Modcfg {
        &self.modcfg
    }
    #[doc = "0xf4 - Idle channel register. This register returns the number of the first idle channel."]
    #[inline(always)]
    pub const fn idle_ch(&self) -> &IdleCh {
        &self.idle_ch
    }
    #[doc = "0xf8 - Global interrupt flag register"]
    #[inline(always)]
    pub const fn irq_flag(&self) -> &IrqFlag {
        &self.irq_flag
    }
}
#[doc = "Array of registers: INTVAL, TIMER, CTRL, STAT"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Array of registers: INTVAL, TIMER, CTRL, STAT"]
pub mod channel;
#[doc = "MODCFG (rw) register accessor: Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`modcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modcfg`]
module"]
#[doc(alias = "MODCFG")]
pub type Modcfg = crate::Reg<modcfg::ModcfgSpec>;
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
pub mod modcfg;
#[doc = "IDLE_CH (r) register accessor: Idle channel register. This register returns the number of the first idle channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle_ch`]
module"]
#[doc(alias = "IDLE_CH")]
pub type IdleCh = crate::Reg<idle_ch::IdleChSpec>;
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
pub mod idle_ch;
#[doc = "IRQ_FLAG (rw) register accessor: Global interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_flag`]
module"]
#[doc(alias = "IRQ_FLAG")]
pub type IrqFlag = crate::Reg<irq_flag::IrqFlagSpec>;
#[doc = "Global interrupt flag register"]
pub mod irq_flag;
