#[doc = "Register `XFERCFG` reader"]
pub type R = crate::R<XfercfgSpec>;
#[doc = "Register `XFERCFG` writer"]
pub type W = crate::W<XfercfgSpec>;
#[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgvalid {
    #[doc = "0: Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    NotValid = 0,
    #[doc = "1: Valid. The current channel descriptor is considered valid."]
    Valid = 1,
}
impl From<Cfgvalid> for bool {
    #[inline(always)]
    fn from(variant: Cfgvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGVALID` reader - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
pub type CfgvalidR = crate::BitReader<Cfgvalid>;
impl CfgvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfgvalid {
        match self.bits {
            false => Cfgvalid::NotValid,
            true => Cfgvalid::Valid,
        }
    }
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == Cfgvalid::NotValid
    }
    #[doc = "Valid. The current channel descriptor is considered valid."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Cfgvalid::Valid
    }
}
#[doc = "Field `CFGVALID` writer - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
pub type CfgvalidW<'a, REG> = crate::BitWriter<'a, REG, Cfgvalid>;
impl<'a, REG> CfgvalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgvalid::NotValid)
    }
    #[doc = "Valid. The current channel descriptor is considered valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgvalid::Valid)
    }
}
#[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reload {
    #[doc = "0: Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    Disabled = 0,
    #[doc = "1: Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    Enabled = 1,
}
impl From<Reload> for bool {
    #[inline(always)]
    fn from(variant: Reload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
pub type ReloadR = crate::BitReader<Reload>;
impl ReloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reload {
        match self.bits {
            false => Reload::Disabled,
            true => Reload::Enabled,
        }
    }
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Reload::Disabled
    }
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Reload::Enabled
    }
}
#[doc = "Field `RELOAD` writer - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG, Reload>;
impl<'a, REG> ReloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Disabled)
    }
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Enabled)
    }
}
#[doc = "Software Trigger.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swtrig {
    #[doc = "0: Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    NotSet = 0,
    #[doc = "1: Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    Set = 1,
}
impl From<Swtrig> for bool {
    #[inline(always)]
    fn from(variant: Swtrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG` reader - Software Trigger."]
pub type SwtrigR = crate::BitReader<Swtrig>;
impl SwtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swtrig {
        match self.bits {
            false => Swtrig::NotSet,
            true => Swtrig::Set,
        }
    }
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Swtrig::NotSet
    }
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Swtrig::Set
    }
}
#[doc = "Field `SWTRIG` writer - Software Trigger."]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG, Swtrig>;
impl<'a, REG> SwtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    #[inline(always)]
    pub fn not_set(self) -> &'a mut crate::W<REG> {
        self.variant(Swtrig::NotSet)
    }
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Swtrig::Set)
    }
}
#[doc = "Clear Trigger.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrtrig {
    #[doc = "0: Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NotCleared = 0,
    #[doc = "1: Cleared. The trigger is cleared when this descriptor is exhausted"]
    Cleared = 1,
}
impl From<Clrtrig> for bool {
    #[inline(always)]
    fn from(variant: Clrtrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRTRIG` reader - Clear Trigger."]
pub type ClrtrigR = crate::BitReader<Clrtrig>;
impl ClrtrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clrtrig {
        match self.bits {
            false => Clrtrig::NotCleared,
            true => Clrtrig::Cleared,
        }
    }
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    #[inline(always)]
    pub fn is_not_cleared(&self) -> bool {
        *self == Clrtrig::NotCleared
    }
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == Clrtrig::Cleared
    }
}
#[doc = "Field `CLRTRIG` writer - Clear Trigger."]
pub type ClrtrigW<'a, REG> = crate::BitWriter<'a, REG, Clrtrig>;
impl<'a, REG> ClrtrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    #[inline(always)]
    pub fn not_cleared(self) -> &'a mut crate::W<REG> {
        self.variant(Clrtrig::NotCleared)
    }
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(Clrtrig::Cleared)
    }
}
#[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setinta {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    Set = 1,
}
impl From<Setinta> for bool {
    #[inline(always)]
    fn from(variant: Setinta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETINTA` reader - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
pub type SetintaR = crate::BitReader<Setinta>;
impl SetintaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Setinta {
        match self.bits {
            false => Setinta::NoEffect,
            true => Setinta::Set,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Setinta::NoEffect
    }
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Setinta::Set
    }
}
#[doc = "Field `SETINTA` writer - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
pub type SetintaW<'a, REG> = crate::BitWriter<'a, REG, Setinta>;
impl<'a, REG> SetintaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Setinta::NoEffect)
    }
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Setinta::Set)
    }
}
#[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setintb {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    Set = 1,
}
impl From<Setintb> for bool {
    #[inline(always)]
    fn from(variant: Setintb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETINTB` reader - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
pub type SetintbR = crate::BitReader<Setintb>;
impl SetintbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Setintb {
        match self.bits {
            false => Setintb::NoEffect,
            true => Setintb::Set,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Setintb::NoEffect
    }
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Setintb::Set
    }
}
#[doc = "Field `SETINTB` writer - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
pub type SetintbW<'a, REG> = crate::BitWriter<'a, REG, Setintb>;
impl<'a, REG> SetintbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Setintb::NoEffect)
    }
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Setintb::Set)
    }
}
#[doc = "Transfer width used for this DMA channel.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Width {
    #[doc = "0: 8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    Bit8 = 0,
    #[doc = "1: 16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    Bit16 = 1,
    #[doc = "2: 32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    Bit32 = 2,
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(variant: Width) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Width {
    type Ux = u8;
}
impl crate::IsEnum for Width {}
#[doc = "Field `WIDTH` reader - Transfer width used for this DMA channel."]
pub type WidthR = crate::FieldReader<Width>;
impl WidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Width> {
        match self.bits {
            0 => Some(Width::Bit8),
            1 => Some(Width::Bit16),
            2 => Some(Width::Bit32),
            _ => None,
        }
    }
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == Width::Bit8
    }
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn is_bit_16(&self) -> bool {
        *self == Width::Bit16
    }
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn is_bit_32(&self) -> bool {
        *self == Width::Bit32
    }
}
#[doc = "Field `WIDTH` writer - Transfer width used for this DMA channel."]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Width>;
impl<'a, REG> WidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut crate::W<REG> {
        self.variant(Width::Bit8)
    }
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn bit_16(self) -> &'a mut crate::W<REG> {
        self.variant(Width::Bit16)
    }
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    #[inline(always)]
    pub fn bit_32(self) -> &'a mut crate::W<REG> {
        self.variant(Width::Bit32)
    }
}
#[doc = "Determines whether the source address is incremented for each DMA transfer.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcinc {
    #[doc = "0: No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NoIncrement = 0,
    #[doc = "1: 1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WidthX1 = 1,
    #[doc = "2: 2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WidthX2 = 2,
    #[doc = "3: 4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WidthX4 = 3,
}
impl From<Srcinc> for u8 {
    #[inline(always)]
    fn from(variant: Srcinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcinc {
    type Ux = u8;
}
impl crate::IsEnum for Srcinc {}
#[doc = "Field `SRCINC` reader - Determines whether the source address is incremented for each DMA transfer."]
pub type SrcincR = crate::FieldReader<Srcinc>;
impl SrcincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcinc {
        match self.bits {
            0 => Srcinc::NoIncrement,
            1 => Srcinc::WidthX1,
            2 => Srcinc::WidthX2,
            3 => Srcinc::WidthX4,
            _ => unreachable!(),
        }
    }
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == Srcinc::NoIncrement
    }
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    #[inline(always)]
    pub fn is_width_x_1(&self) -> bool {
        *self == Srcinc::WidthX1
    }
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn is_width_x_2(&self) -> bool {
        *self == Srcinc::WidthX2
    }
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn is_width_x_4(&self) -> bool {
        *self == Srcinc::WidthX4
    }
}
#[doc = "Field `SRCINC` writer - Determines whether the source address is incremented for each DMA transfer."]
pub type SrcincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srcinc, crate::Safe>;
impl<'a, REG> SrcincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::NoIncrement)
    }
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    #[inline(always)]
    pub fn width_x_1(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::WidthX1)
    }
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_2(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::WidthX2)
    }
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_4(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::WidthX4)
    }
}
#[doc = "Determines whether the destination address is incremented for each DMA transfer.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dstinc {
    #[doc = "0: No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NoIncrement = 0,
    #[doc = "1: 1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WidthX1 = 1,
    #[doc = "2: 2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WidthX2 = 2,
    #[doc = "3: 4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WidthX4 = 3,
}
impl From<Dstinc> for u8 {
    #[inline(always)]
    fn from(variant: Dstinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dstinc {
    type Ux = u8;
}
impl crate::IsEnum for Dstinc {}
#[doc = "Field `DSTINC` reader - Determines whether the destination address is incremented for each DMA transfer."]
pub type DstincR = crate::FieldReader<Dstinc>;
impl DstincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstinc {
        match self.bits {
            0 => Dstinc::NoIncrement,
            1 => Dstinc::WidthX1,
            2 => Dstinc::WidthX2,
            3 => Dstinc::WidthX4,
            _ => unreachable!(),
        }
    }
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == Dstinc::NoIncrement
    }
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    #[inline(always)]
    pub fn is_width_x_1(&self) -> bool {
        *self == Dstinc::WidthX1
    }
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn is_width_x_2(&self) -> bool {
        *self == Dstinc::WidthX2
    }
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn is_width_x_4(&self) -> bool {
        *self == Dstinc::WidthX4
    }
}
#[doc = "Field `DSTINC` writer - Determines whether the destination address is incremented for each DMA transfer."]
pub type DstincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dstinc, crate::Safe>;
impl<'a, REG> DstincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::NoIncrement)
    }
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    #[inline(always)]
    pub fn width_x_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::WidthX1)
    }
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::WidthX2)
    }
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    #[inline(always)]
    pub fn width_x_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::WidthX4)
    }
}
#[doc = "Field `XFERCOUNT` reader - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
pub type XfercountR = crate::FieldReader<u16>;
#[doc = "Field `XFERCOUNT` writer - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
pub type XfercountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub fn cfgvalid(&self) -> CfgvalidR {
        CfgvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline(always)]
    pub fn swtrig(&self) -> SwtrigR {
        SwtrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline(always)]
    pub fn clrtrig(&self) -> ClrtrigR {
        ClrtrigR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setinta(&self) -> SetintaR {
        SetintaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setintb(&self) -> SetintbR {
        SetintbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn srcinc(&self) -> SrcincR {
        SrcincR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn dstinc(&self) -> DstincR {
        DstincR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub fn xfercount(&self) -> XfercountR {
        XfercountR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XFERCFG")
            .field("cfgvalid", &self.cfgvalid())
            .field("reload", &self.reload())
            .field("swtrig", &self.swtrig())
            .field("clrtrig", &self.clrtrig())
            .field("setinta", &self.setinta())
            .field("setintb", &self.setintb())
            .field("width", &self.width())
            .field("srcinc", &self.srcinc())
            .field("dstinc", &self.dstinc())
            .field("xfercount", &self.xfercount())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub fn cfgvalid(&mut self) -> CfgvalidW<XfercfgSpec> {
        CfgvalidW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<XfercfgSpec> {
        ReloadW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Trigger."]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SwtrigW<XfercfgSpec> {
        SwtrigW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Trigger."]
    #[inline(always)]
    pub fn clrtrig(&mut self) -> ClrtrigW<XfercfgSpec> {
        ClrtrigW::new(self, 3)
    }
    #[doc = "Bit 4 - Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setinta(&mut self) -> SetintaW<XfercfgSpec> {
        SetintaW::new(self, 4)
    }
    #[doc = "Bit 5 - Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn setintb(&mut self) -> SetintbW<XfercfgSpec> {
        SetintbW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Transfer width used for this DMA channel."]
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<XfercfgSpec> {
        WidthW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SrcincW<XfercfgSpec> {
        SrcincW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DstincW<XfercfgSpec> {
        DstincW::new(self, 14)
    }
    #[doc = "Bits 16:25 - Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub fn xfercount(&mut self) -> XfercountW<XfercfgSpec> {
        XfercountW::new(self, 16)
    }
}
#[doc = "Transfer configuration register for DMA channel .\n\nYou can [`read`](crate::Reg::read) this register and get [`xfercfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xfercfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XfercfgSpec;
impl crate::RegisterSpec for XfercfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xfercfg::R`](R) reader structure"]
impl crate::Readable for XfercfgSpec {}
#[doc = "`write(|w| ..)` method takes [`xfercfg::W`](W) writer structure"]
impl crate::Writable for XfercfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XFERCFG to value 0"]
impl crate::Resettable for XfercfgSpec {
    const RESET_VALUE: u32 = 0;
}
