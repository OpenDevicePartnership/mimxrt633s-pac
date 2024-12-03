#[doc = "Register `VEND_SPEC2` reader"]
pub type R = crate::R<VendSpec2Spec>;
#[doc = "Register `VEND_SPEC2` writer"]
pub type W = crate::W<VendSpec2Spec>;
#[doc = "Card Interrupt Detection Test\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardIntD3Test {
    #[doc = "0: Check the card interrupt only when DATA3 is high."]
    CardIntD3Test0 = 0,
    #[doc = "1: Check the card interrupt by ignoring the status of DATA3."]
    CardIntD3Test1 = 1,
}
impl From<CardIntD3Test> for bool {
    #[inline(always)]
    fn from(variant: CardIntD3Test) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INT_D3_TEST` reader - Card Interrupt Detection Test"]
pub type CardIntD3TestR = crate::BitReader<CardIntD3Test>;
impl CardIntD3TestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CardIntD3Test {
        match self.bits {
            false => CardIntD3Test::CardIntD3Test0,
            true => CardIntD3Test::CardIntD3Test1,
        }
    }
    #[doc = "Check the card interrupt only when DATA3 is high."]
    #[inline(always)]
    pub fn is_card_int_d3_test_0(&self) -> bool {
        *self == CardIntD3Test::CardIntD3Test0
    }
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    #[inline(always)]
    pub fn is_card_int_d3_test_1(&self) -> bool {
        *self == CardIntD3Test::CardIntD3Test1
    }
}
#[doc = "Field `CARD_INT_D3_TEST` writer - Card Interrupt Detection Test"]
pub type CardIntD3TestW<'a, REG> = crate::BitWriter<'a, REG, CardIntD3Test>;
impl<'a, REG> CardIntD3TestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Check the card interrupt only when DATA3 is high."]
    #[inline(always)]
    pub fn card_int_d3_test_0(self) -> &'a mut crate::W<REG> {
        self.variant(CardIntD3Test::CardIntD3Test0)
    }
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    #[inline(always)]
    pub fn card_int_d3_test_1(self) -> &'a mut crate::W<REG> {
        self.variant(CardIntD3Test::CardIntD3Test1)
    }
}
#[doc = "Field `TUNING_8bit_EN` reader - TUNING_8bit_EN"]
pub type Tuning8bitEnR = crate::BitReader;
#[doc = "Field `TUNING_8bit_EN` writer - TUNING_8bit_EN"]
pub type Tuning8bitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING_1bit_EN` reader - TUNING_1bit_EN"]
pub type Tuning1bitEnR = crate::BitReader;
#[doc = "Field `TUNING_1bit_EN` writer - TUNING_1bit_EN"]
pub type Tuning1bitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TUNING_CMD_EN\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TuningCmdEn {
    #[doc = "0: Auto tuning circuit does not check the CMD line."]
    TuningCmdEn0 = 0,
    #[doc = "1: Auto tuning circuit checks the CMD line."]
    TuningCmdEn1 = 1,
}
impl From<TuningCmdEn> for bool {
    #[inline(always)]
    fn from(variant: TuningCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUNING_CMD_EN` reader - TUNING_CMD_EN"]
pub type TuningCmdEnR = crate::BitReader<TuningCmdEn>;
impl TuningCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TuningCmdEn {
        match self.bits {
            false => TuningCmdEn::TuningCmdEn0,
            true => TuningCmdEn::TuningCmdEn1,
        }
    }
    #[doc = "Auto tuning circuit does not check the CMD line."]
    #[inline(always)]
    pub fn is_tuning_cmd_en_0(&self) -> bool {
        *self == TuningCmdEn::TuningCmdEn0
    }
    #[doc = "Auto tuning circuit checks the CMD line."]
    #[inline(always)]
    pub fn is_tuning_cmd_en_1(&self) -> bool {
        *self == TuningCmdEn::TuningCmdEn1
    }
}
#[doc = "Field `TUNING_CMD_EN` writer - TUNING_CMD_EN"]
pub type TuningCmdEnW<'a, REG> = crate::BitWriter<'a, REG, TuningCmdEn>;
impl<'a, REG> TuningCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto tuning circuit does not check the CMD line."]
    #[inline(always)]
    pub fn tuning_cmd_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(TuningCmdEn::TuningCmdEn0)
    }
    #[doc = "Auto tuning circuit checks the CMD line."]
    #[inline(always)]
    pub fn tuning_cmd_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(TuningCmdEn::TuningCmdEn1)
    }
}
#[doc = "Field `HS400_WR_CLK_STOP_EN` reader - HS400 Write Clock Stop Enable"]
pub type Hs400WrClkStopEnR = crate::BitReader;
#[doc = "Field `HS400_WR_CLK_STOP_EN` writer - HS400 Write Clock Stop Enable"]
pub type Hs400WrClkStopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS400_RD_CLK_STOP_EN` reader - HS400 Read Clock Stop Enable"]
pub type Hs400RdClkStopEnR = crate::BitReader;
#[doc = "Field `HS400_RD_CLK_STOP_EN` writer - HS400 Read Clock Stop Enable"]
pub type Hs400RdClkStopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Argument2 register enable for ACMD23\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmd23Argu2En {
    #[doc = "0: Disable"]
    Acmd23Argu2En0 = 0,
    #[doc = "1: Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    Acmd23Argu2En1 = 1,
}
impl From<Acmd23Argu2En> for bool {
    #[inline(always)]
    fn from(variant: Acmd23Argu2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD23_ARGU2_EN` reader - Argument2 register enable for ACMD23"]
pub type Acmd23Argu2EnR = crate::BitReader<Acmd23Argu2En>;
impl Acmd23Argu2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmd23Argu2En {
        match self.bits {
            false => Acmd23Argu2En::Acmd23Argu2En0,
            true => Acmd23Argu2En::Acmd23Argu2En1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_acmd23_argu2_en_0(&self) -> bool {
        *self == Acmd23Argu2En::Acmd23Argu2En0
    }
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    #[inline(always)]
    pub fn is_acmd23_argu2_en_1(&self) -> bool {
        *self == Acmd23Argu2En::Acmd23Argu2En1
    }
}
#[doc = "Field `ACMD23_ARGU2_EN` writer - Argument2 register enable for ACMD23"]
pub type Acmd23Argu2EnW<'a, REG> = crate::BitWriter<'a, REG, Acmd23Argu2En>;
impl<'a, REG> Acmd23Argu2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn acmd23_argu2_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Acmd23Argu2En::Acmd23Argu2En0)
    }
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    #[inline(always)]
    pub fn acmd23_argu2_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Acmd23Argu2En::Acmd23Argu2En1)
    }
}
#[doc = "Field `AHB_RST` reader - AHB BUS reset"]
pub type AhbRstR = crate::BitReader;
#[doc = "Field `AHB_RST` writer - AHB BUS reset"]
pub type AhbRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline(always)]
    pub fn card_int_d3_test(&self) -> CardIntD3TestR {
        CardIntD3TestR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TUNING_8bit_EN"]
    #[inline(always)]
    pub fn tuning_8bit_en(&self) -> Tuning8bitEnR {
        Tuning8bitEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TUNING_1bit_EN"]
    #[inline(always)]
    pub fn tuning_1bit_en(&self) -> Tuning1bitEnR {
        Tuning1bitEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TUNING_CMD_EN"]
    #[inline(always)]
    pub fn tuning_cmd_en(&self) -> TuningCmdEnR {
        TuningCmdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - HS400 Write Clock Stop Enable"]
    #[inline(always)]
    pub fn hs400_wr_clk_stop_en(&self) -> Hs400WrClkStopEnR {
        Hs400WrClkStopEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HS400 Read Clock Stop Enable"]
    #[inline(always)]
    pub fn hs400_rd_clk_stop_en(&self) -> Hs400RdClkStopEnR {
        Hs400RdClkStopEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub fn acmd23_argu2_en(&self) -> Acmd23Argu2EnR {
        Acmd23Argu2EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - AHB BUS reset"]
    #[inline(always)]
    pub fn ahb_rst(&self) -> AhbRstR {
        AhbRstR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VEND_SPEC2")
            .field("card_int_d3_test", &self.card_int_d3_test())
            .field("tuning_8bit_en", &self.tuning_8bit_en())
            .field("tuning_1bit_en", &self.tuning_1bit_en())
            .field("tuning_cmd_en", &self.tuning_cmd_en())
            .field("hs400_wr_clk_stop_en", &self.hs400_wr_clk_stop_en())
            .field("hs400_rd_clk_stop_en", &self.hs400_rd_clk_stop_en())
            .field("acmd23_argu2_en", &self.acmd23_argu2_en())
            .field("ahb_rst", &self.ahb_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline(always)]
    pub fn card_int_d3_test(&mut self) -> CardIntD3TestW<VendSpec2Spec> {
        CardIntD3TestW::new(self, 3)
    }
    #[doc = "Bit 4 - TUNING_8bit_EN"]
    #[inline(always)]
    pub fn tuning_8bit_en(&mut self) -> Tuning8bitEnW<VendSpec2Spec> {
        Tuning8bitEnW::new(self, 4)
    }
    #[doc = "Bit 5 - TUNING_1bit_EN"]
    #[inline(always)]
    pub fn tuning_1bit_en(&mut self) -> Tuning1bitEnW<VendSpec2Spec> {
        Tuning1bitEnW::new(self, 5)
    }
    #[doc = "Bit 6 - TUNING_CMD_EN"]
    #[inline(always)]
    pub fn tuning_cmd_en(&mut self) -> TuningCmdEnW<VendSpec2Spec> {
        TuningCmdEnW::new(self, 6)
    }
    #[doc = "Bit 10 - HS400 Write Clock Stop Enable"]
    #[inline(always)]
    pub fn hs400_wr_clk_stop_en(&mut self) -> Hs400WrClkStopEnW<VendSpec2Spec> {
        Hs400WrClkStopEnW::new(self, 10)
    }
    #[doc = "Bit 11 - HS400 Read Clock Stop Enable"]
    #[inline(always)]
    pub fn hs400_rd_clk_stop_en(&mut self) -> Hs400RdClkStopEnW<VendSpec2Spec> {
        Hs400RdClkStopEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub fn acmd23_argu2_en(&mut self) -> Acmd23Argu2EnW<VendSpec2Spec> {
        Acmd23Argu2EnW::new(self, 12)
    }
    #[doc = "Bit 14 - AHB BUS reset"]
    #[inline(always)]
    pub fn ahb_rst(&mut self) -> AhbRstW<VendSpec2Spec> {
        AhbRstW::new(self, 14)
    }
}
#[doc = "Vendor Specific 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vend_spec2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vend_spec2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendSpec2Spec;
impl crate::RegisterSpec for VendSpec2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vend_spec2::R`](R) reader structure"]
impl crate::Readable for VendSpec2Spec {}
#[doc = "`write(|w| ..)` method takes [`vend_spec2::W`](W) writer structure"]
impl crate::Writable for VendSpec2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VEND_SPEC2 to value 0x1006"]
impl crate::Resettable for VendSpec2Spec {
    const RESET_VALUE: u32 = 0x1006;
}
