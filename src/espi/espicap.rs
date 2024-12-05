#[doc = "Register `ESPICAP` reader"]
pub type R = crate::R<EspicapSpec>;
#[doc = "Register `ESPICAP` writer"]
pub type W = crate::W<EspicapSpec>;
#[doc = "SPI mode allowed (host still has to select):\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spicap {
    #[doc = "0: SPI only"]
    Spi = 0,
    #[doc = "1: BiSPI and SPI"]
    BspiSpi = 1,
    #[doc = "2: FLEXSPI and SPI"]
    FlexspiSpi = 2,
    #[doc = "3: any"]
    Any = 3,
}
impl From<Spicap> for u8 {
    #[inline(always)]
    fn from(variant: Spicap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spicap {
    type Ux = u8;
}
impl crate::IsEnum for Spicap {}
#[doc = "Field `SPICAP` reader - SPI mode allowed (host still has to select):"]
pub type SpicapR = crate::FieldReader<Spicap>;
impl SpicapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spicap {
        match self.bits {
            0 => Spicap::Spi,
            1 => Spicap::BspiSpi,
            2 => Spicap::FlexspiSpi,
            3 => Spicap::Any,
            _ => unreachable!(),
        }
    }
    #[doc = "SPI only"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Spicap::Spi
    }
    #[doc = "BiSPI and SPI"]
    #[inline(always)]
    pub fn is_bspi_spi(&self) -> bool {
        *self == Spicap::BspiSpi
    }
    #[doc = "FLEXSPI and SPI"]
    #[inline(always)]
    pub fn is_flexspi_spi(&self) -> bool {
        *self == Spicap::FlexspiSpi
    }
    #[doc = "any"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == Spicap::Any
    }
}
#[doc = "Field `SPICAP` writer - SPI mode allowed (host still has to select):"]
pub type SpicapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Spicap, crate::Safe>;
impl<'a, REG> SpicapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI only"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Spicap::Spi)
    }
    #[doc = "BiSPI and SPI"]
    #[inline(always)]
    pub fn bspi_spi(self) -> &'a mut crate::W<REG> {
        self.variant(Spicap::BspiSpi)
    }
    #[doc = "FLEXSPI and SPI"]
    #[inline(always)]
    pub fn flexspi_spi(self) -> &'a mut crate::W<REG> {
        self.variant(Spicap::FlexspiSpi)
    }
    #[doc = "any"]
    #[inline(always)]
    pub fn any(self) -> &'a mut crate::W<REG> {
        self.variant(Spicap::Any)
    }
}
#[doc = "Maximum SPI Clock speed to allow (host still chooses):\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxspd {
    #[doc = "0: <=20MHz"]
    SmallThan20m = 0,
    #[doc = "1: <=25MHz (may be 24Mhz)"]
    SmallThan25m = 1,
    #[doc = "2: <=33MHz (may be 30MHz)"]
    SmallThan33m = 2,
    #[doc = "3: <=50MHz (may be 48MHz)"]
    SmallThan50m = 3,
    #[doc = "4: <= 66MHz (may be 60MHz)"]
    SmallThan66m = 4,
}
impl From<Maxspd> for u8 {
    #[inline(always)]
    fn from(variant: Maxspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxspd {
    type Ux = u8;
}
impl crate::IsEnum for Maxspd {}
#[doc = "Field `MAXSPD` reader - Maximum SPI Clock speed to allow (host still chooses):"]
pub type MaxspdR = crate::FieldReader<Maxspd>;
impl MaxspdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxspd> {
        match self.bits {
            0 => Some(Maxspd::SmallThan20m),
            1 => Some(Maxspd::SmallThan25m),
            2 => Some(Maxspd::SmallThan33m),
            3 => Some(Maxspd::SmallThan50m),
            4 => Some(Maxspd::SmallThan66m),
            _ => None,
        }
    }
    #[doc = "<=20MHz"]
    #[inline(always)]
    pub fn is_small_than_20m(&self) -> bool {
        *self == Maxspd::SmallThan20m
    }
    #[doc = "<=25MHz (may be 24Mhz)"]
    #[inline(always)]
    pub fn is_small_than_25m(&self) -> bool {
        *self == Maxspd::SmallThan25m
    }
    #[doc = "<=33MHz (may be 30MHz)"]
    #[inline(always)]
    pub fn is_small_than_33m(&self) -> bool {
        *self == Maxspd::SmallThan33m
    }
    #[doc = "<=50MHz (may be 48MHz)"]
    #[inline(always)]
    pub fn is_small_than_50m(&self) -> bool {
        *self == Maxspd::SmallThan50m
    }
    #[doc = "<= 66MHz (may be 60MHz)"]
    #[inline(always)]
    pub fn is_small_than_66m(&self) -> bool {
        *self == Maxspd::SmallThan66m
    }
}
#[doc = "Field `MAXSPD` writer - Maximum SPI Clock speed to allow (host still chooses):"]
pub type MaxspdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Maxspd>;
impl<'a, REG> MaxspdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "<=20MHz"]
    #[inline(always)]
    pub fn small_than_20m(self) -> &'a mut crate::W<REG> {
        self.variant(Maxspd::SmallThan20m)
    }
    #[doc = "<=25MHz (may be 24Mhz)"]
    #[inline(always)]
    pub fn small_than_25m(self) -> &'a mut crate::W<REG> {
        self.variant(Maxspd::SmallThan25m)
    }
    #[doc = "<=33MHz (may be 30MHz)"]
    #[inline(always)]
    pub fn small_than_33m(self) -> &'a mut crate::W<REG> {
        self.variant(Maxspd::SmallThan33m)
    }
    #[doc = "<=50MHz (may be 48MHz)"]
    #[inline(always)]
    pub fn small_than_50m(self) -> &'a mut crate::W<REG> {
        self.variant(Maxspd::SmallThan50m)
    }
    #[doc = "<= 66MHz (may be 60MHz)"]
    #[inline(always)]
    pub fn small_than_66m(self) -> &'a mut crate::W<REG> {
        self.variant(Maxspd::SmallThan66m)
    }
}
#[doc = "Field `ALPIN` reader - Allow Alert to be a pin if the Host wants"]
pub type AlpinR = crate::BitReader;
#[doc = "Field `ALPIN` writer - Allow Alert to be a pin if the Host wants"]
pub type AlpinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOBOK` reader - If 1, allow OOB"]
pub type OobokR = crate::BitReader;
#[doc = "Field `OOBOK` writer - If 1, allow OOB"]
pub type OobokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMMX` reader - If 1, allow 128 byte payload for memory and OOB access, else limit to 64"]
pub type MemmxR = crate::BitReader;
#[doc = "Field `MEMMX` writer - If 1, allow 128 byte payload for memory and OOB access, else limit to 64"]
pub type MemmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects max Flash payload size to allow:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashmx {
    #[doc = "0: 64 bytes"]
    Byte64 = 0,
    #[doc = "1: 128 bytes"]
    Byte128 = 1,
    #[doc = "2: 256 bytes"]
    Byte256 = 2,
    #[doc = "3: 512 bytes"]
    Byte512 = 3,
}
impl From<Flashmx> for u8 {
    #[inline(always)]
    fn from(variant: Flashmx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashmx {
    type Ux = u8;
}
impl crate::IsEnum for Flashmx {}
#[doc = "Field `FLASHMX` reader - Selects max Flash payload size to allow:"]
pub type FlashmxR = crate::FieldReader<Flashmx>;
impl FlashmxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashmx {
        match self.bits {
            0 => Flashmx::Byte64,
            1 => Flashmx::Byte128,
            2 => Flashmx::Byte256,
            3 => Flashmx::Byte512,
            _ => unreachable!(),
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_byte64(&self) -> bool {
        *self == Flashmx::Byte64
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_byte128(&self) -> bool {
        *self == Flashmx::Byte128
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_byte256(&self) -> bool {
        *self == Flashmx::Byte256
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_byte512(&self) -> bool {
        *self == Flashmx::Byte512
    }
}
#[doc = "Field `FLASHMX` writer - Selects max Flash payload size to allow:"]
pub type FlashmxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flashmx, crate::Safe>;
impl<'a, REG> FlashmxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn byte64(self) -> &'a mut crate::W<REG> {
        self.variant(Flashmx::Byte64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn byte128(self) -> &'a mut crate::W<REG> {
        self.variant(Flashmx::Byte128)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn byte256(self) -> &'a mut crate::W<REG> {
        self.variant(Flashmx::Byte256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn byte512(self) -> &'a mut crate::W<REG> {
        self.variant(Flashmx::Byte512)
    }
}
#[doc = "Field `SAF` reader - If 1, then Slave Attached Flash is possible with this firmware"]
pub type SafR = crate::BitReader;
#[doc = "Field `SAF` writer - If 1, then Slave Attached Flash is possible with this firmware"]
pub type SafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Min Erase sector size allowed if SAF used\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Safera {
    #[doc = "0: Minimally 2KB"]
    Min2kb = 0,
    #[doc = "1: Minimally 4KB"]
    Min4kb = 1,
    #[doc = "2: Minimally 8KB"]
    Min8kb = 2,
    #[doc = "3: Minimally 16KB"]
    Min16kb = 3,
}
impl From<Safera> for u8 {
    #[inline(always)]
    fn from(variant: Safera) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Safera {
    type Ux = u8;
}
impl crate::IsEnum for Safera {}
#[doc = "Field `SAFERA` reader - Min Erase sector size allowed if SAF used"]
pub type SaferaR = crate::FieldReader<Safera>;
impl SaferaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Safera {
        match self.bits {
            0 => Safera::Min2kb,
            1 => Safera::Min4kb,
            2 => Safera::Min8kb,
            3 => Safera::Min16kb,
            _ => unreachable!(),
        }
    }
    #[doc = "Minimally 2KB"]
    #[inline(always)]
    pub fn is_min_2kb(&self) -> bool {
        *self == Safera::Min2kb
    }
    #[doc = "Minimally 4KB"]
    #[inline(always)]
    pub fn is_min_4kb(&self) -> bool {
        *self == Safera::Min4kb
    }
    #[doc = "Minimally 8KB"]
    #[inline(always)]
    pub fn is_min_8kb(&self) -> bool {
        *self == Safera::Min8kb
    }
    #[doc = "Minimally 16KB"]
    #[inline(always)]
    pub fn is_min_16kb(&self) -> bool {
        *self == Safera::Min16kb
    }
}
#[doc = "Field `SAFERA` writer - Min Erase sector size allowed if SAF used"]
pub type SaferaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Safera, crate::Safe>;
impl<'a, REG> SaferaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimally 2KB"]
    #[inline(always)]
    pub fn min_2kb(self) -> &'a mut crate::W<REG> {
        self.variant(Safera::Min2kb)
    }
    #[doc = "Minimally 4KB"]
    #[inline(always)]
    pub fn min_4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Safera::Min4kb)
    }
    #[doc = "Minimally 8KB"]
    #[inline(always)]
    pub fn min_8kb(self) -> &'a mut crate::W<REG> {
        self.variant(Safera::Min8kb)
    }
    #[doc = "Minimally 16KB"]
    #[inline(always)]
    pub fn min_16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Safera::Min16kb)
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI mode allowed (host still has to select):"]
    #[inline(always)]
    pub fn spicap(&self) -> SpicapR {
        SpicapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Maximum SPI Clock speed to allow (host still chooses):"]
    #[inline(always)]
    pub fn maxspd(&self) -> MaxspdR {
        MaxspdR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Allow Alert to be a pin if the Host wants"]
    #[inline(always)]
    pub fn alpin(&self) -> AlpinR {
        AlpinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If 1, allow OOB"]
    #[inline(always)]
    pub fn oobok(&self) -> OobokR {
        OobokR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If 1, allow 128 byte payload for memory and OOB access, else limit to 64"]
    #[inline(always)]
    pub fn memmx(&self) -> MemmxR {
        MemmxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Selects max Flash payload size to allow:"]
    #[inline(always)]
    pub fn flashmx(&self) -> FlashmxR {
        FlashmxR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - If 1, then Slave Attached Flash is possible with this firmware"]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Min Erase sector size allowed if SAF used"]
    #[inline(always)]
    pub fn safera(&self) -> SaferaR {
        SaferaR::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPICAP")
            .field("spicap", &self.spicap())
            .field("maxspd", &self.maxspd())
            .field("alpin", &self.alpin())
            .field("oobok", &self.oobok())
            .field("memmx", &self.memmx())
            .field("flashmx", &self.flashmx())
            .field("saf", &self.saf())
            .field("safera", &self.safera())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI mode allowed (host still has to select):"]
    #[inline(always)]
    pub fn spicap(&mut self) -> SpicapW<EspicapSpec> {
        SpicapW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Maximum SPI Clock speed to allow (host still chooses):"]
    #[inline(always)]
    pub fn maxspd(&mut self) -> MaxspdW<EspicapSpec> {
        MaxspdW::new(self, 4)
    }
    #[doc = "Bit 7 - Allow Alert to be a pin if the Host wants"]
    #[inline(always)]
    pub fn alpin(&mut self) -> AlpinW<EspicapSpec> {
        AlpinW::new(self, 7)
    }
    #[doc = "Bit 8 - If 1, allow OOB"]
    #[inline(always)]
    pub fn oobok(&mut self) -> OobokW<EspicapSpec> {
        OobokW::new(self, 8)
    }
    #[doc = "Bit 9 - If 1, allow 128 byte payload for memory and OOB access, else limit to 64"]
    #[inline(always)]
    pub fn memmx(&mut self) -> MemmxW<EspicapSpec> {
        MemmxW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Selects max Flash payload size to allow:"]
    #[inline(always)]
    pub fn flashmx(&mut self) -> FlashmxW<EspicapSpec> {
        FlashmxW::new(self, 10)
    }
    #[doc = "Bit 12 - If 1, then Slave Attached Flash is possible with this firmware"]
    #[inline(always)]
    pub fn saf(&mut self) -> SafW<EspicapSpec> {
        SafW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Min Erase sector size allowed if SAF used"]
    #[inline(always)]
    pub fn safera(&mut self) -> SaferaW<EspicapSpec> {
        SaferaW::new(self, 13)
    }
}
#[doc = "eSPI Capabilities of MCU to send to Host\n\nYou can [`read`](crate::Reg::read) this register and get [`espicap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espicap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspicapSpec;
impl crate::RegisterSpec for EspicapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espicap::R`](R) reader structure"]
impl crate::Readable for EspicapSpec {}
#[doc = "`write(|w| ..)` method takes [`espicap::W`](W) writer structure"]
impl crate::Writable for EspicapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESPICAP to value 0"]
impl crate::Resettable for EspicapSpec {
    const RESET_VALUE: u32 = 0;
}
