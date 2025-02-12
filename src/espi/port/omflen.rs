#[doc = "Register `OMFLEN` reader"]
pub type R = crate::R<OmflenSpec>;
#[doc = "Register `OMFLEN` writer"]
pub type W = crate::W<OmflenSpec>;
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
        f.debug_struct("OMFLEN")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<OmflenSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub fn trans(&mut self) -> TransW<OmflenSpec> {
        TransW::new(self, 12)
    }
}
#[doc = "Port OOB, Mastering and Flash Length (for OOB, Bus Mastering, and Flash)\n\nYou can [`read`](crate::Reg::read) this register and get [`omflen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omflen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OmflenSpec;
impl crate::RegisterSpec for OmflenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omflen::R`](R) reader structure"]
impl crate::Readable for OmflenSpec {}
#[doc = "`write(|w| ..)` method takes [`omflen::W`](W) writer structure"]
impl crate::Writable for OmflenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OMFLEN to value 0"]
impl crate::Resettable for OmflenSpec {
    const RESET_VALUE: u32 = 0;
}
