#[doc = "Register `DLY` reader"]
pub type R = crate::R<DlySpec>;
#[doc = "Register `DLY` writer"]
pub type W = crate::W<DlySpec>;
#[doc = "Field `PRE_DELAY` reader - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type PreDelayR = crate::FieldReader;
#[doc = "Field `PRE_DELAY` writer - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type PreDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POST_DELAY` reader - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type PostDelayR = crate::FieldReader;
#[doc = "Field `POST_DELAY` writer - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type PostDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRAME_DELAY` reader - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type FrameDelayR = crate::FieldReader;
#[doc = "Field `FRAME_DELAY` writer - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type FrameDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRANSFER_DELAY` reader - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
pub type TransferDelayR = crate::FieldReader;
#[doc = "Field `TRANSFER_DELAY` writer - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
pub type TransferDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn pre_delay(&self) -> PreDelayR {
        PreDelayR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn post_delay(&self) -> PostDelayR {
        PostDelayR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn frame_delay(&self) -> FrameDelayR {
        FrameDelayR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub fn transfer_delay(&self) -> TransferDelayR {
        TransferDelayR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY")
            .field("pre_delay", &self.pre_delay())
            .field("post_delay", &self.post_delay())
            .field("frame_delay", &self.frame_delay())
            .field("transfer_delay", &self.transfer_delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    #[must_use]
    pub fn pre_delay(&mut self) -> PreDelayW<DlySpec> {
        PreDelayW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    #[must_use]
    pub fn post_delay(&mut self) -> PostDelayW<DlySpec> {
        PostDelayW::new(self, 4)
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    #[must_use]
    pub fn frame_delay(&mut self) -> FrameDelayW<DlySpec> {
        FrameDelayW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_delay(&mut self) -> TransferDelayW<DlySpec> {
        TransferDelayW::new(self, 12)
    }
}
#[doc = "SPI Delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`dly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlySpec;
impl crate::RegisterSpec for DlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dly::R`](R) reader structure"]
impl crate::Readable for DlySpec {}
#[doc = "`write(|w| ..)` method takes [`dly::W`](W) writer structure"]
impl crate::Writable for DlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLY to value 0"]
impl crate::Resettable for DlySpec {
    const RESET_VALUE: u32 = 0;
}
