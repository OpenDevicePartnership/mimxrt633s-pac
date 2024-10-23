#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mctrl: Mctrl,
    mstat: Mstat,
    intenset: Intenset,
    intenclr: Intenclr,
    intstat: Intstat,
    dmactrl: Dmactrl,
    rambase: Rambase,
    mapbase: Mapbase,
    irqpush: Irqpush,
    wirewo: Wirewo,
    wirero: Wirero,
    _reserved11: [u8; 0x04],
    p80stat: P80stat,
    stataddr: Stataddr,
    _reserved13: [u8; 0x08],
    espicap: Espicap,
    espicfg: Espicfg,
    espimisc: Espimisc,
    _reserved16: [u8; 0xb4],
    p0cfg: P0cfg,
    p0stat: P0stat,
    p0irule_stat: P0iruleStat,
    _reserved_19_p: [u8; 0x04],
    p0data_in: P0dataIn,
    _reserved_21_p: [u8; 0x04],
    _reserved22: [u8; 0x08],
    p1cfg: P1cfg,
    p1stat: P1stat,
    p1irule_stat: P1iruleStat,
    _reserved_25_p: [u8; 0x04],
    p1data_in: P1dataIn,
    _reserved_27_p: [u8; 0x04],
    _reserved28: [u8; 0x08],
    p2cfg: P2cfg,
    p2stat: P2stat,
    p2irule_stat: P2iruleStat,
    _reserved_31_p: [u8; 0x04],
    p2data_in: P2dataIn,
    _reserved_33_p: [u8; 0x04],
    _reserved34: [u8; 0x08],
    p3cfg: P3cfg,
    p3stat: P3stat,
    p3irule_stat: P3iruleStat,
    _reserved_37_p: [u8; 0x04],
    p3data_in: P3dataIn,
    _reserved_39_p: [u8; 0x04],
    _reserved40: [u8; 0x08],
    p4cfg: P4cfg,
    p4stat: P4stat,
    p4irule_stat: P4iruleStat,
    _reserved_43_p: [u8; 0x04],
    p4data_in: P4dataIn,
    _reserved_45_p: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Master Control for whole peripheral"]
    #[inline(always)]
    pub const fn mctrl(&self) -> &Mctrl {
        &self.mctrl
    }
    #[doc = "0x04 - Master Status of whole peripheral"]
    #[inline(always)]
    pub const fn mstat(&self) -> &Mstat {
        &self.mstat
    }
    #[doc = "0x08 - Interrupt Set (enable)"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - Interrupt Clear (disable)"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - Masked interrupt status (causes)"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x14 - Selects DMA for Ports (if used)"]
    #[inline(always)]
    pub const fn dmactrl(&self) -> &Dmactrl {
        &self.dmactrl
    }
    #[doc = "0x18 - Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable)."]
    #[inline(always)]
    pub const fn rambase(&self) -> &Rambase {
        &self.rambase
    }
    #[doc = "0x1c - Base0 and Base1 mapped offsets for ports"]
    #[inline(always)]
    pub const fn mapbase(&self) -> &Mapbase {
        &self.mapbase
    }
    #[doc = "0x20 - IRQ to drive into Host (with eSPI)"]
    #[inline(always)]
    pub const fn irqpush(&self) -> &Irqpush {
        &self.irqpush
    }
    #[doc = "0x24 - Wire states for Host to see; if LPC, this is the IRQ states."]
    #[inline(always)]
    pub const fn wirewo(&self) -> &Wirewo {
        &self.wirewo
    }
    #[doc = "0x28 - Wire states from Host"]
    #[inline(always)]
    pub const fn wirero(&self) -> &Wirero {
        &self.wirero
    }
    #[doc = "0x30 - Port 80 Status (byte and prev byte)"]
    #[inline(always)]
    pub const fn p80stat(&self) -> &P80stat {
        &self.p80stat
    }
    #[doc = "0x34 - Location of Status block in memory space, if enabled."]
    #[inline(always)]
    pub const fn stataddr(&self) -> &Stataddr {
        &self.stataddr
    }
    #[doc = "0x40 - eSPI Capabilities of MCU to send to Host"]
    #[inline(always)]
    pub const fn espicap(&self) -> &Espicap {
        &self.espicap
    }
    #[doc = "0x44 - eSPI Configuration settings from eSPI"]
    #[inline(always)]
    pub const fn espicfg(&self) -> &Espicfg {
        &self.espicfg
    }
    #[doc = "0x48 - Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert)."]
    #[inline(always)]
    pub const fn espimisc(&self) -> &Espimisc {
        &self.espimisc
    }
    #[doc = "0x100 - no description available"]
    #[inline(always)]
    pub const fn p0cfg(&self) -> &P0cfg {
        &self.p0cfg
    }
    #[doc = "0x104 - no description available"]
    #[inline(always)]
    pub const fn p0stat(&self) -> &P0stat {
        &self.p0stat
    }
    #[doc = "0x108 - The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p0irule_stat(&self) -> &P0iruleStat {
        &self.p0irule_stat
    }
    #[doc = "0x10c - The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p0omflen(&self) -> &P0omflen {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
    #[inline(always)]
    pub const fn p0addr(&self) -> &P0addr {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x110 - no description available"]
    #[inline(always)]
    pub const fn p0data_in(&self) -> &P0dataIn {
        &self.p0data_in
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub const fn p0ramuse(&self) -> &P0ramuse {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub const fn p0data_out(&self) -> &P0dataOut {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x120 - no description available"]
    #[inline(always)]
    pub const fn p1cfg(&self) -> &P1cfg {
        &self.p1cfg
    }
    #[doc = "0x124 - no description available"]
    #[inline(always)]
    pub const fn p1stat(&self) -> &P1stat {
        &self.p1stat
    }
    #[doc = "0x128 - The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p1irule_stat(&self) -> &P1iruleStat {
        &self.p1irule_stat
    }
    #[doc = "0x12c - The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p1omflen(&self) -> &P1omflen {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
    #[inline(always)]
    pub const fn p1addr(&self) -> &P1addr {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub const fn p1data_in(&self) -> &P1dataIn {
        &self.p1data_in
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn p1ramuse(&self) -> &P1ramuse {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub const fn p1data_out(&self) -> &P1dataOut {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub const fn p2cfg(&self) -> &P2cfg {
        &self.p2cfg
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub const fn p2stat(&self) -> &P2stat {
        &self.p2stat
    }
    #[doc = "0x148 - The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p2irule_stat(&self) -> &P2iruleStat {
        &self.p2irule_stat
    }
    #[doc = "0x14c - The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p2omflen(&self) -> &P2omflen {
        unsafe { &*(self as *const Self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x14c - The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
    #[inline(always)]
    pub const fn p2addr(&self) -> &P2addr {
        unsafe { &*(self as *const Self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x150 - no description available"]
    #[inline(always)]
    pub const fn p2data_in(&self) -> &P2dataIn {
        &self.p2data_in
    }
    #[doc = "0x154 - no description available"]
    #[inline(always)]
    pub const fn p2ramuse(&self) -> &P2ramuse {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - no description available"]
    #[inline(always)]
    pub const fn p2data_out(&self) -> &P2dataOut {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x160 - no description available"]
    #[inline(always)]
    pub const fn p3cfg(&self) -> &P3cfg {
        &self.p3cfg
    }
    #[doc = "0x164 - no description available"]
    #[inline(always)]
    pub const fn p3stat(&self) -> &P3stat {
        &self.p3stat
    }
    #[doc = "0x168 - The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p3irule_stat(&self) -> &P3iruleStat {
        &self.p3irule_stat
    }
    #[doc = "0x16c - The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p3omflen(&self) -> &P3omflen {
        unsafe { &*(self as *const Self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x16c - The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
    #[inline(always)]
    pub const fn p3addr(&self) -> &P3addr {
        unsafe { &*(self as *const Self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x170 - no description available"]
    #[inline(always)]
    pub const fn p3data_in(&self) -> &P3dataIn {
        &self.p3data_in
    }
    #[doc = "0x174 - no description available"]
    #[inline(always)]
    pub const fn p3ramuse(&self) -> &P3ramuse {
        unsafe { &*(self as *const Self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x174 - no description available"]
    #[inline(always)]
    pub const fn p3data_out(&self) -> &P3dataOut {
        unsafe { &*(self as *const Self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x180 - no description available"]
    #[inline(always)]
    pub const fn p4cfg(&self) -> &P4cfg {
        &self.p4cfg
    }
    #[doc = "0x184 - no description available"]
    #[inline(always)]
    pub const fn p4stat(&self) -> &P4stat {
        &self.p4stat
    }
    #[doc = "0x188 - The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p4irule_stat(&self) -> &P4iruleStat {
        &self.p4irule_stat
    }
    #[doc = "0x18c - The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p4omflen(&self) -> &P4omflen {
        unsafe { &*(self as *const Self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x18c - The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
    #[inline(always)]
    pub const fn p4addr(&self) -> &P4addr {
        unsafe { &*(self as *const Self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x190 - no description available"]
    #[inline(always)]
    pub const fn p4data_in(&self) -> &P4dataIn {
        &self.p4data_in
    }
    #[doc = "0x194 - no description available"]
    #[inline(always)]
    pub const fn p4ramuse(&self) -> &P4ramuse {
        unsafe { &*(self as *const Self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x194 - no description available"]
    #[inline(always)]
    pub const fn p4data_out(&self) -> &P4dataOut {
        unsafe { &*(self as *const Self).cast::<u8>().add(404).cast() }
    }
}
#[doc = "MCTRL (rw) register accessor: Master Control for whole peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`]
module"]
#[doc(alias = "MCTRL")]
pub type Mctrl = crate::Reg<mctrl::MctrlSpec>;
#[doc = "Master Control for whole peripheral"]
pub mod mctrl;
#[doc = "MSTAT (rw) register accessor: Master Status of whole peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`mstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstat`]
module"]
#[doc(alias = "MSTAT")]
pub type Mstat = crate::Reg<mstat::MstatSpec>;
#[doc = "Master Status of whole peripheral"]
pub mod mstat;
#[doc = "INTENSET (rw) register accessor: Interrupt Set (enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Set (enable)"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Interrupt Clear (disable)\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Clear (disable)"]
pub mod intenclr;
#[doc = "INTSTAT (rw) register accessor: Masked interrupt status (causes)\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`]
module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Masked interrupt status (causes)"]
pub mod intstat;
#[doc = "DMACTRL (rw) register accessor: Selects DMA for Ports (if used)\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl`]
module"]
#[doc(alias = "DMACTRL")]
pub type Dmactrl = crate::Reg<dmactrl::DmactrlSpec>;
#[doc = "Selects DMA for Ports (if used)"]
pub mod dmactrl;
#[doc = "RAMBASE (rw) register accessor: Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable).\n\nYou can [`read`](crate::Reg::read) this register and get [`rambase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rambase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rambase`]
module"]
#[doc(alias = "RAMBASE")]
pub type Rambase = crate::Reg<rambase::RambaseSpec>;
#[doc = "Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable)."]
pub mod rambase;
#[doc = "MAPBASE (rw) register accessor: Base0 and Base1 mapped offsets for ports\n\nYou can [`read`](crate::Reg::read) this register and get [`mapbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mapbase`]
module"]
#[doc(alias = "MAPBASE")]
pub type Mapbase = crate::Reg<mapbase::MapbaseSpec>;
#[doc = "Base0 and Base1 mapped offsets for ports"]
pub mod mapbase;
#[doc = "IRQPUSH (rw) register accessor: IRQ to drive into Host (with eSPI)\n\nYou can [`read`](crate::Reg::read) this register and get [`irqpush::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqpush::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqpush`]
module"]
#[doc(alias = "IRQPUSH")]
pub type Irqpush = crate::Reg<irqpush::IrqpushSpec>;
#[doc = "IRQ to drive into Host (with eSPI)"]
pub mod irqpush;
#[doc = "WIREWO (rw) register accessor: Wire states for Host to see; if LPC, this is the IRQ states.\n\nYou can [`read`](crate::Reg::read) this register and get [`wirewo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wirewo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wirewo`]
module"]
#[doc(alias = "WIREWO")]
pub type Wirewo = crate::Reg<wirewo::WirewoSpec>;
#[doc = "Wire states for Host to see; if LPC, this is the IRQ states."]
pub mod wirewo;
#[doc = "WIRERO (r) register accessor: Wire states from Host\n\nYou can [`read`](crate::Reg::read) this register and get [`wirero::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wirero`]
module"]
#[doc(alias = "WIRERO")]
pub type Wirero = crate::Reg<wirero::WireroSpec>;
#[doc = "Wire states from Host"]
pub mod wirero;
#[doc = "P80STAT (rw) register accessor: Port 80 Status (byte and prev byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`p80stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p80stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p80stat`]
module"]
#[doc(alias = "P80STAT")]
pub type P80stat = crate::Reg<p80stat::P80statSpec>;
#[doc = "Port 80 Status (byte and prev byte)"]
pub mod p80stat;
#[doc = "STATADDR (rw) register accessor: Location of Status block in memory space, if enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`stataddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stataddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stataddr`]
module"]
#[doc(alias = "STATADDR")]
pub type Stataddr = crate::Reg<stataddr::StataddrSpec>;
#[doc = "Location of Status block in memory space, if enabled."]
pub mod stataddr;
#[doc = "ESPICAP (rw) register accessor: eSPI Capabilities of MCU to send to Host\n\nYou can [`read`](crate::Reg::read) this register and get [`espicap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espicap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espicap`]
module"]
#[doc(alias = "ESPICAP")]
pub type Espicap = crate::Reg<espicap::EspicapSpec>;
#[doc = "eSPI Capabilities of MCU to send to Host"]
pub mod espicap;
#[doc = "ESPICFG (r) register accessor: eSPI Configuration settings from eSPI\n\nYou can [`read`](crate::Reg::read) this register and get [`espicfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espicfg`]
module"]
#[doc(alias = "ESPICFG")]
pub type Espicfg = crate::Reg<espicfg::EspicfgSpec>;
#[doc = "eSPI Configuration settings from eSPI"]
pub mod espicfg;
#[doc = "ESPIMISC (rw) register accessor: Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert).\n\nYou can [`read`](crate::Reg::read) this register and get [`espimisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espimisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espimisc`]
module"]
#[doc(alias = "ESPIMISC")]
pub type Espimisc = crate::Reg<espimisc::EspimiscSpec>;
#[doc = "Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert)."]
pub mod espimisc;
#[doc = "P0CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p0cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0cfg`]
module"]
#[doc(alias = "P0CFG")]
pub type P0cfg = crate::Reg<p0cfg::P0cfgSpec>;
#[doc = "no description available"]
pub mod p0cfg;
#[doc = "P0STAT (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0stat`]
module"]
#[doc(alias = "P0STAT")]
pub type P0stat = crate::Reg<p0stat::P0statSpec>;
#[doc = "no description available"]
pub mod p0stat;
#[doc = "P0IRuleStat (rw) register accessor: The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET.\n\nYou can [`read`](crate::Reg::read) this register and get [`p0irule_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0irule_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0irule_stat`]
module"]
#[doc(alias = "P0IRuleStat")]
pub type P0iruleStat = crate::Reg<p0irule_stat::P0iruleStatSpec>;
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
pub mod p0irule_stat;
#[doc = "P0ADDR (rw) register accessor: The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F).\n\nYou can [`read`](crate::Reg::read) this register and get [`p0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0addr`]
module"]
#[doc(alias = "P0ADDR")]
pub type P0addr = crate::Reg<p0addr::P0addrSpec>;
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
pub mod p0addr;
#[doc = "P0OMFLEN (rw) register accessor: The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`p0omflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0omflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0omflen`]
module"]
#[doc(alias = "P0OMFLEN")]
pub type P0omflen = crate::Reg<p0omflen::P0omflenSpec>;
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
pub mod p0omflen;
#[doc = "P0DataIn (r) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p0data_in::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0data_in`]
module"]
#[doc(alias = "P0DataIn")]
pub type P0dataIn = crate::Reg<p0data_in::P0dataInSpec>;
#[doc = "no description available"]
pub mod p0data_in;
#[doc = "P0DataOut (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p0data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0data_out`]
module"]
#[doc(alias = "P0DataOut")]
pub type P0dataOut = crate::Reg<p0data_out::P0dataOutSpec>;
#[doc = "no description available"]
pub mod p0data_out;
#[doc = "P0RAMUse (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p0ramuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ramuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p0ramuse`]
module"]
#[doc(alias = "P0RAMUse")]
pub type P0ramuse = crate::Reg<p0ramuse::P0ramuseSpec>;
#[doc = "no description available"]
pub mod p0ramuse;
#[doc = "P1CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p1cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1cfg`]
module"]
#[doc(alias = "P1CFG")]
pub type P1cfg = crate::Reg<p1cfg::P1cfgSpec>;
#[doc = "no description available"]
pub mod p1cfg;
#[doc = "P1STAT (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1stat`]
module"]
#[doc(alias = "P1STAT")]
pub type P1stat = crate::Reg<p1stat::P1statSpec>;
#[doc = "no description available"]
pub mod p1stat;
#[doc = "P1IRuleStat (rw) register accessor: The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET.\n\nYou can [`read`](crate::Reg::read) this register and get [`p1irule_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1irule_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1irule_stat`]
module"]
#[doc(alias = "P1IRuleStat")]
pub type P1iruleStat = crate::Reg<p1irule_stat::P1iruleStatSpec>;
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
pub mod p1irule_stat;
#[doc = "P1ADDR (rw) register accessor: The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F).\n\nYou can [`read`](crate::Reg::read) this register and get [`p1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1addr`]
module"]
#[doc(alias = "P1ADDR")]
pub type P1addr = crate::Reg<p1addr::P1addrSpec>;
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
pub mod p1addr;
#[doc = "P1OMFLEN (rw) register accessor: The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`p1omflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1omflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1omflen`]
module"]
#[doc(alias = "P1OMFLEN")]
pub type P1omflen = crate::Reg<p1omflen::P1omflenSpec>;
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
pub mod p1omflen;
#[doc = "P1DataIn (r) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p1data_in::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1data_in`]
module"]
#[doc(alias = "P1DataIn")]
pub type P1dataIn = crate::Reg<p1data_in::P1dataInSpec>;
#[doc = "no description available"]
pub mod p1data_in;
#[doc = "P1DataOut (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p1data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1data_out`]
module"]
#[doc(alias = "P1DataOut")]
pub type P1dataOut = crate::Reg<p1data_out::P1dataOutSpec>;
#[doc = "no description available"]
pub mod p1data_out;
#[doc = "P1RAMUse (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ramuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ramuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ramuse`]
module"]
#[doc(alias = "P1RAMUse")]
pub type P1ramuse = crate::Reg<p1ramuse::P1ramuseSpec>;
#[doc = "no description available"]
pub mod p1ramuse;
#[doc = "P2CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p2cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2cfg`]
module"]
#[doc(alias = "P2CFG")]
pub type P2cfg = crate::Reg<p2cfg::P2cfgSpec>;
#[doc = "no description available"]
pub mod p2cfg;
#[doc = "P2STAT (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p2stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2stat`]
module"]
#[doc(alias = "P2STAT")]
pub type P2stat = crate::Reg<p2stat::P2statSpec>;
#[doc = "no description available"]
pub mod p2stat;
#[doc = "P2IRuleStat (rw) register accessor: The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET.\n\nYou can [`read`](crate::Reg::read) this register and get [`p2irule_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2irule_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2irule_stat`]
module"]
#[doc(alias = "P2IRuleStat")]
pub type P2iruleStat = crate::Reg<p2irule_stat::P2iruleStatSpec>;
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
pub mod p2irule_stat;
#[doc = "P2ADDR (rw) register accessor: The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F).\n\nYou can [`read`](crate::Reg::read) this register and get [`p2addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2addr`]
module"]
#[doc(alias = "P2ADDR")]
pub type P2addr = crate::Reg<p2addr::P2addrSpec>;
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
pub mod p2addr;
#[doc = "P2OMFLEN (rw) register accessor: The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`p2omflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2omflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2omflen`]
module"]
#[doc(alias = "P2OMFLEN")]
pub type P2omflen = crate::Reg<p2omflen::P2omflenSpec>;
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
pub mod p2omflen;
#[doc = "P2DataIn (r) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p2data_in::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2data_in`]
module"]
#[doc(alias = "P2DataIn")]
pub type P2dataIn = crate::Reg<p2data_in::P2dataInSpec>;
#[doc = "no description available"]
pub mod p2data_in;
#[doc = "P2DataOut (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p2data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2data_out`]
module"]
#[doc(alias = "P2DataOut")]
pub type P2dataOut = crate::Reg<p2data_out::P2dataOutSpec>;
#[doc = "no description available"]
pub mod p2data_out;
#[doc = "P2RAMUse (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p2ramuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ramuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ramuse`]
module"]
#[doc(alias = "P2RAMUse")]
pub type P2ramuse = crate::Reg<p2ramuse::P2ramuseSpec>;
#[doc = "no description available"]
pub mod p2ramuse;
#[doc = "P3CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p3cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3cfg`]
module"]
#[doc(alias = "P3CFG")]
pub type P3cfg = crate::Reg<p3cfg::P3cfgSpec>;
#[doc = "no description available"]
pub mod p3cfg;
#[doc = "P3STAT (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p3stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3stat`]
module"]
#[doc(alias = "P3STAT")]
pub type P3stat = crate::Reg<p3stat::P3statSpec>;
#[doc = "no description available"]
pub mod p3stat;
#[doc = "P3IRuleStat (rw) register accessor: The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET.\n\nYou can [`read`](crate::Reg::read) this register and get [`p3irule_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3irule_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3irule_stat`]
module"]
#[doc(alias = "P3IRuleStat")]
pub type P3iruleStat = crate::Reg<p3irule_stat::P3iruleStatSpec>;
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
pub mod p3irule_stat;
#[doc = "P3ADDR (rw) register accessor: The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F).\n\nYou can [`read`](crate::Reg::read) this register and get [`p3addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3addr`]
module"]
#[doc(alias = "P3ADDR")]
pub type P3addr = crate::Reg<p3addr::P3addrSpec>;
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
pub mod p3addr;
#[doc = "P3OMFLEN (rw) register accessor: The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`p3omflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3omflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3omflen`]
module"]
#[doc(alias = "P3OMFLEN")]
pub type P3omflen = crate::Reg<p3omflen::P3omflenSpec>;
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
pub mod p3omflen;
#[doc = "P3DataIn (r) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p3data_in::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3data_in`]
module"]
#[doc(alias = "P3DataIn")]
pub type P3dataIn = crate::Reg<p3data_in::P3dataInSpec>;
#[doc = "no description available"]
pub mod p3data_in;
#[doc = "P3DataOut (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p3data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3data_out`]
module"]
#[doc(alias = "P3DataOut")]
pub type P3dataOut = crate::Reg<p3data_out::P3dataOutSpec>;
#[doc = "no description available"]
pub mod p3data_out;
#[doc = "P3RAMUse (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ramuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ramuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ramuse`]
module"]
#[doc(alias = "P3RAMUse")]
pub type P3ramuse = crate::Reg<p3ramuse::P3ramuseSpec>;
#[doc = "no description available"]
pub mod p3ramuse;
#[doc = "P4CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p4cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4cfg`]
module"]
#[doc(alias = "P4CFG")]
pub type P4cfg = crate::Reg<p4cfg::P4cfgSpec>;
#[doc = "no description available"]
pub mod p4cfg;
#[doc = "P4STAT (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p4stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4stat`]
module"]
#[doc(alias = "P4STAT")]
pub type P4stat = crate::Reg<p4stat::P4statSpec>;
#[doc = "no description available"]
pub mod p4stat;
#[doc = "P4IRuleStat (rw) register accessor: The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET.\n\nYou can [`read`](crate::Reg::read) this register and get [`p4irule_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4irule_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4irule_stat`]
module"]
#[doc(alias = "P4IRuleStat")]
pub type P4iruleStat = crate::Reg<p4irule_stat::P4iruleStatSpec>;
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
pub mod p4irule_stat;
#[doc = "P4ADDR (rw) register accessor: The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F).\n\nYou can [`read`](crate::Reg::read) this register and get [`p4addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4addr`]
module"]
#[doc(alias = "P4ADDR")]
pub type P4addr = crate::Reg<p4addr::P4addrSpec>;
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &amp;~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &amp;~0x1F)."]
pub mod p4addr;
#[doc = "P4OMFLEN (rw) register accessor: The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`p4omflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4omflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4omflen`]
module"]
#[doc(alias = "P4OMFLEN")]
pub type P4omflen = crate::Reg<p4omflen::P4omflenSpec>;
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
pub mod p4omflen;
#[doc = "P4DataIn (r) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p4data_in::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4data_in`]
module"]
#[doc(alias = "P4DataIn")]
pub type P4dataIn = crate::Reg<p4data_in::P4dataInSpec>;
#[doc = "no description available"]
pub mod p4data_in;
#[doc = "P4DataOut (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p4data_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4data_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4data_out`]
module"]
#[doc(alias = "P4DataOut")]
pub type P4dataOut = crate::Reg<p4data_out::P4dataOutSpec>;
#[doc = "no description available"]
pub mod p4data_out;
#[doc = "P4RAMUse (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ramuse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ramuse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4ramuse`]
module"]
#[doc(alias = "P4RAMUse")]
pub type P4ramuse = crate::Reg<p4ramuse::P4ramuseSpec>;
#[doc = "no description available"]
pub mod p4ramuse;
