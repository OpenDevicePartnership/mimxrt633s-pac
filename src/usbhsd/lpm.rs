#[doc = "Register `LPM` reader"]
pub type R = crate::R<LpmSpec>;
#[doc = "Register `LPM` writer"]
pub type W = crate::W<LpmSpec>;
#[doc = "Field `HIRD_HW` reader - Host Initiated Resume Duration - HW."]
pub type HirdHwR = crate::FieldReader;
#[doc = "Field `HIRD_SW` reader - Host Initiated Resume Duration - SW."]
pub type HirdSwR = crate::FieldReader;
#[doc = "Field `HIRD_SW` writer - Host Initiated Resume Duration - SW."]
pub type HirdSwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATA_PENDING` reader - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
pub type DataPendingR = crate::BitReader;
#[doc = "Field `DATA_PENDING` writer - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
pub type DataPendingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW."]
    #[inline(always)]
    pub fn hird_hw(&self) -> HirdHwR {
        HirdHwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub fn hird_sw(&self) -> HirdSwR {
        HirdSwR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub fn data_pending(&self) -> DataPendingR {
        DataPendingR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPM")
            .field("hird_hw", &self.hird_hw())
            .field("hird_sw", &self.hird_sw())
            .field("data_pending", &self.data_pending())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW."]
    #[inline(always)]
    #[must_use]
    pub fn hird_sw(&mut self) -> HirdSwW<LpmSpec> {
        HirdSwW::new(self, 4)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    #[must_use]
    pub fn data_pending(&mut self) -> DataPendingW<LpmSpec> {
        DataPendingW::new(self, 8)
    }
}
#[doc = "USB Link Power Management register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmSpec;
impl crate::RegisterSpec for LpmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpm::R`](R) reader structure"]
impl crate::Readable for LpmSpec {}
#[doc = "`write(|w| ..)` method takes [`lpm::W`](W) writer structure"]
impl crate::Writable for LpmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPM to value 0"]
impl crate::Resettable for LpmSpec {
    const RESET_VALUE: u32 = 0;
}
