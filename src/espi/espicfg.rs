#[doc = "Register `ESPICFG` reader"]
pub type R = crate::R<EspicfgSpec>;
#[doc = "Field `MEMSZ` reader - If 1, will use 128 byte payload for memory and OOB access, else limited to 64"]
pub type MemszR = crate::BitReader;
#[doc = "Field `FLASHSZ` reader - Indicates max Flash payload size selected using same values as FLASHMX"]
pub type FlashszR = crate::FieldReader;
#[doc = "SPI Mode selected by Host\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spimod {
    #[doc = "0: SPI"]
    Spi = 0,
    #[doc = "1: BiSPI"]
    Bspi = 1,
    #[doc = "2: FLEXSPI"]
    Flexspi = 2,
}
impl From<Spimod> for u8 {
    #[inline(always)]
    fn from(variant: Spimod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spimod {
    type Ux = u8;
}
impl crate::IsEnum for Spimod {}
#[doc = "Field `SPIMOD` reader - SPI Mode selected by Host"]
pub type SpimodR = crate::FieldReader<Spimod>;
impl SpimodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Spimod> {
        match self.bits {
            0 => Some(Spimod::Spi),
            1 => Some(Spimod::Bspi),
            2 => Some(Spimod::Flexspi),
            _ => None,
        }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Spimod::Spi
    }
    #[doc = "BiSPI"]
    #[inline(always)]
    pub fn is_bspi(&self) -> bool {
        *self == Spimod::Bspi
    }
    #[doc = "FLEXSPI"]
    #[inline(always)]
    pub fn is_flexspi(&self) -> bool {
        *self == Spimod::Flexspi
    }
}
#[doc = "Field `ALERT` reader - Alert is a pin vs. MISO"]
pub type AlertR = crate::BitReader;
#[doc = "Field `ALERTOD` reader - Alert is OD as a pin"]
pub type AlertodR = crate::BitReader;
#[doc = "SPI Speed selected by Host. See note in section 2.2 on higher eSPI clock speeds and system setup.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spispd {
    #[doc = "0: 20MHz or less"]
    LessAnd20m = 0,
    #[doc = "1: 25MHz or 24MHz"]
    Freq25m24m = 1,
    #[doc = "2: 33MHz or 30MHz"]
    Freq33m30m = 2,
    #[doc = "3: 50MHz or 48MHz"]
    Freq50m48m = 3,
    #[doc = "4: 66MHz or 60MHz"]
    Freq66m60m = 4,
}
impl From<Spispd> for u8 {
    #[inline(always)]
    fn from(variant: Spispd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spispd {
    type Ux = u8;
}
impl crate::IsEnum for Spispd {}
#[doc = "Field `SPISPD` reader - SPI Speed selected by Host. See note in section 2.2 on higher eSPI clock speeds and system setup."]
pub type SpispdR = crate::FieldReader<Spispd>;
impl SpispdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Spispd> {
        match self.bits {
            0 => Some(Spispd::LessAnd20m),
            1 => Some(Spispd::Freq25m24m),
            2 => Some(Spispd::Freq33m30m),
            3 => Some(Spispd::Freq50m48m),
            4 => Some(Spispd::Freq66m60m),
            _ => None,
        }
    }
    #[doc = "20MHz or less"]
    #[inline(always)]
    pub fn is_less_and_20m(&self) -> bool {
        *self == Spispd::LessAnd20m
    }
    #[doc = "25MHz or 24MHz"]
    #[inline(always)]
    pub fn is_freq_25m_24m(&self) -> bool {
        *self == Spispd::Freq25m24m
    }
    #[doc = "33MHz or 30MHz"]
    #[inline(always)]
    pub fn is_freq_33m_30m(&self) -> bool {
        *self == Spispd::Freq33m30m
    }
    #[doc = "50MHz or 48MHz"]
    #[inline(always)]
    pub fn is_freq_50m_48m(&self) -> bool {
        *self == Spispd::Freq50m48m
    }
    #[doc = "66MHz or 60MHz"]
    #[inline(always)]
    pub fn is_freq_66m_60m(&self) -> bool {
        *self == Spispd::Freq66m60m
    }
}
#[doc = "Field `CRC` reader - CRC checking is enabled"]
pub type CrcR = crate::BitReader;
#[doc = "Field `BUSMOK` reader - Bus Master is OK"]
pub type BusmokR = crate::BitReader;
#[doc = "Field `MEMENA` reader - Channel 0 (memory) is enabled"]
pub type MemenaR = crate::BitReader;
#[doc = "Field `VWOK` reader - Channel 1 (Vwire) is enabled"]
pub type VwokR = crate::BitReader;
#[doc = "Field `OOBOK` reader - Channel 2 (OOB) is enabled"]
pub type OobokR = crate::BitReader;
#[doc = "Flash erase size (and if enabled):\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flshera {
    #[doc = "0: Flash is not enabled"]
    Disabled = 0,
    #[doc = "1: Erase is 4K"]
    Erase4k = 1,
    #[doc = "2: Erase is 64K"]
    Erase64k = 2,
    #[doc = "3: Erase allows 4K and 64K"]
    Erase4k64k = 3,
    #[doc = "4: Erase is 128K"]
    Erase128k = 4,
    #[doc = "5: Erase is 256K"]
    Erase256k = 5,
}
impl From<Flshera> for u8 {
    #[inline(always)]
    fn from(variant: Flshera) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flshera {
    type Ux = u8;
}
impl crate::IsEnum for Flshera {}
#[doc = "Field `FLSHERA` reader - Flash erase size (and if enabled):"]
pub type FlsheraR = crate::FieldReader<Flshera>;
impl FlsheraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flshera> {
        match self.bits {
            0 => Some(Flshera::Disabled),
            1 => Some(Flshera::Erase4k),
            2 => Some(Flshera::Erase64k),
            3 => Some(Flshera::Erase4k64k),
            4 => Some(Flshera::Erase128k),
            5 => Some(Flshera::Erase256k),
            _ => None,
        }
    }
    #[doc = "Flash is not enabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flshera::Disabled
    }
    #[doc = "Erase is 4K"]
    #[inline(always)]
    pub fn is_erase_4k(&self) -> bool {
        *self == Flshera::Erase4k
    }
    #[doc = "Erase is 64K"]
    #[inline(always)]
    pub fn is_erase_64k(&self) -> bool {
        *self == Flshera::Erase64k
    }
    #[doc = "Erase allows 4K and 64K"]
    #[inline(always)]
    pub fn is_erase_4k_64k(&self) -> bool {
        *self == Flshera::Erase4k64k
    }
    #[doc = "Erase is 128K"]
    #[inline(always)]
    pub fn is_erase_128k(&self) -> bool {
        *self == Flshera::Erase128k
    }
    #[doc = "Erase is 256K"]
    #[inline(always)]
    pub fn is_erase_256k(&self) -> bool {
        *self == Flshera::Erase256k
    }
}
#[doc = "Field `FLSHOK` reader - Channel 3 (Flash) is enabled."]
pub type FlshokR = crate::BitReader;
#[doc = "Field `SAF` reader - Is 1 if Slave Attached Flash is enabled. Only possible if SAF is set in ESPICAP."]
pub type SafR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If 1, will use 128 byte payload for memory and OOB access, else limited to 64"]
    #[inline(always)]
    pub fn memsz(&self) -> MemszR {
        MemszR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Indicates max Flash payload size selected using same values as FLASHMX"]
    #[inline(always)]
    pub fn flashsz(&self) -> FlashszR {
        FlashszR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - SPI Mode selected by Host"]
    #[inline(always)]
    pub fn spimod(&self) -> SpimodR {
        SpimodR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Alert is a pin vs. MISO"]
    #[inline(always)]
    pub fn alert(&self) -> AlertR {
        AlertR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alert is OD as a pin"]
    #[inline(always)]
    pub fn alertod(&self) -> AlertodR {
        AlertodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - SPI Speed selected by Host. See note in section 2.2 on higher eSPI clock speeds and system setup."]
    #[inline(always)]
    pub fn spispd(&self) -> SpispdR {
        SpispdR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - CRC checking is enabled"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Master is OK"]
    #[inline(always)]
    pub fn busmok(&self) -> BusmokR {
        BusmokR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 0 (memory) is enabled"]
    #[inline(always)]
    pub fn memena(&self) -> MemenaR {
        MemenaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 1 (Vwire) is enabled"]
    #[inline(always)]
    pub fn vwok(&self) -> VwokR {
        VwokR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 2 (OOB) is enabled"]
    #[inline(always)]
    pub fn oobok(&self) -> OobokR {
        OobokR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - Flash erase size (and if enabled):"]
    #[inline(always)]
    pub fn flshera(&self) -> FlsheraR {
        FlsheraR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Channel 3 (Flash) is enabled."]
    #[inline(always)]
    pub fn flshok(&self) -> FlshokR {
        FlshokR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Is 1 if Slave Attached Flash is enabled. Only possible if SAF is set in ESPICAP."]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPICFG")
            .field("memsz", &self.memsz())
            .field("flashsz", &self.flashsz())
            .field("spimod", &self.spimod())
            .field("alert", &self.alert())
            .field("alertod", &self.alertod())
            .field("spispd", &self.spispd())
            .field("crc", &self.crc())
            .field("busmok", &self.busmok())
            .field("memena", &self.memena())
            .field("vwok", &self.vwok())
            .field("oobok", &self.oobok())
            .field("flshera", &self.flshera())
            .field("flshok", &self.flshok())
            .field("saf", &self.saf())
            .finish()
    }
}
#[doc = "eSPI Configuration settings from eSPI\n\nYou can [`read`](crate::Reg::read) this register and get [`espicfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspicfgSpec;
impl crate::RegisterSpec for EspicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espicfg::R`](R) reader structure"]
impl crate::Readable for EspicfgSpec {}
#[doc = "`reset()` method sets ESPICFG to value 0"]
impl crate::Resettable for EspicfgSpec {
    const RESET_VALUE: u32 = 0;
}
