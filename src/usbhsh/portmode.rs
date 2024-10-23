#[doc = "Register `PORTMODE` reader"]
pub type R = crate::R<PortmodeSpec>;
#[doc = "Register `PORTMODE` writer"]
pub type W = crate::W<PortmodeSpec>;
#[doc = "Field `DEV_ENABLE` reader - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
pub type DevEnableR = crate::BitReader;
#[doc = "Field `DEV_ENABLE` writer - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
pub type DevEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DevEnableR {
        DevEnableR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTMODE")
            .field("dev_enable", &self.dev_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - If this bit is set to one, the port will behave as a USB device. If this bit is set to zero, the port will be controlled by the USB host block."]
    #[inline(always)]
    #[must_use]
    pub fn dev_enable(&mut self) -> DevEnableW<PortmodeSpec> {
        DevEnableW::new(self, 16)
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block\n\nYou can [`read`](crate::Reg::read) this register and get [`portmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortmodeSpec;
impl crate::RegisterSpec for PortmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portmode::R`](R) reader structure"]
impl crate::Readable for PortmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`portmode::W`](W) writer structure"]
impl crate::Writable for PortmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PORTMODE to value 0x0005_0000"]
impl crate::Resettable for PortmodeSpec {
    const RESET_VALUE: u32 = 0x0005_0000;
}
