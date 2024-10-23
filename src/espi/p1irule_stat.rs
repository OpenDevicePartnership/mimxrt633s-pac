#[doc = "Register `P1IRuleStat` reader"]
pub type R = crate::R<P1iruleStatSpec>;
#[doc = "Register `P1IRuleStat` writer"]
pub type W = crate::W<P1iruleStatSpec>;
#[doc = "Field `USTAT` reader - User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
pub type UstatR = crate::FieldReader;
#[doc = "Field `USTAT` writer - User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
pub type UstatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INTERR` reader - Interrupt if an error is detected (classes of error defined in section 2.14)"]
pub type InterrR = crate::BitReader;
#[doc = "Field `INTERR` writer - Interrupt if an error is detected (classes of error defined in section 2.14)"]
pub type InterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRD` reader - Interrupt if Read or 1st Read or Bus master Started."]
pub type IntrdR = crate::BitReader;
#[doc = "Field `INTRD` writer - Interrupt if Read or 1st Read or Bus master Started."]
pub type IntrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTWR` reader - Interrupt if Write or 1st Write or Bus master Finished."]
pub type IntwrR = crate::BitReader;
#[doc = "Field `INTWR` writer - Interrupt if Write or 1st Write or Bus master Finished."]
pub type IntwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPC` reader - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type IntspcR = crate::FieldReader;
#[doc = "Field `INTSPC` writer - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type IntspcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SSTCL` writer - Status set/clear"]
pub type SstclW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRST` writer - Resets the RdStatus and WrStatus in PStatus register"]
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[inline(always)]
    pub fn ustat(&self) -> UstatR {
        UstatR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    pub fn interr(&self) -> InterrR {
        InterrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub fn intrd(&self) -> IntrdR {
        IntrdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    pub fn intwr(&self) -> IntwrR {
        IntwrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc(&self) -> IntspcR {
        IntspcR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1IRuleStat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[inline(always)]
    #[must_use]
    pub fn ustat(&mut self) -> UstatW<P1iruleStatSpec> {
        UstatW::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    #[must_use]
    pub fn interr(&mut self) -> InterrW<P1iruleStatSpec> {
        InterrW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    #[must_use]
    pub fn intrd(&mut self) -> IntrdW<P1iruleStatSpec> {
        IntrdW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    #[must_use]
    pub fn intwr(&mut self) -> IntwrW<P1iruleStatSpec> {
        IntwrW::new(self, 10)
    }
    #[doc = "Bits 11:14 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    #[must_use]
    pub fn intspc(&mut self) -> IntspcW<P1iruleStatSpec> {
        IntspcW::new(self, 11)
    }
    #[doc = "Bits 16:19 - Status set/clear"]
    #[inline(always)]
    #[must_use]
    pub fn sstcl(&mut self) -> SstclW<P1iruleStatSpec> {
        SstclW::new(self, 16)
    }
    #[doc = "Bit 20 - Resets the RdStatus and WrStatus in PStatus register"]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SrstW<P1iruleStatSpec> {
        SrstW::new(self, 20)
    }
}
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET.\n\nYou can [`read`](crate::Reg::read) this register and get [`p1irule_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1irule_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1iruleStatSpec;
impl crate::RegisterSpec for P1iruleStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p1irule_stat::R`](R) reader structure"]
impl crate::Readable for P1iruleStatSpec {}
#[doc = "`write(|w| ..)` method takes [`p1irule_stat::W`](W) writer structure"]
impl crate::Writable for P1iruleStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P1IRuleStat to value 0"]
impl crate::Resettable for P1iruleStatSpec {
    const RESET_VALUE: u32 = 0;
}
