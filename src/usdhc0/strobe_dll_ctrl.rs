#[doc = "Register `STROBE_DLL_CTRL` reader"]
pub type R = crate::R<StrobeDllCtrlSpec>;
#[doc = "Register `STROBE_DLL_CTRL` writer"]
pub type W = crate::W<StrobeDllCtrlSpec>;
#[doc = "Field `STROBE_DLL_CTRL_ENABLE` reader - Strobe DLL Control Enable"]
pub type StrobeDllCtrlEnableR = crate::BitReader;
#[doc = "Field `STROBE_DLL_CTRL_ENABLE` writer - Strobe DLL Control Enable"]
pub type StrobeDllCtrlEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_DLL_CTRL_RESET` reader - Strobe DLL Control Reset"]
pub type StrobeDllCtrlResetR = crate::BitReader;
#[doc = "Field `STROBE_DLL_CTRL_RESET` writer - Strobe DLL Control Reset"]
pub type StrobeDllCtrlResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_FORCE_UPD` reader - Strobe DLL Control Slave Force Updated"]
pub type StrobeDllCtrlSlvForceUpdR = crate::BitReader;
#[doc = "Field `STROBE_DLL_CTRL_SLV_FORCE_UPD` writer - Strobe DLL Control Slave Force Updated"]
pub type StrobeDllCtrlSlvForceUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_DLY_TARGET` reader - Strobe DLL Control Slave Delay Target"]
pub type StrobeDllCtrlSlvDlyTargetR = crate::FieldReader;
#[doc = "Field `STROBE_DLL_CTRL_SLV_DLY_TARGET` writer - Strobe DLL Control Slave Delay Target"]
pub type StrobeDllCtrlSlvDlyTargetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_0` reader - Strobe DLL Control Gate Update"]
pub type StrobeDllCtrlGateUpdate0R = crate::BitReader;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_0` writer - Strobe DLL Control Gate Update"]
pub type StrobeDllCtrlGateUpdate0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_1` reader - Strobe DLL Control Gate Update"]
pub type StrobeDllCtrlGateUpdate1R = crate::BitReader;
#[doc = "Field `STROBE_DLL_CTRL_GATE_UPDATE_1` writer - Strobe DLL Control Gate Update"]
pub type StrobeDllCtrlGateUpdate1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE` reader - Strobe DLL Control Slave Override"]
pub type StrobeDllCtrlSlvOverrideR = crate::BitReader;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE` writer - Strobe DLL Control Slave Override"]
pub type StrobeDllCtrlSlvOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE_VAL` reader - Strobe DLL Control Slave Override Value"]
pub type StrobeDllCtrlSlvOverrideValR = crate::FieldReader;
#[doc = "Field `STROBE_DLL_CTRL_SLV_OVERRIDE_VAL` writer - Strobe DLL Control Slave Override Value"]
pub type StrobeDllCtrlSlvOverrideValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `STROBE_DLL_CTRL_SLV_UPDATE_INT` reader - Strobe DLL Control Slave Update Interval"]
pub type StrobeDllCtrlSlvUpdateIntR = crate::FieldReader;
#[doc = "Field `STROBE_DLL_CTRL_SLV_UPDATE_INT` writer - Strobe DLL Control Slave Update Interval"]
pub type StrobeDllCtrlSlvUpdateIntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STROBE_DLL_CTRL_REF_UPDATE_INT` reader - Strobe DLL Control Reference Update Interval"]
pub type StrobeDllCtrlRefUpdateIntR = crate::FieldReader;
#[doc = "Field `STROBE_DLL_CTRL_REF_UPDATE_INT` writer - Strobe DLL Control Reference Update Interval"]
pub type StrobeDllCtrlRefUpdateIntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Strobe DLL Control Enable"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_enable(&self) -> StrobeDllCtrlEnableR {
        StrobeDllCtrlEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Strobe DLL Control Reset"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_reset(&self) -> StrobeDllCtrlResetR {
        StrobeDllCtrlResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Strobe DLL Control Slave Force Updated"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_force_upd(&self) -> StrobeDllCtrlSlvForceUpdR {
        StrobeDllCtrlSlvForceUpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Strobe DLL Control Slave Delay Target"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_dly_target(&self) -> StrobeDllCtrlSlvDlyTargetR {
        StrobeDllCtrlSlvDlyTargetR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Strobe DLL Control Gate Update"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_gate_update_0(&self) -> StrobeDllCtrlGateUpdate0R {
        StrobeDllCtrlGateUpdate0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Strobe DLL Control Gate Update"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_gate_update_1(&self) -> StrobeDllCtrlGateUpdate1R {
        StrobeDllCtrlGateUpdate1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Strobe DLL Control Slave Override"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_override(&self) -> StrobeDllCtrlSlvOverrideR {
        StrobeDllCtrlSlvOverrideR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Strobe DLL Control Slave Override Value"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_override_val(&self) -> StrobeDllCtrlSlvOverrideValR {
        StrobeDllCtrlSlvOverrideValR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 20:27 - Strobe DLL Control Slave Update Interval"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_slv_update_int(&self) -> StrobeDllCtrlSlvUpdateIntR {
        StrobeDllCtrlSlvUpdateIntR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Strobe DLL Control Reference Update Interval"]
    #[inline(always)]
    pub fn strobe_dll_ctrl_ref_update_int(&self) -> StrobeDllCtrlRefUpdateIntR {
        StrobeDllCtrlRefUpdateIntR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STROBE_DLL_CTRL")
            .field("strobe_dll_ctrl_enable", &self.strobe_dll_ctrl_enable())
            .field("strobe_dll_ctrl_reset", &self.strobe_dll_ctrl_reset())
            .field(
                "strobe_dll_ctrl_slv_force_upd",
                &self.strobe_dll_ctrl_slv_force_upd(),
            )
            .field(
                "strobe_dll_ctrl_slv_dly_target",
                &self.strobe_dll_ctrl_slv_dly_target(),
            )
            .field(
                "strobe_dll_ctrl_gate_update_0",
                &self.strobe_dll_ctrl_gate_update_0(),
            )
            .field(
                "strobe_dll_ctrl_gate_update_1",
                &self.strobe_dll_ctrl_gate_update_1(),
            )
            .field(
                "strobe_dll_ctrl_slv_override",
                &self.strobe_dll_ctrl_slv_override(),
            )
            .field(
                "strobe_dll_ctrl_slv_override_val",
                &self.strobe_dll_ctrl_slv_override_val(),
            )
            .field(
                "strobe_dll_ctrl_slv_update_int",
                &self.strobe_dll_ctrl_slv_update_int(),
            )
            .field(
                "strobe_dll_ctrl_ref_update_int",
                &self.strobe_dll_ctrl_ref_update_int(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Strobe DLL Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_enable(&mut self) -> StrobeDllCtrlEnableW<StrobeDllCtrlSpec> {
        StrobeDllCtrlEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Strobe DLL Control Reset"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_reset(&mut self) -> StrobeDllCtrlResetW<StrobeDllCtrlSpec> {
        StrobeDllCtrlResetW::new(self, 1)
    }
    #[doc = "Bit 2 - Strobe DLL Control Slave Force Updated"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_force_upd(
        &mut self,
    ) -> StrobeDllCtrlSlvForceUpdW<StrobeDllCtrlSpec> {
        StrobeDllCtrlSlvForceUpdW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Strobe DLL Control Slave Delay Target"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_dly_target(
        &mut self,
    ) -> StrobeDllCtrlSlvDlyTargetW<StrobeDllCtrlSpec> {
        StrobeDllCtrlSlvDlyTargetW::new(self, 3)
    }
    #[doc = "Bit 6 - Strobe DLL Control Gate Update"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_gate_update_0(
        &mut self,
    ) -> StrobeDllCtrlGateUpdate0W<StrobeDllCtrlSpec> {
        StrobeDllCtrlGateUpdate0W::new(self, 6)
    }
    #[doc = "Bit 7 - Strobe DLL Control Gate Update"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_gate_update_1(
        &mut self,
    ) -> StrobeDllCtrlGateUpdate1W<StrobeDllCtrlSpec> {
        StrobeDllCtrlGateUpdate1W::new(self, 7)
    }
    #[doc = "Bit 8 - Strobe DLL Control Slave Override"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_override(&mut self) -> StrobeDllCtrlSlvOverrideW<StrobeDllCtrlSpec> {
        StrobeDllCtrlSlvOverrideW::new(self, 8)
    }
    #[doc = "Bits 9:15 - Strobe DLL Control Slave Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_override_val(
        &mut self,
    ) -> StrobeDllCtrlSlvOverrideValW<StrobeDllCtrlSpec> {
        StrobeDllCtrlSlvOverrideValW::new(self, 9)
    }
    #[doc = "Bits 20:27 - Strobe DLL Control Slave Update Interval"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_slv_update_int(
        &mut self,
    ) -> StrobeDllCtrlSlvUpdateIntW<StrobeDllCtrlSpec> {
        StrobeDllCtrlSlvUpdateIntW::new(self, 20)
    }
    #[doc = "Bits 28:31 - Strobe DLL Control Reference Update Interval"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_dll_ctrl_ref_update_int(
        &mut self,
    ) -> StrobeDllCtrlRefUpdateIntW<StrobeDllCtrlSpec> {
        StrobeDllCtrlRefUpdateIntW::new(self, 28)
    }
}
#[doc = "Strobe DLL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`strobe_dll_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`strobe_dll_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrobeDllCtrlSpec;
impl crate::RegisterSpec for StrobeDllCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`strobe_dll_ctrl::R`](R) reader structure"]
impl crate::Readable for StrobeDllCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`strobe_dll_ctrl::W`](W) writer structure"]
impl crate::Writable for StrobeDllCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STROBE_DLL_CTRL to value 0"]
impl crate::Resettable for StrobeDllCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
