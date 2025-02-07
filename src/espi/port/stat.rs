#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `RDSTAT` reader - Status of Host Read data"]
pub type RdstatR = crate::FieldReader;
#[doc = "Field `WRSTAT` reader - Status of Host Writes"]
pub type WrstatR = crate::FieldReader;
#[doc = "Field `INTERR` reader - Interrupt was caused by error"]
pub type InterrR = crate::BitReader;
#[doc = "Field `INTERR` writer - Interrupt was caused by error"]
pub type InterrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INTRD` reader - Interrupt was caused by Read or 1st Read or Bus master Started."]
pub type IntrdR = crate::BitReader;
#[doc = "Field `INTRD` writer - Interrupt was caused by Read or 1st Read or Bus master Started."]
pub type IntrdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INTWR` reader - Interrupt was caused by Write or 1st Write or Bus master Started."]
pub type IntwrR = crate::BitReader;
#[doc = "Field `INTWR` writer - Interrupt was caused by Write or 1st Write or Bus master Started."]
pub type IntwrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INTSPC0` reader - "]
pub type Intspc0R = crate::BitReader;
#[doc = "Field `INTSPC0` writer - "]
pub type Intspc0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INTSPC1` reader - "]
pub type Intspc1R = crate::BitReader;
#[doc = "Field `INTSPC1` writer - "]
pub type Intspc1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INTSPC2` reader - "]
pub type Intspc2R = crate::BitReader;
#[doc = "Field `INTSPC2` writer - "]
pub type Intspc2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INTSPC3` reader - "]
pub type Intspc3R = crate::BitReader;
#[doc = "Field `INTSPC3` writer - "]
pub type Intspc3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERR` reader - Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
pub type ErrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Status of Host Read data"]
    #[inline(always)]
    pub fn rdstat(&self) -> RdstatR {
        RdstatR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Status of Host Writes"]
    #[inline(always)]
    pub fn wrstat(&self) -> WrstatR {
        WrstatR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt was caused by error"]
    #[inline(always)]
    pub fn interr(&self) -> InterrR {
        InterrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub fn intrd(&self) -> IntrdR {
        IntrdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub fn intwr(&self) -> IntwrR {
        IntwrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn intspc0(&self) -> Intspc0R {
        Intspc0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn intspc1(&self) -> Intspc1R {
        Intspc1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn intspc2(&self) -> Intspc2R {
        Intspc2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn intspc3(&self) -> Intspc3R {
        Intspc3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("err", &self.err())
            .field("intspc0", &self.intspc0())
            .field("intspc1", &self.intspc1())
            .field("intspc2", &self.intspc2())
            .field("intspc3", &self.intspc3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Interrupt was caused by error"]
    #[inline(always)]
    pub fn interr(&mut self) -> InterrW<StatSpec> {
        InterrW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub fn intrd(&mut self) -> IntrdW<StatSpec> {
        IntrdW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub fn intwr(&mut self) -> IntwrW<StatSpec> {
        IntwrW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn intspc0(&mut self) -> Intspc0W<StatSpec> {
        Intspc0W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn intspc1(&mut self) -> Intspc1W<StatSpec> {
        Intspc1W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn intspc2(&mut self) -> Intspc2W<StatSpec> {
        Intspc2W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn intspc3(&mut self) -> Intspc3W<StatSpec> {
        Intspc3W::new(self, 14)
    }
}
#[doc = "Port Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f00;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
