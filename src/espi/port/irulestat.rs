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
#[doc = "Field `INTSPC0` reader - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc0R = crate::BitReader;
#[doc = "Field `INTSPC0` writer - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPC1` reader - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc1R = crate::BitReader;
#[doc = "Field `INTSPC1` writer - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPC2` reader - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc2R = crate::BitReader;
#[doc = "Field `INTSPC2` writer - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPC3` reader - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc3R = crate::BitReader;
#[doc = "Field `INTSPC3` writer - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
pub type Intspc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Status set/clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sstcl {
    #[doc = "1: Started by MCU"]
    Mcustart = 1,
    #[doc = "2: Completed by MCU"]
    Mcudone = 2,
    #[doc = "3: Mailbox is Empty"]
    Empty = 3,
}
impl From<Sstcl> for u8 {
    #[inline(always)]
    fn from(variant: Sstcl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sstcl {
    type Ux = u8;
}
impl crate::IsEnum for Sstcl {}
#[doc = "Field `SSTCL` writer - Status set/clear"]
pub type SstclW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sstcl>;
impl<'a, REG> SstclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Started by MCU"]
    #[inline(always)]
    pub fn mcustart(self) -> &'a mut crate::W<REG> {
        self.variant(Sstcl::Mcustart)
    }
    #[doc = "Completed by MCU"]
    #[inline(always)]
    pub fn mcudone(self) -> &'a mut crate::W<REG> {
        self.variant(Sstcl::Mcudone)
    }
    #[doc = "Mailbox is Empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Sstcl::Empty)
    }
}
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
    #[doc = "Bit 11 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc0(&self) -> Intspc0R {
        Intspc0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc1(&self) -> Intspc1R {
        Intspc1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc2(&self) -> Intspc2R {
        Intspc2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc3(&self) -> Intspc3R {
        Intspc3R::new(((self.bits >> 14) & 1) != 0)
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
            .field("intspc0", &self.intspc0())
            .field("intspc1", &self.intspc1())
            .field("intspc2", &self.intspc2())
            .field("intspc3", &self.intspc3())
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
    #[doc = "Bit 11 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc0(&mut self) -> Intspc0W<IrulestatSpec> {
        Intspc0W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc1(&mut self) -> Intspc1W<IrulestatSpec> {
        Intspc1W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc2(&mut self) -> Intspc2W<IrulestatSpec> {
        Intspc2W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub fn intspc3(&mut self) -> Intspc3W<IrulestatSpec> {
        Intspc3W::new(self, 14)
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
