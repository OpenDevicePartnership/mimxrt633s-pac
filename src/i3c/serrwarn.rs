#[doc = "Register `SERRWARN` reader"]
pub type R = crate::R<SerrwarnSpec>;
#[doc = "Register `SERRWARN` writer"]
pub type W = crate::W<SerrwarnSpec>;
#[doc = "Field `ORUN` reader - Overrun error"]
pub type OrunR = crate::BitReader;
#[doc = "Field `ORUN` writer - Overrun error"]
pub type OrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URUN` reader - Underrun error"]
pub type UrunR = crate::BitReader;
#[doc = "Field `URUN` writer - Underrun error"]
pub type UrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URUNNACK` reader - Underrun and Not Acknowledged (NACKed) error"]
pub type UrunnackR = crate::BitReader;
#[doc = "Field `URUNNACK` writer - Underrun and Not Acknowledged (NACKed) error"]
pub type UrunnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERM` reader - Terminated error"]
pub type TermR = crate::BitReader;
#[doc = "Field `TERM` writer - Terminated error"]
pub type TermW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSTART` reader - Invalid start error"]
pub type InvstartR = crate::BitReader;
#[doc = "Field `INVSTART` writer - Invalid start error"]
pub type InvstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPAR` reader - SDR parity error"]
pub type SparR = crate::BitReader;
#[doc = "Field `SPAR` writer - SDR parity error"]
pub type SparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPAR` reader - HDR parity error"]
pub type HparR = crate::BitReader;
#[doc = "Field `HPAR` writer - HDR parity error"]
pub type HparW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCRC` reader - HDR-DDR CRC error"]
pub type HcrcR = crate::BitReader;
#[doc = "Field `HCRC` writer - HDR-DDR CRC error"]
pub type HcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0S1` reader - S0 or S1 error"]
pub type S0s1R = crate::BitReader;
#[doc = "Field `S0S1` writer - S0 or S1 error"]
pub type S0s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OREAD` reader - Over-read error"]
pub type OreadR = crate::BitReader;
#[doc = "Field `OREAD` writer - Over-read error"]
pub type OreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWRITE` reader - Over-write error"]
pub type OwriteR = crate::BitReader;
#[doc = "Field `OWRITE` writer - Over-write error"]
pub type OwriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn orun(&self) -> OrunR {
        OrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underrun error"]
    #[inline(always)]
    pub fn urun(&self) -> UrunR {
        UrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underrun and Not Acknowledged (NACKed) error"]
    #[inline(always)]
    pub fn urunnack(&self) -> UrunnackR {
        UrunnackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Terminated error"]
    #[inline(always)]
    pub fn term(&self) -> TermR {
        TermR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invalid start error"]
    #[inline(always)]
    pub fn invstart(&self) -> InvstartR {
        InvstartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SDR parity error"]
    #[inline(always)]
    pub fn spar(&self) -> SparR {
        SparR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HDR parity error"]
    #[inline(always)]
    pub fn hpar(&self) -> HparR {
        HparR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HDR-DDR CRC error"]
    #[inline(always)]
    pub fn hcrc(&self) -> HcrcR {
        HcrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S0 or S1 error"]
    #[inline(always)]
    pub fn s0s1(&self) -> S0s1R {
        S0s1R::new(((self.bits >> 11) & 1) != 0)
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
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SERRWARN")
            .field("orun", &self.orun())
            .field("urun", &self.urun())
            .field("urunnack", &self.urunnack())
            .field("term", &self.term())
            .field("invstart", &self.invstart())
            .field("spar", &self.spar())
            .field("hpar", &self.hpar())
            .field("hcrc", &self.hcrc())
            .field("s0s1", &self.s0s1())
            .field("oread", &self.oread())
            .field("owrite", &self.owrite())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn orun(&mut self) -> OrunW<SerrwarnSpec> {
        OrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Underrun error"]
    #[inline(always)]
    pub fn urun(&mut self) -> UrunW<SerrwarnSpec> {
        UrunW::new(self, 1)
    }
    #[doc = "Bit 2 - Underrun and Not Acknowledged (NACKed) error"]
    #[inline(always)]
    pub fn urunnack(&mut self) -> UrunnackW<SerrwarnSpec> {
        UrunnackW::new(self, 2)
    }
    #[doc = "Bit 3 - Terminated error"]
    #[inline(always)]
    pub fn term(&mut self) -> TermW<SerrwarnSpec> {
        TermW::new(self, 3)
    }
    #[doc = "Bit 4 - Invalid start error"]
    #[inline(always)]
    pub fn invstart(&mut self) -> InvstartW<SerrwarnSpec> {
        InvstartW::new(self, 4)
    }
    #[doc = "Bit 8 - SDR parity error"]
    #[inline(always)]
    pub fn spar(&mut self) -> SparW<SerrwarnSpec> {
        SparW::new(self, 8)
    }
    #[doc = "Bit 9 - HDR parity error"]
    #[inline(always)]
    pub fn hpar(&mut self) -> HparW<SerrwarnSpec> {
        HparW::new(self, 9)
    }
    #[doc = "Bit 10 - HDR-DDR CRC error"]
    #[inline(always)]
    pub fn hcrc(&mut self) -> HcrcW<SerrwarnSpec> {
        HcrcW::new(self, 10)
    }
    #[doc = "Bit 11 - S0 or S1 error"]
    #[inline(always)]
    pub fn s0s1(&mut self) -> S0s1W<SerrwarnSpec> {
        S0s1W::new(self, 11)
    }
    #[doc = "Bit 16 - Over-read error"]
    #[inline(always)]
    pub fn oread(&mut self) -> OreadW<SerrwarnSpec> {
        OreadW::new(self, 16)
    }
    #[doc = "Bit 17 - Over-write error"]
    #[inline(always)]
    pub fn owrite(&mut self) -> OwriteW<SerrwarnSpec> {
        OwriteW::new(self, 17)
    }
}
#[doc = "Slave Errors and Warnings Register\n\nYou can [`read`](crate::Reg::read) this register and get [`serrwarn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serrwarn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerrwarnSpec;
impl crate::RegisterSpec for SerrwarnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`serrwarn::R`](R) reader structure"]
impl crate::Readable for SerrwarnSpec {}
#[doc = "`write(|w| ..)` method takes [`serrwarn::W`](W) writer structure"]
impl crate::Writable for SerrwarnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SERRWARN to value 0"]
impl crate::Resettable for SerrwarnSpec {
    const RESET_VALUE: u32 = 0;
}
