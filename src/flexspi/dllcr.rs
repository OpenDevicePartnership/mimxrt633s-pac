#[doc = "Register `DLLCR%s` reader"]
pub type R = crate::R<DllcrSpec>;
#[doc = "Register `DLLCR%s` writer"]
pub type W = crate::W<DllcrSpec>;
#[doc = "Field `DLLEN` reader - DLL calibration enable."]
pub type DllenR = crate::BitReader;
#[doc = "Field `DLLEN` writer - DLL calibration enable."]
pub type DllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLRESET` reader - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
pub type DllresetR = crate::BitReader;
#[doc = "Field `DLLRESET` writer - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
pub type DllresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVDLYTARGET` reader - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
pub type SlvdlytargetR = crate::FieldReader;
#[doc = "Field `SLVDLYTARGET` writer - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
pub type SlvdlytargetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OVRDEN` reader - Slave clock delay line delay cell number selection override enable."]
pub type OvrdenR = crate::BitReader;
#[doc = "Field `OVRDEN` writer - Slave clock delay line delay cell number selection override enable."]
pub type OvrdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRDVAL` reader - Slave clock delay line delay cell number selection override value."]
pub type OvrdvalR = crate::FieldReader;
#[doc = "Field `OVRDVAL` writer - Slave clock delay line delay cell number selection override value."]
pub type OvrdvalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    pub fn dllen(&self) -> DllenR {
        DllenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    pub fn dllreset(&self) -> DllresetR {
        DllresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[inline(always)]
    pub fn slvdlytarget(&self) -> SlvdlytargetR {
        SlvdlytargetR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub fn ovrden(&self) -> OvrdenR {
        OvrdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub fn ovrdval(&self) -> OvrdvalR {
        OvrdvalR::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLLCR")
            .field("dllen", &self.dllen())
            .field("dllreset", &self.dllreset())
            .field("slvdlytarget", &self.slvdlytarget())
            .field("ovrden", &self.ovrden())
            .field("ovrdval", &self.ovrdval())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DLL calibration enable."]
    #[inline(always)]
    #[must_use]
    pub fn dllen(&mut self) -> DllenW<DllcrSpec> {
        DllenW::new(self, 0)
    }
    #[doc = "Bit 1 - Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    #[must_use]
    pub fn dllreset(&mut self) -> DllresetW<DllcrSpec> {
        DllresetW::new(self, 1)
    }
    #[doc = "Bits 3:6 - The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[inline(always)]
    #[must_use]
    pub fn slvdlytarget(&mut self) -> SlvdlytargetW<DllcrSpec> {
        SlvdlytargetW::new(self, 3)
    }
    #[doc = "Bit 8 - Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    #[must_use]
    pub fn ovrden(&mut self) -> OvrdenW<DllcrSpec> {
        OvrdenW::new(self, 8)
    }
    #[doc = "Bits 9:14 - Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    #[must_use]
    pub fn ovrdval(&mut self) -> OvrdvalW<DllcrSpec> {
        OvrdvalW::new(self, 9)
    }
}
#[doc = "DLL Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllcrSpec;
impl crate::RegisterSpec for DllcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dllcr::R`](R) reader structure"]
impl crate::Readable for DllcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dllcr::W`](W) writer structure"]
impl crate::Writable for DllcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLLCR%s to value 0x0100"]
impl crate::Resettable for DllcrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
