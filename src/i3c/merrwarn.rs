#[doc = "Register `MERRWARN` reader"]
pub type R = crate::R<MerrwarnSpec>;
#[doc = "Register `MERRWARN` writer"]
pub type W = crate::W<MerrwarnSpec>;
#[doc = "Field `NACK` reader - Not acknowledge (NACK) error"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - Not acknowledge (NACK) error"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRABT` reader - WRABT (Write abort) error"]
pub type WrabtR = crate::BitReader;
#[doc = "Field `WRABT` writer - WRABT (Write abort) error"]
pub type WrabtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERM` reader - Terminate error"]
pub type TermR = crate::BitReader;
#[doc = "Field `TERM` writer - Terminate error"]
pub type TermW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPAR` reader - High data rate parity"]
pub type HparR = crate::BitReader;
#[doc = "Field `HPAR` writer - High data rate parity"]
pub type HparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCRC` reader - High data rate CRC error"]
pub type HcrcR = crate::BitReader;
#[doc = "Field `HCRC` writer - High data rate CRC error"]
pub type HcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OREAD` reader - Over-read error"]
pub type OreadR = crate::BitReader;
#[doc = "Field `OREAD` writer - Over-read error"]
pub type OreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWRITE` reader - Over-write error"]
pub type OwriteR = crate::BitReader;
#[doc = "Field `OWRITE` writer - Over-write error"]
pub type OwriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSGERR` reader - Message error"]
pub type MsgerrR = crate::BitReader;
#[doc = "Field `MSGERR` writer - Message error"]
pub type MsgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVREQ` reader - Invalid request error"]
pub type InvreqR = crate::BitReader;
#[doc = "Field `INVREQ` writer - Invalid request error"]
pub type InvreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - TIMEOUT error"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - TIMEOUT error"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Not acknowledge (NACK) error"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WRABT (Write abort) error"]
    #[inline(always)]
    pub fn wrabt(&self) -> WrabtR {
        WrabtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Terminate error"]
    #[inline(always)]
    pub fn term(&self) -> TermR {
        TermR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - High data rate parity"]
    #[inline(always)]
    pub fn hpar(&self) -> HparR {
        HparR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High data rate CRC error"]
    #[inline(always)]
    pub fn hcrc(&self) -> HcrcR {
        HcrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Over-read error"]
    #[inline(always)]
    pub fn oread(&self) -> OreadR {
        OreadR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Over-write error"]
    #[inline(always)]
    pub fn owrite(&self) -> OwriteR {
        OwriteR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Message error"]
    #[inline(always)]
    pub fn msgerr(&self) -> MsgerrR {
        MsgerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Invalid request error"]
    #[inline(always)]
    pub fn invreq(&self) -> InvreqR {
        InvreqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMEOUT error"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MERRWARN")
            .field("nack", &self.nack())
            .field("wrabt", &self.wrabt())
            .field("term", &self.term())
            .field("hpar", &self.hpar())
            .field("hcrc", &self.hcrc())
            .field("oread", &self.oread())
            .field("owrite", &self.owrite())
            .field("msgerr", &self.msgerr())
            .field("invreq", &self.invreq())
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Not acknowledge (NACK) error"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<MerrwarnSpec> {
        NackW::new(self, 2)
    }
    #[doc = "Bit 3 - WRABT (Write abort) error"]
    #[inline(always)]
    pub fn wrabt(&mut self) -> WrabtW<MerrwarnSpec> {
        WrabtW::new(self, 3)
    }
    #[doc = "Bit 4 - Terminate error"]
    #[inline(always)]
    pub fn term(&mut self) -> TermW<MerrwarnSpec> {
        TermW::new(self, 4)
    }
    #[doc = "Bit 9 - High data rate parity"]
    #[inline(always)]
    pub fn hpar(&mut self) -> HparW<MerrwarnSpec> {
        HparW::new(self, 9)
    }
    #[doc = "Bit 10 - High data rate CRC error"]
    #[inline(always)]
    pub fn hcrc(&mut self) -> HcrcW<MerrwarnSpec> {
        HcrcW::new(self, 10)
    }
    #[doc = "Bit 16 - Over-read error"]
    #[inline(always)]
    pub fn oread(&mut self) -> OreadW<MerrwarnSpec> {
        OreadW::new(self, 16)
    }
    #[doc = "Bit 17 - Over-write error"]
    #[inline(always)]
    pub fn owrite(&mut self) -> OwriteW<MerrwarnSpec> {
        OwriteW::new(self, 17)
    }
    #[doc = "Bit 18 - Message error"]
    #[inline(always)]
    pub fn msgerr(&mut self) -> MsgerrW<MerrwarnSpec> {
        MsgerrW::new(self, 18)
    }
    #[doc = "Bit 19 - Invalid request error"]
    #[inline(always)]
    pub fn invreq(&mut self) -> InvreqW<MerrwarnSpec> {
        InvreqW::new(self, 19)
    }
    #[doc = "Bit 20 - TIMEOUT error"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<MerrwarnSpec> {
        TimeoutW::new(self, 20)
    }
}
#[doc = "Master Errors and Warnings Register\n\nYou can [`read`](crate::Reg::read) this register and get [`merrwarn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`merrwarn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MerrwarnSpec;
impl crate::RegisterSpec for MerrwarnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`merrwarn::R`](R) reader structure"]
impl crate::Readable for MerrwarnSpec {}
#[doc = "`write(|w| ..)` method takes [`merrwarn::W`](W) writer structure"]
impl crate::Writable for MerrwarnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MERRWARN to value 0"]
impl crate::Resettable for MerrwarnSpec {
    const RESET_VALUE: u32 = 0;
}
