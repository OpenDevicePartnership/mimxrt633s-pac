#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Periphreqen {
    #[doc = "0: Disabled. Peripheral DMA requests are disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. Peripheral DMA requests are enabled."]
    Enabled = 1,
}
impl From<Periphreqen> for bool {
    #[inline(always)]
    fn from(variant: Periphreqen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIPHREQEN` reader - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
pub type PeriphreqenR = crate::BitReader<Periphreqen>;
impl PeriphreqenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Periphreqen {
        match self.bits {
            false => Periphreqen::Disabled,
            true => Periphreqen::Enabled,
        }
    }
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Periphreqen::Disabled
    }
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Periphreqen::Enabled
    }
}
#[doc = "Field `PERIPHREQEN` writer - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
pub type PeriphreqenW<'a, REG> = crate::BitWriter<'a, REG, Periphreqen>;
impl<'a, REG> PeriphreqenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Periphreqen::Disabled)
    }
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Periphreqen::Enabled)
    }
}
#[doc = "Hardware Triggering Enable for this channel.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwtrigen {
    #[doc = "0: Disabled. Hardware triggering is not used."]
    Disabled = 0,
    #[doc = "1: Enabled. Use hardware triggering."]
    Enabled = 1,
}
impl From<Hwtrigen> for bool {
    #[inline(always)]
    fn from(variant: Hwtrigen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWTRIGEN` reader - Hardware Triggering Enable for this channel."]
pub type HwtrigenR = crate::BitReader<Hwtrigen>;
impl HwtrigenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwtrigen {
        match self.bits {
            false => Hwtrigen::Disabled,
            true => Hwtrigen::Enabled,
        }
    }
    #[doc = "Disabled. Hardware triggering is not used."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hwtrigen::Disabled
    }
    #[doc = "Enabled. Use hardware triggering."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hwtrigen::Enabled
    }
}
#[doc = "Field `HWTRIGEN` writer - Hardware Triggering Enable for this channel."]
pub type HwtrigenW<'a, REG> = crate::BitWriter<'a, REG, Hwtrigen>;
impl<'a, REG> HwtrigenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Hardware triggering is not used."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwtrigen::Disabled)
    }
    #[doc = "Enabled. Use hardware triggering."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwtrigen::Enabled)
    }
}
#[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigpol {
    #[doc = "0: Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ActiveLowFalling = 0,
    #[doc = "1: Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ActiveHighRising = 1,
}
impl From<Trigpol> for bool {
    #[inline(always)]
    fn from(variant: Trigpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGPOL` reader - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
pub type TrigpolR = crate::BitReader<Trigpol>;
impl TrigpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigpol {
        match self.bits {
            false => Trigpol::ActiveLowFalling,
            true => Trigpol::ActiveHighRising,
        }
    }
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn is_active_low_falling(&self) -> bool {
        *self == Trigpol::ActiveLowFalling
    }
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn is_active_high_rising(&self) -> bool {
        *self == Trigpol::ActiveHighRising
    }
}
#[doc = "Field `TRIGPOL` writer - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
pub type TrigpolW<'a, REG> = crate::BitWriter<'a, REG, Trigpol>;
impl<'a, REG> TrigpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_low_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Trigpol::ActiveLowFalling)
    }
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    #[inline(always)]
    pub fn active_high_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Trigpol::ActiveHighRising)
    }
}
#[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigtype {
    #[doc = "0: Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    Edge = 0,
    #[doc = "1: Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    Level = 1,
}
impl From<Trigtype> for bool {
    #[inline(always)]
    fn from(variant: Trigtype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGTYPE` reader - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
pub type TrigtypeR = crate::BitReader<Trigtype>;
impl TrigtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigtype {
        match self.bits {
            false => Trigtype::Edge,
            true => Trigtype::Level,
        }
    }
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Trigtype::Edge
    }
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Trigtype::Level
    }
}
#[doc = "Field `TRIGTYPE` writer - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
pub type TrigtypeW<'a, REG> = crate::BitWriter<'a, REG, Trigtype>;
impl<'a, REG> TrigtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Trigtype::Edge)
    }
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Trigtype::Level)
    }
}
#[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigburst {
    #[doc = "0: Single transfer. Hardware trigger causes a single transfer."]
    Single = 0,
    #[doc = "1: Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    Burst = 1,
}
impl From<Trigburst> for bool {
    #[inline(always)]
    fn from(variant: Trigburst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGBURST` reader - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
pub type TrigburstR = crate::BitReader<Trigburst>;
impl TrigburstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigburst {
        match self.bits {
            false => Trigburst::Single,
            true => Trigburst::Burst,
        }
    }
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Trigburst::Single
    }
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Trigburst::Burst
    }
}
#[doc = "Field `TRIGBURST` writer - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
pub type TrigburstW<'a, REG> = crate::BitWriter<'a, REG, Trigburst>;
impl<'a, REG> TrigburstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Trigburst::Single)
    }
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Trigburst::Burst)
    }
}
#[doc = "Field `BURSTPOWER` reader - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
pub type BurstpowerR = crate::FieldReader;
#[doc = "Field `BURSTPOWER` writer - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
pub type BurstpowerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcburstwrap {
    #[doc = "0: Disabled. Source burst wrapping is not enabled for this DMA channel."]
    Disabled = 0,
    #[doc = "1: Enabled. Source burst wrapping is enabled for this DMA channel."]
    Enabled = 1,
}
impl From<Srcburstwrap> for bool {
    #[inline(always)]
    fn from(variant: Srcburstwrap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCBURSTWRAP` reader - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
pub type SrcburstwrapR = crate::BitReader<Srcburstwrap>;
impl SrcburstwrapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcburstwrap {
        match self.bits {
            false => Srcburstwrap::Disabled,
            true => Srcburstwrap::Enabled,
        }
    }
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Srcburstwrap::Disabled
    }
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Srcburstwrap::Enabled
    }
}
#[doc = "Field `SRCBURSTWRAP` writer - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
pub type SrcburstwrapW<'a, REG> = crate::BitWriter<'a, REG, Srcburstwrap>;
impl<'a, REG> SrcburstwrapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Srcburstwrap::Disabled)
    }
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Srcburstwrap::Enabled)
    }
}
#[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dstburstwrap {
    #[doc = "0: Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    Disabled = 0,
    #[doc = "1: Enabled. Destination burst wrapping is enabled for this DMA channel."]
    Enabled = 1,
}
impl From<Dstburstwrap> for bool {
    #[inline(always)]
    fn from(variant: Dstburstwrap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTBURSTWRAP` reader - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
pub type DstburstwrapR = crate::BitReader<Dstburstwrap>;
impl DstburstwrapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstburstwrap {
        match self.bits {
            false => Dstburstwrap::Disabled,
            true => Dstburstwrap::Enabled,
        }
    }
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dstburstwrap::Disabled
    }
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dstburstwrap::Enabled
    }
}
#[doc = "Field `DSTBURSTWRAP` writer - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
pub type DstburstwrapW<'a, REG> = crate::BitWriter<'a, REG, Dstburstwrap>;
impl<'a, REG> DstburstwrapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dstburstwrap::Disabled)
    }
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dstburstwrap::Enabled)
    }
}
#[doc = "Field `CHPRIORITY` reader - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
pub type ChpriorityR = crate::FieldReader;
#[doc = "Field `CHPRIORITY` writer - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
pub type ChpriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub fn periphreqen(&self) -> PeriphreqenR {
        PeriphreqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub fn hwtrigen(&self) -> HwtrigenR {
        HwtrigenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub fn trigpol(&self) -> TrigpolR {
        TrigpolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub fn trigtype(&self) -> TrigtypeR {
        TrigtypeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub fn trigburst(&self) -> TrigburstR {
        TrigburstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub fn burstpower(&self) -> BurstpowerR {
        BurstpowerR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub fn srcburstwrap(&self) -> SrcburstwrapR {
        SrcburstwrapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub fn dstburstwrap(&self) -> DstburstwrapR {
        DstburstwrapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub fn chpriority(&self) -> ChpriorityR {
        ChpriorityR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("periphreqen", &self.periphreqen())
            .field("hwtrigen", &self.hwtrigen())
            .field("trigpol", &self.trigpol())
            .field("trigtype", &self.trigtype())
            .field("trigburst", &self.trigburst())
            .field("burstpower", &self.burstpower())
            .field("srcburstwrap", &self.srcburstwrap())
            .field("dstburstwrap", &self.dstburstwrap())
            .field("chpriority", &self.chpriority())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub fn periphreqen(&mut self) -> PeriphreqenW<CfgSpec> {
        PeriphreqenW::new(self, 0)
    }
    #[doc = "Bit 1 - Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub fn hwtrigen(&mut self) -> HwtrigenW<CfgSpec> {
        HwtrigenW::new(self, 1)
    }
    #[doc = "Bit 4 - Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub fn trigpol(&mut self) -> TrigpolW<CfgSpec> {
        TrigpolW::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub fn trigtype(&mut self) -> TrigtypeW<CfgSpec> {
        TrigtypeW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub fn trigburst(&mut self) -> TrigburstW<CfgSpec> {
        TrigburstW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub fn burstpower(&mut self) -> BurstpowerW<CfgSpec> {
        BurstpowerW::new(self, 8)
    }
    #[doc = "Bit 14 - Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub fn srcburstwrap(&mut self) -> SrcburstwrapW<CfgSpec> {
        SrcburstwrapW::new(self, 14)
    }
    #[doc = "Bit 15 - Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub fn dstburstwrap(&mut self) -> DstburstwrapW<CfgSpec> {
        DstburstwrapW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub fn chpriority(&mut self) -> ChpriorityW<CfgSpec> {
        ChpriorityW::new(self, 16)
    }
}
#[doc = "Configuration register for DMA channel .\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
