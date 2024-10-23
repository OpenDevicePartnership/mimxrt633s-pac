#[doc = "Register `FORCE_EVENT` reader"]
pub type R = crate::R<ForceEventSpec>;
#[doc = "Register `FORCE_EVENT` writer"]
pub type W = crate::W<ForceEventSpec>;
#[doc = "Field `FEVTAC12NE` reader - Force Event Auto Command 12 Not Executed"]
pub type Fevtac12neR = crate::BitReader;
#[doc = "Field `FEVTAC12NE` writer - Force Event Auto Command 12 Not Executed"]
pub type Fevtac12neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTAC12TOE` reader - Force Event Auto Command 12 Time Out Error"]
pub type Fevtac12toeR = crate::BitReader;
#[doc = "Field `FEVTAC12TOE` writer - Force Event Auto Command 12 Time Out Error"]
pub type Fevtac12toeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTAC12CE` reader - Force Event Auto Command 12 CRC Error"]
pub type Fevtac12ceR = crate::BitReader;
#[doc = "Field `FEVTAC12CE` writer - Force Event Auto Command 12 CRC Error"]
pub type Fevtac12ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTAC12EBE` reader - Force Event Auto Command 12 End Bit Error"]
pub type Fevtac12ebeR = crate::BitReader;
#[doc = "Field `FEVTAC12EBE` writer - Force Event Auto Command 12 End Bit Error"]
pub type Fevtac12ebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTAC12IE` reader - Force Event Auto Command 12 Index Error"]
pub type Fevtac12ieR = crate::BitReader;
#[doc = "Field `FEVTAC12IE` writer - Force Event Auto Command 12 Index Error"]
pub type Fevtac12ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTCNIBAC12E` reader - Force Event Command Not Executed By Auto Command 12 Error"]
pub type Fevtcnibac12eR = crate::BitReader;
#[doc = "Field `FEVTCNIBAC12E` writer - Force Event Command Not Executed By Auto Command 12 Error"]
pub type Fevtcnibac12eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTCTOE` reader - Force Event Command Time Out Error"]
pub type FevtctoeR = crate::BitReader;
#[doc = "Field `FEVTCTOE` writer - Force Event Command Time Out Error"]
pub type FevtctoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTCCE` reader - Force Event Command CRC Error"]
pub type FevtcceR = crate::BitReader;
#[doc = "Field `FEVTCCE` writer - Force Event Command CRC Error"]
pub type FevtcceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTCEBE` reader - Force Event Command End Bit Error"]
pub type FevtcebeR = crate::BitReader;
#[doc = "Field `FEVTCEBE` writer - Force Event Command End Bit Error"]
pub type FevtcebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTCIE` reader - Force Event Command Index Error"]
pub type FevtcieR = crate::BitReader;
#[doc = "Field `FEVTCIE` writer - Force Event Command Index Error"]
pub type FevtcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTDTOE` reader - Force Event Data Time Out Error"]
pub type FevtdtoeR = crate::BitReader;
#[doc = "Field `FEVTDTOE` writer - Force Event Data Time Out Error"]
pub type FevtdtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTDCE` reader - Force Event Data CRC Error"]
pub type FevtdceR = crate::BitReader;
#[doc = "Field `FEVTDCE` writer - Force Event Data CRC Error"]
pub type FevtdceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTDEBE` reader - Force Event Data End Bit Error"]
pub type FevtdebeR = crate::BitReader;
#[doc = "Field `FEVTDEBE` writer - Force Event Data End Bit Error"]
pub type FevtdebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTAC12E` reader - Force Event Auto Command 12 Error"]
pub type Fevtac12eR = crate::BitReader;
#[doc = "Field `FEVTAC12E` writer - Force Event Auto Command 12 Error"]
pub type Fevtac12eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTTNE` reader - Force Tuning Error"]
pub type FevttneR = crate::BitReader;
#[doc = "Field `FEVTTNE` writer - Force Tuning Error"]
pub type FevttneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTDMAE` reader - Force Event DMA Error"]
pub type FevtdmaeR = crate::BitReader;
#[doc = "Field `FEVTDMAE` writer - Force Event DMA Error"]
pub type FevtdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEVTCINT` reader - Force Event Card Interrupt"]
pub type FevtcintR = crate::BitReader;
#[doc = "Field `FEVTCINT` writer - Force Event Card Interrupt"]
pub type FevtcintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    pub fn fevtac12ne(&self) -> Fevtac12neR {
        Fevtac12neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    pub fn fevtac12toe(&self) -> Fevtac12toeR {
        Fevtac12toeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    pub fn fevtac12ce(&self) -> Fevtac12ceR {
        Fevtac12ceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    pub fn fevtac12ebe(&self) -> Fevtac12ebeR {
        Fevtac12ebeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    pub fn fevtac12ie(&self) -> Fevtac12ieR {
        Fevtac12ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    pub fn fevtcnibac12e(&self) -> Fevtcnibac12eR {
        Fevtcnibac12eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Force Event Command Time Out Error"]
    #[inline(always)]
    pub fn fevtctoe(&self) -> FevtctoeR {
        FevtctoeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force Event Command CRC Error"]
    #[inline(always)]
    pub fn fevtcce(&self) -> FevtcceR {
        FevtcceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force Event Command End Bit Error"]
    #[inline(always)]
    pub fn fevtcebe(&self) -> FevtcebeR {
        FevtcebeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force Event Command Index Error"]
    #[inline(always)]
    pub fn fevtcie(&self) -> FevtcieR {
        FevtcieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force Event Data Time Out Error"]
    #[inline(always)]
    pub fn fevtdtoe(&self) -> FevtdtoeR {
        FevtdtoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Force Event Data CRC Error"]
    #[inline(always)]
    pub fn fevtdce(&self) -> FevtdceR {
        FevtdceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Force Event Data End Bit Error"]
    #[inline(always)]
    pub fn fevtdebe(&self) -> FevtdebeR {
        FevtdebeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Force Event Auto Command 12 Error"]
    #[inline(always)]
    pub fn fevtac12e(&self) -> Fevtac12eR {
        Fevtac12eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Force Tuning Error"]
    #[inline(always)]
    pub fn fevttne(&self) -> FevttneR {
        FevttneR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Force Event DMA Error"]
    #[inline(always)]
    pub fn fevtdmae(&self) -> FevtdmaeR {
        FevtdmaeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Force Event Card Interrupt"]
    #[inline(always)]
    pub fn fevtcint(&self) -> FevtcintR {
        FevtcintR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FORCE_EVENT")
            .field("fevtac12ne", &self.fevtac12ne())
            .field("fevtac12toe", &self.fevtac12toe())
            .field("fevtac12ce", &self.fevtac12ce())
            .field("fevtac12ebe", &self.fevtac12ebe())
            .field("fevtac12ie", &self.fevtac12ie())
            .field("fevtcnibac12e", &self.fevtcnibac12e())
            .field("fevtctoe", &self.fevtctoe())
            .field("fevtcce", &self.fevtcce())
            .field("fevtcebe", &self.fevtcebe())
            .field("fevtcie", &self.fevtcie())
            .field("fevtdtoe", &self.fevtdtoe())
            .field("fevtdce", &self.fevtdce())
            .field("fevtdebe", &self.fevtdebe())
            .field("fevtac12e", &self.fevtac12e())
            .field("fevttne", &self.fevttne())
            .field("fevtdmae", &self.fevtdmae())
            .field("fevtcint", &self.fevtcint())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ne(&mut self) -> Fevtac12neW<ForceEventSpec> {
        Fevtac12neW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12toe(&mut self) -> Fevtac12toeW<ForceEventSpec> {
        Fevtac12toeW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ce(&mut self) -> Fevtac12ceW<ForceEventSpec> {
        Fevtac12ceW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ebe(&mut self) -> Fevtac12ebeW<ForceEventSpec> {
        Fevtac12ebeW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12ie(&mut self) -> Fevtac12ieW<ForceEventSpec> {
        Fevtac12ieW::new(self, 4)
    }
    #[doc = "Bit 7 - Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcnibac12e(&mut self) -> Fevtcnibac12eW<ForceEventSpec> {
        Fevtcnibac12eW::new(self, 7)
    }
    #[doc = "Bit 16 - Force Event Command Time Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtctoe(&mut self) -> FevtctoeW<ForceEventSpec> {
        FevtctoeW::new(self, 16)
    }
    #[doc = "Bit 17 - Force Event Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcce(&mut self) -> FevtcceW<ForceEventSpec> {
        FevtcceW::new(self, 17)
    }
    #[doc = "Bit 18 - Force Event Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcebe(&mut self) -> FevtcebeW<ForceEventSpec> {
        FevtcebeW::new(self, 18)
    }
    #[doc = "Bit 19 - Force Event Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcie(&mut self) -> FevtcieW<ForceEventSpec> {
        FevtcieW::new(self, 19)
    }
    #[doc = "Bit 20 - Force Event Data Time Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdtoe(&mut self) -> FevtdtoeW<ForceEventSpec> {
        FevtdtoeW::new(self, 20)
    }
    #[doc = "Bit 21 - Force Event Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdce(&mut self) -> FevtdceW<ForceEventSpec> {
        FevtdceW::new(self, 21)
    }
    #[doc = "Bit 22 - Force Event Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdebe(&mut self) -> FevtdebeW<ForceEventSpec> {
        FevtdebeW::new(self, 22)
    }
    #[doc = "Bit 24 - Force Event Auto Command 12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtac12e(&mut self) -> Fevtac12eW<ForceEventSpec> {
        Fevtac12eW::new(self, 24)
    }
    #[doc = "Bit 26 - Force Tuning Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevttne(&mut self) -> FevttneW<ForceEventSpec> {
        FevttneW::new(self, 26)
    }
    #[doc = "Bit 28 - Force Event DMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn fevtdmae(&mut self) -> FevtdmaeW<ForceEventSpec> {
        FevtdmaeW::new(self, 28)
    }
    #[doc = "Bit 31 - Force Event Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fevtcint(&mut self) -> FevtcintW<ForceEventSpec> {
        FevtcintW::new(self, 31)
    }
}
#[doc = "Force Event\n\nYou can [`read`](crate::Reg::read) this register and get [`force_event::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_event::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceEventSpec;
impl crate::RegisterSpec for ForceEventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`force_event::R`](R) reader structure"]
impl crate::Readable for ForceEventSpec {}
#[doc = "`write(|w| ..)` method takes [`force_event::W`](W) writer structure"]
impl crate::Writable for ForceEventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FORCE_EVENT to value 0"]
impl crate::Resettable for ForceEventSpec {
    const RESET_VALUE: u32 = 0;
}
