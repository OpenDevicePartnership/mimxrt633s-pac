#[doc = "Register `DLL_CTRL` reader"]
pub type R = crate::R<DllCtrlSpec>;
#[doc = "Register `DLL_CTRL` writer"]
pub type W = crate::W<DllCtrlSpec>;
#[doc = "Field `DLL_CTRL_ENABLE` reader - DLL_CTRL_ENABLE"]
pub type DllCtrlEnableR = crate::BitReader;
#[doc = "Field `DLL_CTRL_ENABLE` writer - DLL_CTRL_ENABLE"]
pub type DllCtrlEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CTRL_RESET` reader - DLL_CTRL_RESET"]
pub type DllCtrlResetR = crate::BitReader;
#[doc = "Field `DLL_CTRL_RESET` writer - DLL_CTRL_RESET"]
pub type DllCtrlResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CTRL_SLV_FORCE_UPD` reader - DLL_CTRL_SLV_FORCE_UPD"]
pub type DllCtrlSlvForceUpdR = crate::BitReader;
#[doc = "Field `DLL_CTRL_SLV_FORCE_UPD` writer - DLL_CTRL_SLV_FORCE_UPD"]
pub type DllCtrlSlvForceUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET0` reader - DLL_CTRL_SLV_DLY_TARGET0"]
pub type DllCtrlSlvDlyTarget0R = crate::FieldReader;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET0` writer - DLL_CTRL_SLV_DLY_TARGET0"]
pub type DllCtrlSlvDlyTarget0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLL_CTRL_GATE_UPDATE` reader - DLL_CTRL_GATE_UPDATE"]
pub type DllCtrlGateUpdateR = crate::BitReader;
#[doc = "Field `DLL_CTRL_GATE_UPDATE` writer - DLL_CTRL_GATE_UPDATE"]
pub type DllCtrlGateUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE` reader - DLL_CTRL_SLV_OVERRIDE"]
pub type DllCtrlSlvOverrideR = crate::BitReader;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE` writer - DLL_CTRL_SLV_OVERRIDE"]
pub type DllCtrlSlvOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE_VAL` reader - DLL_CTRL_SLV_OVERRIDE_VAL"]
pub type DllCtrlSlvOverrideValR = crate::FieldReader;
#[doc = "Field `DLL_CTRL_SLV_OVERRIDE_VAL` writer - DLL_CTRL_SLV_OVERRIDE_VAL"]
pub type DllCtrlSlvOverrideValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET1` reader - DLL_CTRL_SLV_DLY_TARGET1"]
pub type DllCtrlSlvDlyTarget1R = crate::FieldReader;
#[doc = "Field `DLL_CTRL_SLV_DLY_TARGET1` writer - DLL_CTRL_SLV_DLY_TARGET1"]
pub type DllCtrlSlvDlyTarget1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLL_CTRL_SLV_UPDATE_INT` reader - DLL_CTRL_SLV_UPDATE_INT"]
pub type DllCtrlSlvUpdateIntR = crate::FieldReader;
#[doc = "Field `DLL_CTRL_SLV_UPDATE_INT` writer - DLL_CTRL_SLV_UPDATE_INT"]
pub type DllCtrlSlvUpdateIntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLL_CTRL_REF_UPDATE_INT` reader - DLL_CTRL_REF_UPDATE_INT"]
pub type DllCtrlRefUpdateIntR = crate::FieldReader;
#[doc = "Field `DLL_CTRL_REF_UPDATE_INT` writer - DLL_CTRL_REF_UPDATE_INT"]
pub type DllCtrlRefUpdateIntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DLL_CTRL_ENABLE"]
    #[inline(always)]
    pub fn dll_ctrl_enable(&self) -> DllCtrlEnableR {
        DllCtrlEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL_CTRL_RESET"]
    #[inline(always)]
    pub fn dll_ctrl_reset(&self) -> DllCtrlResetR {
        DllCtrlResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DLL_CTRL_SLV_FORCE_UPD"]
    #[inline(always)]
    pub fn dll_ctrl_slv_force_upd(&self) -> DllCtrlSlvForceUpdR {
        DllCtrlSlvForceUpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target0(&self) -> DllCtrlSlvDlyTarget0R {
        DllCtrlSlvDlyTarget0R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - DLL_CTRL_GATE_UPDATE"]
    #[inline(always)]
    pub fn dll_ctrl_gate_update(&self) -> DllCtrlGateUpdateR {
        DllCtrlGateUpdateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DLL_CTRL_SLV_OVERRIDE"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override(&self) -> DllCtrlSlvOverrideR {
        DllCtrlSlvOverrideR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline(always)]
    pub fn dll_ctrl_slv_override_val(&self) -> DllCtrlSlvOverrideValR {
        DllCtrlSlvOverrideValR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline(always)]
    pub fn dll_ctrl_slv_dly_target1(&self) -> DllCtrlSlvDlyTarget1R {
        DllCtrlSlvDlyTarget1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:27 - DLL_CTRL_SLV_UPDATE_INT"]
    #[inline(always)]
    pub fn dll_ctrl_slv_update_int(&self) -> DllCtrlSlvUpdateIntR {
        DllCtrlSlvUpdateIntR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - DLL_CTRL_REF_UPDATE_INT"]
    #[inline(always)]
    pub fn dll_ctrl_ref_update_int(&self) -> DllCtrlRefUpdateIntR {
        DllCtrlRefUpdateIntR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_CTRL")
            .field("dll_ctrl_enable", &self.dll_ctrl_enable())
            .field("dll_ctrl_reset", &self.dll_ctrl_reset())
            .field("dll_ctrl_slv_force_upd", &self.dll_ctrl_slv_force_upd())
            .field("dll_ctrl_slv_dly_target0", &self.dll_ctrl_slv_dly_target0())
            .field("dll_ctrl_gate_update", &self.dll_ctrl_gate_update())
            .field("dll_ctrl_slv_override", &self.dll_ctrl_slv_override())
            .field(
                "dll_ctrl_slv_override_val",
                &self.dll_ctrl_slv_override_val(),
            )
            .field("dll_ctrl_slv_dly_target1", &self.dll_ctrl_slv_dly_target1())
            .field("dll_ctrl_slv_update_int", &self.dll_ctrl_slv_update_int())
            .field("dll_ctrl_ref_update_int", &self.dll_ctrl_ref_update_int())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DLL_CTRL_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_enable(&mut self) -> DllCtrlEnableW<DllCtrlSpec> {
        DllCtrlEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - DLL_CTRL_RESET"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_reset(&mut self) -> DllCtrlResetW<DllCtrlSpec> {
        DllCtrlResetW::new(self, 1)
    }
    #[doc = "Bit 2 - DLL_CTRL_SLV_FORCE_UPD"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_force_upd(&mut self) -> DllCtrlSlvForceUpdW<DllCtrlSpec> {
        DllCtrlSlvForceUpdW::new(self, 2)
    }
    #[doc = "Bits 3:6 - DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_dly_target0(&mut self) -> DllCtrlSlvDlyTarget0W<DllCtrlSpec> {
        DllCtrlSlvDlyTarget0W::new(self, 3)
    }
    #[doc = "Bit 7 - DLL_CTRL_GATE_UPDATE"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_gate_update(&mut self) -> DllCtrlGateUpdateW<DllCtrlSpec> {
        DllCtrlGateUpdateW::new(self, 7)
    }
    #[doc = "Bit 8 - DLL_CTRL_SLV_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_override(&mut self) -> DllCtrlSlvOverrideW<DllCtrlSpec> {
        DllCtrlSlvOverrideW::new(self, 8)
    }
    #[doc = "Bits 9:15 - DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_override_val(&mut self) -> DllCtrlSlvOverrideValW<DllCtrlSpec> {
        DllCtrlSlvOverrideValW::new(self, 9)
    }
    #[doc = "Bits 16:18 - DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_dly_target1(&mut self) -> DllCtrlSlvDlyTarget1W<DllCtrlSpec> {
        DllCtrlSlvDlyTarget1W::new(self, 16)
    }
    #[doc = "Bits 20:27 - DLL_CTRL_SLV_UPDATE_INT"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_slv_update_int(&mut self) -> DllCtrlSlvUpdateIntW<DllCtrlSpec> {
        DllCtrlSlvUpdateIntW::new(self, 20)
    }
    #[doc = "Bits 28:31 - DLL_CTRL_REF_UPDATE_INT"]
    #[inline(always)]
    #[must_use]
    pub fn dll_ctrl_ref_update_int(&mut self) -> DllCtrlRefUpdateIntW<DllCtrlSpec> {
        DllCtrlRefUpdateIntW::new(self, 28)
    }
}
#[doc = "DLL (Delay Line) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllCtrlSpec;
impl crate::RegisterSpec for DllCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_ctrl::R`](R) reader structure"]
impl crate::Readable for DllCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dll_ctrl::W`](W) writer structure"]
impl crate::Writable for DllCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CTRL to value 0x0200"]
impl crate::Resettable for DllCtrlSpec {
    const RESET_VALUE: u32 = 0x0200;
}
