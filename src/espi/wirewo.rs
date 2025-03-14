#[doc = "Register `WIREWO` reader"]
pub type R = crate::R<WirewoSpec>;
#[doc = "Register `WIREWO` writer"]
pub type W = crate::W<WirewoSpec>;
#[doc = "Field `OOB_RST_ACK` reader - Must be asserted in response to OOB_RST."]
pub type OobRstAckR = crate::BitReader;
#[doc = "Field `OOB_RST_ACK` writer - Must be asserted in response to OOB_RST."]
pub type OobRstAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEN_SCIN` reader - Wake up when no S0, SCI if it is."]
pub type WakenScinR = crate::BitReader;
#[doc = "Field `WAKEN_SCIN` writer - Wake up when no S0, SCI if it is."]
pub type WakenScinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMEN` reader - Replacement for Pin."]
pub type PmenR = crate::BitReader;
#[doc = "Field `PMEN` writer - Replacement for Pin."]
pub type PmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIN` reader - Replacement for Pin."]
pub type ScinR = crate::BitReader;
#[doc = "Field `SCIN` writer - Replacement for Pin."]
pub type ScinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMIN` reader - Replacement for Pin / IRQ."]
pub type SminR = crate::BitReader;
#[doc = "Field `SMIN` writer - Replacement for Pin / IRQ."]
pub type SminW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCINN` reader - Replacement for KBDRST Pin."]
pub type RcinnR = crate::BitReader;
#[doc = "Field `RCINN` writer - Replacement for KBDRST Pin."]
pub type RcinnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_RST_ACK` reader - Set in response to HOST_RST_WARN."]
pub type HostRstAckR = crate::BitReader;
#[doc = "Field `HOST_RST_ACK` writer - Set in response to HOST_RST_WARN."]
pub type HostRstAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSACKN` reader - Set in response to SUS_WARNN."]
pub type SusacknR = crate::BitReader;
#[doc = "Field `SUSACKN` writer - Set in response to SUS_WARNN."]
pub type SusacknW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E2P` reader - EC to PCH byte for agreed use."]
pub type E2pR = crate::FieldReader;
#[doc = "Field `E2P` writer - EC to PCH byte for agreed use."]
pub type E2pW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOOT_DONE` reader - Boot load is done, app must write 1."]
pub type BootDoneR = crate::BitReader;
#[doc = "Field `BOOT_DONE` writer - Boot load is done, app must write 1."]
pub type BootDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ERRN` reader - Boot load ended in success if 1. App must write."]
pub type BootErrnR = crate::BitReader;
#[doc = "Field `BOOT_ERRN` writer - Boot load ended in success if 1. App must write."]
pub type BootErrnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSW_PWROK_RST` reader - App write 1 to this when Host goes int G3 vs Deep-Sx. It can be cleared at any time after that. The purpose it to reset some VWires."]
pub type DswPwrokRstR = crate::BitReader;
#[doc = "Field `DSW_PWROK_RST` writer - App write 1 to this when Host goes int G3 vs Deep-Sx. It can be cleared at any time after that. The purpose it to reset some VWires."]
pub type DswPwrokRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - Set to 1 after last write has been pushed out to Host"]
pub type DoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Must be asserted in response to OOB_RST."]
    #[inline(always)]
    pub fn oob_rst_ack(&self) -> OobRstAckR {
        OobRstAckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake up when no S0, SCI if it is."]
    #[inline(always)]
    pub fn waken_scin(&self) -> WakenScinR {
        WakenScinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Replacement for Pin."]
    #[inline(always)]
    pub fn pmen(&self) -> PmenR {
        PmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Replacement for Pin."]
    #[inline(always)]
    pub fn scin(&self) -> ScinR {
        ScinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Replacement for Pin / IRQ."]
    #[inline(always)]
    pub fn smin(&self) -> SminR {
        SminR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Replacement for KBDRST Pin."]
    #[inline(always)]
    pub fn rcinn(&self) -> RcinnR {
        RcinnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set in response to HOST_RST_WARN."]
    #[inline(always)]
    pub fn host_rst_ack(&self) -> HostRstAckR {
        HostRstAckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set in response to SUS_WARNN."]
    #[inline(always)]
    pub fn susackn(&self) -> SusacknR {
        SusacknR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - EC to PCH byte for agreed use."]
    #[inline(always)]
    pub fn e2p(&self) -> E2pR {
        E2pR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Boot load is done, app must write 1."]
    #[inline(always)]
    pub fn boot_done(&self) -> BootDoneR {
        BootDoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Boot load ended in success if 1. App must write."]
    #[inline(always)]
    pub fn boot_errn(&self) -> BootErrnR {
        BootErrnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - App write 1 to this when Host goes int G3 vs Deep-Sx. It can be cleared at any time after that. The purpose it to reset some VWires."]
    #[inline(always)]
    pub fn dsw_pwrok_rst(&self) -> DswPwrokRstR {
        DswPwrokRstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Set to 1 after last write has been pushed out to Host"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIREWO")
            .field("done", &self.done())
            .field("oob_rst_ack", &self.oob_rst_ack())
            .field("waken_scin", &self.waken_scin())
            .field("pmen", &self.pmen())
            .field("scin", &self.scin())
            .field("smin", &self.smin())
            .field("rcinn", &self.rcinn())
            .field("host_rst_ack", &self.host_rst_ack())
            .field("susackn", &self.susackn())
            .field("e2p", &self.e2p())
            .field("boot_done", &self.boot_done())
            .field("boot_errn", &self.boot_errn())
            .field("dsw_pwrok_rst", &self.dsw_pwrok_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Must be asserted in response to OOB_RST."]
    #[inline(always)]
    pub fn oob_rst_ack(&mut self) -> OobRstAckW<WirewoSpec> {
        OobRstAckW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake up when no S0, SCI if it is."]
    #[inline(always)]
    pub fn waken_scin(&mut self) -> WakenScinW<WirewoSpec> {
        WakenScinW::new(self, 1)
    }
    #[doc = "Bit 2 - Replacement for Pin."]
    #[inline(always)]
    pub fn pmen(&mut self) -> PmenW<WirewoSpec> {
        PmenW::new(self, 2)
    }
    #[doc = "Bit 3 - Replacement for Pin."]
    #[inline(always)]
    pub fn scin(&mut self) -> ScinW<WirewoSpec> {
        ScinW::new(self, 3)
    }
    #[doc = "Bit 4 - Replacement for Pin / IRQ."]
    #[inline(always)]
    pub fn smin(&mut self) -> SminW<WirewoSpec> {
        SminW::new(self, 4)
    }
    #[doc = "Bit 5 - Replacement for KBDRST Pin."]
    #[inline(always)]
    pub fn rcinn(&mut self) -> RcinnW<WirewoSpec> {
        RcinnW::new(self, 5)
    }
    #[doc = "Bit 6 - Set in response to HOST_RST_WARN."]
    #[inline(always)]
    pub fn host_rst_ack(&mut self) -> HostRstAckW<WirewoSpec> {
        HostRstAckW::new(self, 6)
    }
    #[doc = "Bit 7 - Set in response to SUS_WARNN."]
    #[inline(always)]
    pub fn susackn(&mut self) -> SusacknW<WirewoSpec> {
        SusacknW::new(self, 7)
    }
    #[doc = "Bits 8:15 - EC to PCH byte for agreed use."]
    #[inline(always)]
    pub fn e2p(&mut self) -> E2pW<WirewoSpec> {
        E2pW::new(self, 8)
    }
    #[doc = "Bit 16 - Boot load is done, app must write 1."]
    #[inline(always)]
    pub fn boot_done(&mut self) -> BootDoneW<WirewoSpec> {
        BootDoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Boot load ended in success if 1. App must write."]
    #[inline(always)]
    pub fn boot_errn(&mut self) -> BootErrnW<WirewoSpec> {
        BootErrnW::new(self, 17)
    }
    #[doc = "Bit 18 - App write 1 to this when Host goes int G3 vs Deep-Sx. It can be cleared at any time after that. The purpose it to reset some VWires."]
    #[inline(always)]
    pub fn dsw_pwrok_rst(&mut self) -> DswPwrokRstW<WirewoSpec> {
        DswPwrokRstW::new(self, 18)
    }
}
#[doc = "Wire states for Host to see; if LPC, this is the IRQ states.\n\nYou can [`read`](crate::Reg::read) this register and get [`wirewo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wirewo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WirewoSpec;
impl crate::RegisterSpec for WirewoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wirewo::R`](R) reader structure"]
impl crate::Readable for WirewoSpec {}
#[doc = "`write(|w| ..)` method takes [`wirewo::W`](W) writer structure"]
impl crate::Writable for WirewoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIREWO to value 0"]
impl crate::Resettable for WirewoSpec {
    const RESET_VALUE: u32 = 0;
}
