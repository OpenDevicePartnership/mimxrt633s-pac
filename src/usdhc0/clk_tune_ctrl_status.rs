#[doc = "Register `CLK_TUNE_CTRL_STATUS` reader"]
pub type R = crate::R<ClkTuneCtrlStatusSpec>;
#[doc = "Register `CLK_TUNE_CTRL_STATUS` writer"]
pub type W = crate::W<ClkTuneCtrlStatusSpec>;
#[doc = "Field `DLY_CELL_SET_POST` reader - DLY_CELL_SET_POST"]
pub type DlyCellSetPostR = crate::FieldReader;
#[doc = "Field `DLY_CELL_SET_POST` writer - DLY_CELL_SET_POST"]
pub type DlyCellSetPostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLY_CELL_SET_OUT` reader - DLY_CELL_SET_OUT"]
pub type DlyCellSetOutR = crate::FieldReader;
#[doc = "Field `DLY_CELL_SET_OUT` writer - DLY_CELL_SET_OUT"]
pub type DlyCellSetOutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLY_CELL_SET_PRE` reader - DLY_CELL_SET_PRE"]
pub type DlyCellSetPreR = crate::FieldReader;
#[doc = "Field `DLY_CELL_SET_PRE` writer - DLY_CELL_SET_PRE"]
pub type DlyCellSetPreW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NXT_ERR` reader - NXT_ERR"]
pub type NxtErrR = crate::BitReader;
#[doc = "Field `TAP_SEL_POST` reader - TAP_SEL_POST"]
pub type TapSelPostR = crate::FieldReader;
#[doc = "Field `TAP_SEL_OUT` reader - TAP_SEL_OUT"]
pub type TapSelOutR = crate::FieldReader;
#[doc = "Field `TAP_SEL_PRE` reader - TAP_SEL_PRE"]
pub type TapSelPreR = crate::FieldReader;
#[doc = "Field `PRE_ERR` reader - PRE_ERR"]
pub type PreErrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - DLY_CELL_SET_POST"]
    #[inline(always)]
    pub fn dly_cell_set_post(&self) -> DlyCellSetPostR {
        DlyCellSetPostR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DLY_CELL_SET_OUT"]
    #[inline(always)]
    pub fn dly_cell_set_out(&self) -> DlyCellSetOutR {
        DlyCellSetOutR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - DLY_CELL_SET_PRE"]
    #[inline(always)]
    pub fn dly_cell_set_pre(&self) -> DlyCellSetPreR {
        DlyCellSetPreR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - NXT_ERR"]
    #[inline(always)]
    pub fn nxt_err(&self) -> NxtErrR {
        NxtErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - TAP_SEL_POST"]
    #[inline(always)]
    pub fn tap_sel_post(&self) -> TapSelPostR {
        TapSelPostR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TAP_SEL_OUT"]
    #[inline(always)]
    pub fn tap_sel_out(&self) -> TapSelOutR {
        TapSelOutR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - TAP_SEL_PRE"]
    #[inline(always)]
    pub fn tap_sel_pre(&self) -> TapSelPreR {
        TapSelPreR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - PRE_ERR"]
    #[inline(always)]
    pub fn pre_err(&self) -> PreErrR {
        PreErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_TUNE_CTRL_STATUS")
            .field("dly_cell_set_post", &self.dly_cell_set_post())
            .field("dly_cell_set_out", &self.dly_cell_set_out())
            .field("dly_cell_set_pre", &self.dly_cell_set_pre())
            .field("nxt_err", &self.nxt_err())
            .field("tap_sel_post", &self.tap_sel_post())
            .field("tap_sel_out", &self.tap_sel_out())
            .field("tap_sel_pre", &self.tap_sel_pre())
            .field("pre_err", &self.pre_err())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - DLY_CELL_SET_POST"]
    #[inline(always)]
    pub fn dly_cell_set_post(&mut self) -> DlyCellSetPostW<ClkTuneCtrlStatusSpec> {
        DlyCellSetPostW::new(self, 0)
    }
    #[doc = "Bits 4:7 - DLY_CELL_SET_OUT"]
    #[inline(always)]
    pub fn dly_cell_set_out(&mut self) -> DlyCellSetOutW<ClkTuneCtrlStatusSpec> {
        DlyCellSetOutW::new(self, 4)
    }
    #[doc = "Bits 8:14 - DLY_CELL_SET_PRE"]
    #[inline(always)]
    pub fn dly_cell_set_pre(&mut self) -> DlyCellSetPreW<ClkTuneCtrlStatusSpec> {
        DlyCellSetPreW::new(self, 8)
    }
}
#[doc = "CLK Tuning Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_tune_ctrl_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_tune_ctrl_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTuneCtrlStatusSpec;
impl crate::RegisterSpec for ClkTuneCtrlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_tune_ctrl_status::R`](R) reader structure"]
impl crate::Readable for ClkTuneCtrlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_tune_ctrl_status::W`](W) writer structure"]
impl crate::Writable for ClkTuneCtrlStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TUNE_CTRL_STATUS to value 0"]
impl crate::Resettable for ClkTuneCtrlStatusSpec {
    const RESET_VALUE: u32 = 0;
}
