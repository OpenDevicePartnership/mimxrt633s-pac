#[doc = "Register `USBCMD` reader"]
pub type R = crate::R<UsbcmdSpec>;
#[doc = "Register `USBCMD` writer"]
pub type W = crate::W<UsbcmdSpec>;
#[doc = "Field `RS` reader - Run/Stop: 1b = Run."]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - Run/Stop: 1b = Run."]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCRESET` reader - Host Controller Reset: This control bit is used by the software to reset the host controller."]
pub type HcresetR = crate::BitReader;
#[doc = "Field `HCRESET` writer - Host Controller Reset: This control bit is used by the software to reset the host controller."]
pub type HcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLS` reader - Frame List Size: This field specifies the size of the frame list."]
pub type FlsR = crate::FieldReader;
#[doc = "Field `FLS` writer - Frame List Size: This field specifies the size of the frame list."]
pub type FlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LHCR` reader - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
pub type LhcrR = crate::BitReader;
#[doc = "Field `LHCR` writer - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
pub type LhcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATL_EN` reader - ATL List enabled."]
pub type AtlEnR = crate::BitReader;
#[doc = "Field `ATL_EN` writer - ATL List enabled."]
pub type AtlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_EN` reader - ISO List enabled."]
pub type IsoEnR = crate::BitReader;
#[doc = "Field `ISO_EN` writer - ISO List enabled."]
pub type IsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - INT List enabled."]
pub type IntEnR = crate::BitReader;
#[doc = "Field `INT_EN` writer - INT List enabled."]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIRD` reader - Host-Initiated Resume Duration."]
pub type HirdR = crate::FieldReader;
#[doc = "Field `HIRD` writer - Host-Initiated Resume Duration."]
pub type HirdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&self) -> HcresetR {
        HcresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&self) -> FlsR {
        FlsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&self) -> LhcrR {
        LhcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&self) -> AtlEnR {
        AtlEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&self) -> IsoEnR {
        IsoEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&self) -> HirdR {
        HirdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCMD")
            .field("rs", &self.rs())
            .field("hcreset", &self.hcreset())
            .field("fls", &self.fls())
            .field("lhcr", &self.lhcr())
            .field("atl_en", &self.atl_en())
            .field("iso_en", &self.iso_en())
            .field("int_en", &self.int_en())
            .field("hird", &self.hird())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<UsbcmdSpec> {
        RsW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&mut self) -> HcresetW<UsbcmdSpec> {
        HcresetW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&mut self) -> FlsW<UsbcmdSpec> {
        FlsW::new(self, 2)
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&mut self) -> LhcrW<UsbcmdSpec> {
        LhcrW::new(self, 7)
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&mut self) -> AtlEnW<UsbcmdSpec> {
        AtlEnW::new(self, 8)
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&mut self) -> IsoEnW<UsbcmdSpec> {
        IsoEnW::new(self, 9)
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&mut self) -> IntEnW<UsbcmdSpec> {
        IntEnW::new(self, 10)
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&mut self) -> HirdW<UsbcmdSpec> {
        HirdW::new(self, 24)
    }
}
#[doc = "USB Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbcmdSpec;
impl crate::RegisterSpec for UsbcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbcmd::R`](R) reader structure"]
impl crate::Readable for UsbcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`usbcmd::W`](W) writer structure"]
impl crate::Writable for UsbcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCMD to value 0"]
impl crate::Resettable for UsbcmdSpec {
    const RESET_VALUE: u32 = 0;
}
