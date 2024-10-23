#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Field `TRIG_NUM` reader - Trigger Number"]
pub type TrigNumR = crate::FieldReader;
#[doc = "Result FIFO Depth\n\nValue on reset: 16"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifosize {
    #[doc = "1: Result FIFO depth = 1 dataword."]
    Fifosize1 = 1,
    #[doc = "4: Result FIFO depth = 4 datawords."]
    Fifosize4 = 4,
    #[doc = "8: Result FIFO depth = 8 datawords."]
    Fifosize8 = 8,
    #[doc = "16: Result FIFO depth = 16 datawords."]
    Fifosize16 = 16,
    #[doc = "32: Result FIFO depth = 32 datawords."]
    Fifosize32 = 32,
    #[doc = "64: Result FIFO depth = 64 datawords."]
    Fifosize64 = 64,
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(variant: Fifosize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifosize {
    type Ux = u8;
}
impl crate::IsEnum for Fifosize {}
#[doc = "Field `FIFOSIZE` reader - Result FIFO Depth"]
pub type FifosizeR = crate::FieldReader<Fifosize>;
impl FifosizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fifosize> {
        match self.bits {
            1 => Some(Fifosize::Fifosize1),
            4 => Some(Fifosize::Fifosize4),
            8 => Some(Fifosize::Fifosize8),
            16 => Some(Fifosize::Fifosize16),
            32 => Some(Fifosize::Fifosize32),
            64 => Some(Fifosize::Fifosize64),
            _ => None,
        }
    }
    #[doc = "Result FIFO depth = 1 dataword."]
    #[inline(always)]
    pub fn is_fifosize_1(&self) -> bool {
        *self == Fifosize::Fifosize1
    }
    #[doc = "Result FIFO depth = 4 datawords."]
    #[inline(always)]
    pub fn is_fifosize_4(&self) -> bool {
        *self == Fifosize::Fifosize4
    }
    #[doc = "Result FIFO depth = 8 datawords."]
    #[inline(always)]
    pub fn is_fifosize_8(&self) -> bool {
        *self == Fifosize::Fifosize8
    }
    #[doc = "Result FIFO depth = 16 datawords."]
    #[inline(always)]
    pub fn is_fifosize_16(&self) -> bool {
        *self == Fifosize::Fifosize16
    }
    #[doc = "Result FIFO depth = 32 datawords."]
    #[inline(always)]
    pub fn is_fifosize_32(&self) -> bool {
        *self == Fifosize::Fifosize32
    }
    #[doc = "Result FIFO depth = 64 datawords."]
    #[inline(always)]
    pub fn is_fifosize_64(&self) -> bool {
        *self == Fifosize::Fifosize64
    }
}
#[doc = "Field `CV_NUM` reader - Compare Value Number"]
pub type CvNumR = crate::FieldReader;
#[doc = "Field `CMD_NUM` reader - Command Buffer Number"]
pub type CmdNumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Trigger Number"]
    #[inline(always)]
    pub fn trig_num(&self) -> TrigNumR {
        TrigNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Result FIFO Depth"]
    #[inline(always)]
    pub fn fifosize(&self) -> FifosizeR {
        FifosizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value Number"]
    #[inline(always)]
    pub fn cv_num(&self) -> CvNumR {
        CvNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Command Buffer Number"]
    #[inline(always)]
    pub fn cmd_num(&self) -> CmdNumR {
        CmdNumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM")
            .field("trig_num", &self.trig_num())
            .field("fifosize", &self.fifosize())
            .field("cv_num", &self.cv_num())
            .field("cmd_num", &self.cmd_num())
            .finish()
    }
}
#[doc = "Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0x0f04_1010"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0x0f04_1010;
}
