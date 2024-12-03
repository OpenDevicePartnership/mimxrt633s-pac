#[doc = "Register `P0STAT` reader"]
pub type R = crate::R<P0statSpec>;
#[doc = "Register `P0STAT` writer"]
pub type W = crate::W<P0statSpec>;
#[doc = "Field `RDSTAT` reader - Status of Host Read data"]
pub type RdstatR = crate::FieldReader;
#[doc = "Field `WRSTAT` reader - Status of Host Writes"]
pub type WrstatR = crate::FieldReader;
#[doc = "Field `INTERR` reader - Interrupt was caused by error"]
pub type InterrR = crate::BitReader;
#[doc = "Field `INTERR` writer - Interrupt was caused by error"]
pub type InterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRD` reader - Interrupt was caused by Read or 1st Read or Bus master Started."]
pub type IntrdR = crate::BitReader;
#[doc = "Field `INTRD` writer - Interrupt was caused by Read or 1st Read or Bus master Started."]
pub type IntrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTWR` reader - Interrupt was caused by Write or 1st Write or Bus master Started."]
pub type IntwrR = crate::BitReader;
#[doc = "Field `INTWR` writer - Interrupt was caused by Write or 1st Write or Bus master Started."]
pub type IntwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPC` reader - Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type IntspcR = crate::FieldReader;
#[doc = "Field `INTSPC` writer - Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type IntspcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 11:14 - Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc(&self) -> IntspcR {
        IntspcR::new(((self.bits >> 11) & 0x0f) as u8)
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
        f.debug_struct("P0STAT")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("err", &self.err())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Interrupt was caused by error"]
    #[inline(always)]
    pub fn interr(&mut self) -> InterrW<P0statSpec> {
        InterrW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub fn intrd(&mut self) -> IntrdW<P0statSpec> {
        IntrdW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub fn intwr(&mut self) -> IntwrW<P0statSpec> {
        IntwrW::new(self, 10)
    }
    #[doc = "Bits 11:14 - Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc(&mut self) -> IntspcW<P0statSpec> {
        IntspcW::new(self, 11)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`p0stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0statSpec;
impl crate::RegisterSpec for P0statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0stat::R`](R) reader structure"]
impl crate::Readable for P0statSpec {}
#[doc = "`write(|w| ..)` method takes [`p0stat::W`](W) writer structure"]
impl crate::Writable for P0statSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P0STAT to value 0"]
impl crate::Resettable for P0statSpec {
    const RESET_VALUE: u32 = 0;
}
