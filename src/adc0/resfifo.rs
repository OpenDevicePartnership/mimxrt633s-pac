#[doc = "Register `RESFIFO` reader"]
pub type R = crate::R<ResfifoSpec>;
#[doc = "Field `D` reader - Data result"]
pub type DR = crate::FieldReader<u16>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsrc {
    #[doc = "0: Trigger source 0 initiated this conversion."]
    Tsrc0 = 0,
    #[doc = "1: Trigger source 1 initiated this conversion."]
    Tsrc1 = 1,
    #[doc = "2: Corresponding trigger source initiated this conversion."]
    Tsrc2 = 2,
    #[doc = "3: Corresponding trigger source initiated this conversion."]
    Tsrc3 = 3,
    #[doc = "4: Corresponding trigger source initiated this conversion."]
    Tsrc4 = 4,
    #[doc = "5: Corresponding trigger source initiated this conversion."]
    Tsrc5 = 5,
    #[doc = "6: Corresponding trigger source initiated this conversion."]
    Tsrc6 = 6,
    #[doc = "7: Corresponding trigger source initiated this conversion."]
    Tsrc7 = 7,
    #[doc = "8: Corresponding trigger source initiated this conversion."]
    Tsrc8 = 8,
    #[doc = "9: Corresponding trigger source initiated this conversion."]
    Tsrc9 = 9,
    #[doc = "15: Trigger source 15 initiated this conversion."]
    Tsrc15 = 15,
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(variant: Tsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsrc {
    type Ux = u8;
}
impl crate::IsEnum for Tsrc {}
#[doc = "Field `TSRC` reader - Trigger Source"]
pub type TsrcR = crate::FieldReader<Tsrc>;
impl TsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsrc> {
        match self.bits {
            0 => Some(Tsrc::Tsrc0),
            1 => Some(Tsrc::Tsrc1),
            2 => Some(Tsrc::Tsrc2),
            3 => Some(Tsrc::Tsrc3),
            4 => Some(Tsrc::Tsrc4),
            5 => Some(Tsrc::Tsrc5),
            6 => Some(Tsrc::Tsrc6),
            7 => Some(Tsrc::Tsrc7),
            8 => Some(Tsrc::Tsrc8),
            9 => Some(Tsrc::Tsrc9),
            15 => Some(Tsrc::Tsrc15),
            _ => None,
        }
    }
    #[doc = "Trigger source 0 initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_0(&self) -> bool {
        *self == Tsrc::Tsrc0
    }
    #[doc = "Trigger source 1 initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_1(&self) -> bool {
        *self == Tsrc::Tsrc1
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_2(&self) -> bool {
        *self == Tsrc::Tsrc2
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_3(&self) -> bool {
        *self == Tsrc::Tsrc3
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_4(&self) -> bool {
        *self == Tsrc::Tsrc4
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_5(&self) -> bool {
        *self == Tsrc::Tsrc5
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_6(&self) -> bool {
        *self == Tsrc::Tsrc6
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_7(&self) -> bool {
        *self == Tsrc::Tsrc7
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_8(&self) -> bool {
        *self == Tsrc::Tsrc8
    }
    #[doc = "Corresponding trigger source initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_9(&self) -> bool {
        *self == Tsrc::Tsrc9
    }
    #[doc = "Trigger source 15 initiated this conversion."]
    #[inline(always)]
    pub fn is_tsrc_15(&self) -> bool {
        *self == Tsrc::Tsrc15
    }
}
#[doc = "Loop count value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Loopcnt {
    #[doc = "0: Result is from initial conversion in command."]
    Loopcnt0 = 0,
    #[doc = "1: Result is from second conversion in command."]
    Loopcnt1 = 1,
    #[doc = "2: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt2 = 2,
    #[doc = "3: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt3 = 3,
    #[doc = "4: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt4 = 4,
    #[doc = "5: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt5 = 5,
    #[doc = "6: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt6 = 6,
    #[doc = "7: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt7 = 7,
    #[doc = "8: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt8 = 8,
    #[doc = "9: Result is from LOOPCNT+1 conversion in command."]
    Loopcnt9 = 9,
    #[doc = "15: Result is from 16th conversion in command."]
    Loopcnt15 = 15,
}
impl From<Loopcnt> for u8 {
    #[inline(always)]
    fn from(variant: Loopcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Loopcnt {
    type Ux = u8;
}
impl crate::IsEnum for Loopcnt {}
#[doc = "Field `LOOPCNT` reader - Loop count value"]
pub type LoopcntR = crate::FieldReader<Loopcnt>;
impl LoopcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Loopcnt> {
        match self.bits {
            0 => Some(Loopcnt::Loopcnt0),
            1 => Some(Loopcnt::Loopcnt1),
            2 => Some(Loopcnt::Loopcnt2),
            3 => Some(Loopcnt::Loopcnt3),
            4 => Some(Loopcnt::Loopcnt4),
            5 => Some(Loopcnt::Loopcnt5),
            6 => Some(Loopcnt::Loopcnt6),
            7 => Some(Loopcnt::Loopcnt7),
            8 => Some(Loopcnt::Loopcnt8),
            9 => Some(Loopcnt::Loopcnt9),
            15 => Some(Loopcnt::Loopcnt15),
            _ => None,
        }
    }
    #[doc = "Result is from initial conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_0(&self) -> bool {
        *self == Loopcnt::Loopcnt0
    }
    #[doc = "Result is from second conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_1(&self) -> bool {
        *self == Loopcnt::Loopcnt1
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_2(&self) -> bool {
        *self == Loopcnt::Loopcnt2
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_3(&self) -> bool {
        *self == Loopcnt::Loopcnt3
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_4(&self) -> bool {
        *self == Loopcnt::Loopcnt4
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_5(&self) -> bool {
        *self == Loopcnt::Loopcnt5
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_6(&self) -> bool {
        *self == Loopcnt::Loopcnt6
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_7(&self) -> bool {
        *self == Loopcnt::Loopcnt7
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_8(&self) -> bool {
        *self == Loopcnt::Loopcnt8
    }
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_9(&self) -> bool {
        *self == Loopcnt::Loopcnt9
    }
    #[doc = "Result is from 16th conversion in command."]
    #[inline(always)]
    pub fn is_loopcnt_15(&self) -> bool {
        *self == Loopcnt::Loopcnt15
    }
}
#[doc = "Command Buffer Source\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdsrc {
    #[doc = "0: Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    Cmdsrc0 = 0,
    #[doc = "1: CMD1 buffer used as control settings for this conversion."]
    Cmdsrc1 = 1,
    #[doc = "2: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc2 = 2,
    #[doc = "3: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc3 = 3,
    #[doc = "4: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc4 = 4,
    #[doc = "5: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc5 = 5,
    #[doc = "6: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc6 = 6,
    #[doc = "7: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc7 = 7,
    #[doc = "8: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc8 = 8,
    #[doc = "9: Corresponding command buffer used as control settings for this conversion."]
    Cmdsrc9 = 9,
    #[doc = "15: CMD15 buffer used as control settings for this conversion."]
    Cmdsrc15 = 15,
}
impl From<Cmdsrc> for u8 {
    #[inline(always)]
    fn from(variant: Cmdsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdsrc {
    type Ux = u8;
}
impl crate::IsEnum for Cmdsrc {}
#[doc = "Field `CMDSRC` reader - Command Buffer Source"]
pub type CmdsrcR = crate::FieldReader<Cmdsrc>;
impl CmdsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmdsrc> {
        match self.bits {
            0 => Some(Cmdsrc::Cmdsrc0),
            1 => Some(Cmdsrc::Cmdsrc1),
            2 => Some(Cmdsrc::Cmdsrc2),
            3 => Some(Cmdsrc::Cmdsrc3),
            4 => Some(Cmdsrc::Cmdsrc4),
            5 => Some(Cmdsrc::Cmdsrc5),
            6 => Some(Cmdsrc::Cmdsrc6),
            7 => Some(Cmdsrc::Cmdsrc7),
            8 => Some(Cmdsrc::Cmdsrc8),
            9 => Some(Cmdsrc::Cmdsrc9),
            15 => Some(Cmdsrc::Cmdsrc15),
            _ => None,
        }
    }
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    #[inline(always)]
    pub fn is_cmdsrc_0(&self) -> bool {
        *self == Cmdsrc::Cmdsrc0
    }
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_1(&self) -> bool {
        *self == Cmdsrc::Cmdsrc1
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_2(&self) -> bool {
        *self == Cmdsrc::Cmdsrc2
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_3(&self) -> bool {
        *self == Cmdsrc::Cmdsrc3
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_4(&self) -> bool {
        *self == Cmdsrc::Cmdsrc4
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_5(&self) -> bool {
        *self == Cmdsrc::Cmdsrc5
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_6(&self) -> bool {
        *self == Cmdsrc::Cmdsrc6
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_7(&self) -> bool {
        *self == Cmdsrc::Cmdsrc7
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_8(&self) -> bool {
        *self == Cmdsrc::Cmdsrc8
    }
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_9(&self) -> bool {
        *self == Cmdsrc::Cmdsrc9
    }
    #[doc = "CMD15 buffer used as control settings for this conversion."]
    #[inline(always)]
    pub fn is_cmdsrc_15(&self) -> bool {
        *self == Cmdsrc::Cmdsrc15
    }
}
#[doc = "FIFO entry is valid\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valid {
    #[doc = "0: FIFO is empty. Discard any read from RESFIFO."]
    Valid0 = 0,
    #[doc = "1: FIFO record read from RESFIFO is valid."]
    Valid1 = 1,
}
impl From<Valid> for bool {
    #[inline(always)]
    fn from(variant: Valid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - FIFO entry is valid"]
pub type ValidR = crate::BitReader<Valid>;
impl ValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Valid {
        match self.bits {
            false => Valid::Valid0,
            true => Valid::Valid1,
        }
    }
    #[doc = "FIFO is empty. Discard any read from RESFIFO."]
    #[inline(always)]
    pub fn is_valid_0(&self) -> bool {
        *self == Valid::Valid0
    }
    #[doc = "FIFO record read from RESFIFO is valid."]
    #[inline(always)]
    pub fn is_valid_1(&self) -> bool {
        *self == Valid::Valid1
    }
}
impl R {
    #[doc = "Bits 3:15 - Data result"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:19 - Trigger Source"]
    #[inline(always)]
    pub fn tsrc(&self) -> TsrcR {
        TsrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Loop count value"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Command Buffer Source"]
    #[inline(always)]
    pub fn cmdsrc(&self) -> CmdsrcR {
        CmdsrcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FIFO entry is valid"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESFIFO")
            .field("d", &self.d())
            .field("tsrc", &self.tsrc())
            .field("loopcnt", &self.loopcnt())
            .field("cmdsrc", &self.cmdsrc())
            .field("valid", &self.valid())
            .finish()
    }
}
#[doc = "ADC Data Result FIFO Register\n\nYou can [`read`](crate::Reg::read) this register and get [`resfifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResfifoSpec;
impl crate::RegisterSpec for ResfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resfifo::R`](R) reader structure"]
impl crate::Readable for ResfifoSpec {}
#[doc = "`reset()` method sets RESFIFO to value 0"]
impl crate::Resettable for ResfifoSpec {
    const RESET_VALUE: u32 = 0;
}
