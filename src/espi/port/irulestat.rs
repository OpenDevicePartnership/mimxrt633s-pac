#[doc = "Register `IRULESTAT` reader"]
pub type R = crate::R<IrulestatSpec>;
#[doc = "Register `IRULESTAT` writer"]
pub type W = crate::W<IrulestatSpec>;
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
        f.debug_struct("IRULESTAT")
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
    pub fn ustat(&mut self) -> UstatW<IrulestatSpec> {
        UstatW::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    pub fn interr(&mut self) -> InterrW<IrulestatSpec> {
        InterrW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub fn intrd(&mut self) -> IntrdW<IrulestatSpec> {
        IntrdW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    pub fn intwr(&mut self) -> IntwrW<IrulestatSpec> {
        IntwrW::new(self, 10)
    }
    #[doc = "Bits 11:14 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc(&mut self) -> IntspcW<IrulestatSpec> {
        IntspcW::new(self, 11)
    }
    #[doc = "Bits 16:19 - Status set/clear"]
    #[inline(always)]
    pub fn sstcl(&mut self) -> SstclW<IrulestatSpec> {
        SstclW::new(self, 16)
    }
    #[doc = "Bit 20 - Resets the RdStatus and WrStatus in PStatus register"]
    #[inline(always)]
    pub fn srst(&mut self) -> SrstW<IrulestatSpec> {
        SrstW::new(self, 20)
    }
}
#[doc = "Port set interrupt rules and user status\n\nYou can [`read`](crate::Reg::read) this register and get [`irulestat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irulestat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrulestatSpec;
impl crate::RegisterSpec for IrulestatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irulestat::R`](R) reader structure"]
impl crate::Readable for IrulestatSpec {}
#[doc = "`write(|w| ..)` method takes [`irulestat::W`](W) writer structure"]
impl crate::Writable for IrulestatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRULESTAT to value 0"]
impl crate::Resettable for IrulestatSpec {
    const RESET_VALUE: u32 = 0;
}
