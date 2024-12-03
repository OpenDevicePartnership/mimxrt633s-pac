#[doc = "Register `P0OMFLEN` reader"]
pub type R = crate::R<P0omflenSpec>;
#[doc = "Register `P0OMFLEN` writer"]
pub type W = crate::W<P0omflenSpec>;
#[doc = "Field `LEN` reader - Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TRANS` reader - Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
pub type TransR = crate::FieldReader;
#[doc = "Field `TRANS` writer - Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
pub type TransW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 12:13 - Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub fn trans(&self) -> TransR {
        TransR::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0OMFLEN")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<P0omflenSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub fn trans(&mut self) -> TransW<P0omflenSpec> {
        TransW::new(self, 12)
    }
}
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`p0omflen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0omflen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0omflenSpec;
impl crate::RegisterSpec for P0omflenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0omflen::R`](R) reader structure"]
impl crate::Readable for P0omflenSpec {}
#[doc = "`write(|w| ..)` method takes [`p0omflen::W`](W) writer structure"]
impl crate::Writable for P0omflenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P0OMFLEN to value 0"]
impl crate::Resettable for P0omflenSpec {
    const RESET_VALUE: u32 = 0;
}
