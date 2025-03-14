#[doc = "Register `WIRERO` reader"]
pub type R = crate::R<WireroSpec>;
#[doc = "Field `SLP_S3N` reader - Sleep state."]
pub type SlpS3nR = crate::BitReader;
#[doc = "Field `SLP_S4N` reader - Sleep state."]
pub type SlpS4nR = crate::BitReader;
#[doc = "Field `SLP_S5N` reader - Sleep state."]
pub type SlpS5nR = crate::BitReader;
#[doc = "Field `SUS_STAT` reader - Warning entry to sleep state soon."]
pub type SusStatR = crate::BitReader;
#[doc = "Field `PLTRSTN` reader - Reset request, affects VWires which must be cleared."]
pub type PltrstnR = crate::BitReader;
#[doc = "Field `OOB_RST_WARN` reader - Notification of OOB being reset. Must be acked using OOB_RST_ACK VWire. Active High."]
pub type OobRstWarnR = crate::BitReader;
#[doc = "Field `HOST_RST_WARN` reader - Warning that Host is about to reset. Must be acked using HOST_RST_ACK. Active High."]
pub type HostRstWarnR = crate::BitReader;
#[doc = "Field `SUS_WARNN` reader - Suspend about to happen, going into Deep-Sx (Power well loss). Must be acked using SUSACK VWire."]
pub type SusWarnnR = crate::BitReader;
#[doc = "Field `SUS_PWRDN_ACKN` reader - Host indicates that suspend power well can be shut down safely."]
pub type SusPwrdnAcknR = crate::BitReader;
#[doc = "Field `SLP_AN` reader - Used when in Sx sleep but ME is on. ASW devices need to be powered. Also called SLP_MN"]
pub type SlpAnR = crate::BitReader;
#[doc = "Field `SLP_LANN` reader - Wired LAN can be powered down."]
pub type SlpLannR = crate::BitReader;
#[doc = "Field `SLP_WLANN` reader - Wireless LAN can be powered down."]
pub type SlpWlannR = crate::BitReader;
#[doc = "Field `P2E` reader - PCH to EC byte for agreed use."]
pub type P2eR = crate::FieldReader;
#[doc = "Field `HOST_C10N` reader - Asserted when Host has entered deep power down state C10 or deeper. It is a sleep sub-state of S0."]
pub type HostC10nR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sleep state."]
    #[inline(always)]
    pub fn slp_s3n(&self) -> SlpS3nR {
        SlpS3nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep state."]
    #[inline(always)]
    pub fn slp_s4n(&self) -> SlpS4nR {
        SlpS4nR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep state."]
    #[inline(always)]
    pub fn slp_s5n(&self) -> SlpS5nR {
        SlpS5nR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Warning entry to sleep state soon."]
    #[inline(always)]
    pub fn sus_stat(&self) -> SusStatR {
        SusStatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset request, affects VWires which must be cleared."]
    #[inline(always)]
    pub fn pltrstn(&self) -> PltrstnR {
        PltrstnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Notification of OOB being reset. Must be acked using OOB_RST_ACK VWire. Active High."]
    #[inline(always)]
    pub fn oob_rst_warn(&self) -> OobRstWarnR {
        OobRstWarnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning that Host is about to reset. Must be acked using HOST_RST_ACK. Active High."]
    #[inline(always)]
    pub fn host_rst_warn(&self) -> HostRstWarnR {
        HostRstWarnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend about to happen, going into Deep-Sx (Power well loss). Must be acked using SUSACK VWire."]
    #[inline(always)]
    pub fn sus_warnn(&self) -> SusWarnnR {
        SusWarnnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host indicates that suspend power well can be shut down safely."]
    #[inline(always)]
    pub fn sus_pwrdn_ackn(&self) -> SusPwrdnAcknR {
        SusPwrdnAcknR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Used when in Sx sleep but ME is on. ASW devices need to be powered. Also called SLP_MN"]
    #[inline(always)]
    pub fn slp_an(&self) -> SlpAnR {
        SlpAnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wired LAN can be powered down."]
    #[inline(always)]
    pub fn slp_lann(&self) -> SlpLannR {
        SlpLannR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wireless LAN can be powered down."]
    #[inline(always)]
    pub fn slp_wlann(&self) -> SlpWlannR {
        SlpWlannR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - PCH to EC byte for agreed use."]
    #[inline(always)]
    pub fn p2e(&self) -> P2eR {
        P2eR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - Asserted when Host has entered deep power down state C10 or deeper. It is a sleep sub-state of S0."]
    #[inline(always)]
    pub fn host_c10n(&self) -> HostC10nR {
        HostC10nR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIRERO")
            .field("slp_s3n", &self.slp_s3n())
            .field("slp_s4n", &self.slp_s4n())
            .field("slp_s5n", &self.slp_s5n())
            .field("sus_stat", &self.sus_stat())
            .field("pltrstn", &self.pltrstn())
            .field("oob_rst_warn", &self.oob_rst_warn())
            .field("host_rst_warn", &self.host_rst_warn())
            .field("sus_warnn", &self.sus_warnn())
            .field("sus_pwrdn_ackn", &self.sus_pwrdn_ackn())
            .field("slp_an", &self.slp_an())
            .field("slp_lann", &self.slp_lann())
            .field("slp_wlann", &self.slp_wlann())
            .field("p2e", &self.p2e())
            .field("host_c10n", &self.host_c10n())
            .finish()
    }
}
#[doc = "Wire states from Host\n\nYou can [`read`](crate::Reg::read) this register and get [`wirero::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WireroSpec;
impl crate::RegisterSpec for WireroSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wirero::R`](R) reader structure"]
impl crate::Readable for WireroSpec {}
#[doc = "`reset()` method sets WIRERO to value 0"]
impl crate::Resettable for WireroSpec {
    const RESET_VALUE: u32 = 0;
}
