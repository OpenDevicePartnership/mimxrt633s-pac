#[doc = "Register `HCSPARAMS` reader"]
pub type R = crate::R<HcsparamsSpec>;
#[doc = "Field `N_PORTS` reader - This register specifies the number of physical downstream ports implemented on this host controller."]
pub type NPortsR = crate::FieldReader;
#[doc = "Field `PPC` reader - This field indicates whether the host controller implementation includes port power control."]
pub type PpcR = crate::BitReader;
#[doc = "Field `P_INDICATOR` reader - This bit indicates whether the ports support port indicator control."]
pub type PIndicatorR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - This register specifies the number of physical downstream ports implemented on this host controller."]
    #[inline(always)]
    pub fn n_ports(&self) -> NPortsR {
        NPortsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This field indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    pub fn ppc(&self) -> PpcR {
        PpcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit indicates whether the ports support port indicator control."]
    #[inline(always)]
    pub fn p_indicator(&self) -> PIndicatorR {
        PIndicatorR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPARAMS")
            .field("n_ports", &self.n_ports())
            .field("ppc", &self.ppc())
            .field("p_indicator", &self.p_indicator())
            .finish()
    }
}
#[doc = "Host Controller Structural Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsparams::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcsparamsSpec;
impl crate::RegisterSpec for HcsparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsparams::R`](R) reader structure"]
impl crate::Readable for HcsparamsSpec {}
#[doc = "`reset()` method sets HCSPARAMS to value 0x0001_0011"]
impl crate::Resettable for HcsparamsSpec {
    const RESET_VALUE: u32 = 0x0001_0011;
}
