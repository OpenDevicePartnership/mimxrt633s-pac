#[doc = "Register `DMAC1_ITRIG_SEL[%s]` reader"]
pub type R = crate::R<Dmac1ItrigSelSpec>;
#[doc = "Register `DMAC1_ITRIG_SEL[%s]` writer"]
pub type W = crate::W<Dmac1ItrigSelSpec>;
#[doc = "DMA Input Triggers(n) Selection. 18:1 Selection for each. . .\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma1ItrigSel {
    #[doc = "0: NSGPIOPINT0_INT0"]
    Nsgpiopint0Int0 = 0,
    #[doc = "1: NSGPIOPINT0_INT1"]
    Nsgpiopint0Int1 = 1,
    #[doc = "2: NSGPIOPINT0_INT2"]
    Nsgpiopint0Int2 = 2,
    #[doc = "3: NSGPIOPINT0_INT3"]
    Nsgpiopint0Int3 = 3,
    #[doc = "4: CT32BIT0_DMAREQ_M0"]
    Ct32bit0DmareqM0 = 4,
    #[doc = "5: CT32BIT0_DMAREQ_M1"]
    Ct32bit0DmareqM1 = 5,
    #[doc = "6: CT32BIT1_DMAREQ_M0"]
    Ct32bit1DmareqM0 = 6,
    #[doc = "7: CT32BIT1_DMAREQ_M1"]
    Ct32bit1DmareqM1 = 7,
    #[doc = "8: CT32BIT2_DMAREQ_M0"]
    Ct32bit2DmareqM0 = 8,
    #[doc = "9: CT32BIT2_DMAREQ_M1"]
    Ct32bit2DmareqM1 = 9,
    #[doc = "10: CT32BIT3_DMAREQ_M0"]
    Ct32bit3DmareqM0 = 10,
    #[doc = "11: CT32BIT3_DMAREQ_M1"]
    Ct32bit3DmareqM1 = 11,
    #[doc = "12: CT32BIT4_DMAREQ_M0"]
    Ct32bit4DmareqM0 = 12,
    #[doc = "13: CT32BIT4_DMAREQ_M1"]
    Ct32bit4DmareqM1 = 13,
    #[doc = "14: DMAC1_TRIGOUT_A"]
    Dmac1TrigoutA = 14,
    #[doc = "15: DMAC1_TRIGOUT_B"]
    Dmac1TrigoutB = 15,
    #[doc = "16: DMAC1_TRIGOUT_C"]
    Dmac1TrigoutC = 16,
    #[doc = "17: DMAC0_TRIGOUT_D"]
    Dmac1TrigoutD = 17,
    #[doc = "18: SCT0_DMAC0"]
    Sct0Dmac0 = 18,
    #[doc = "19: SCT0_DMAC1"]
    Sct0Dmac1 = 19,
    #[doc = "20: HASHCRYPT_OUT_DMA"]
    HashcryptOutDma = 20,
    #[doc = "21: ACMP_DMA"]
    AcmpDma = 21,
    #[doc = "24: ADC_DMAC"]
    AdcDmac = 24,
    #[doc = "28: FLEXSPI_RX"]
    FlexspiRx = 28,
    #[doc = "29: FLEXSPI_TX"]
    FlexspiTx = 29,
}
impl From<Dma1ItrigSel> for u8 {
    #[inline(always)]
    fn from(variant: Dma1ItrigSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma1ItrigSel {
    type Ux = u8;
}
impl crate::IsEnum for Dma1ItrigSel {}
#[doc = "Field `DMA1_ITRIG_SEL` reader - DMA Input Triggers(n) Selection. 18:1 Selection for each. . ."]
pub type Dma1ItrigSelR = crate::FieldReader<Dma1ItrigSel>;
impl Dma1ItrigSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dma1ItrigSel> {
        match self.bits {
            0 => Some(Dma1ItrigSel::Nsgpiopint0Int0),
            1 => Some(Dma1ItrigSel::Nsgpiopint0Int1),
            2 => Some(Dma1ItrigSel::Nsgpiopint0Int2),
            3 => Some(Dma1ItrigSel::Nsgpiopint0Int3),
            4 => Some(Dma1ItrigSel::Ct32bit0DmareqM0),
            5 => Some(Dma1ItrigSel::Ct32bit0DmareqM1),
            6 => Some(Dma1ItrigSel::Ct32bit1DmareqM0),
            7 => Some(Dma1ItrigSel::Ct32bit1DmareqM1),
            8 => Some(Dma1ItrigSel::Ct32bit2DmareqM0),
            9 => Some(Dma1ItrigSel::Ct32bit2DmareqM1),
            10 => Some(Dma1ItrigSel::Ct32bit3DmareqM0),
            11 => Some(Dma1ItrigSel::Ct32bit3DmareqM1),
            12 => Some(Dma1ItrigSel::Ct32bit4DmareqM0),
            13 => Some(Dma1ItrigSel::Ct32bit4DmareqM1),
            14 => Some(Dma1ItrigSel::Dmac1TrigoutA),
            15 => Some(Dma1ItrigSel::Dmac1TrigoutB),
            16 => Some(Dma1ItrigSel::Dmac1TrigoutC),
            17 => Some(Dma1ItrigSel::Dmac1TrigoutD),
            18 => Some(Dma1ItrigSel::Sct0Dmac0),
            19 => Some(Dma1ItrigSel::Sct0Dmac1),
            20 => Some(Dma1ItrigSel::HashcryptOutDma),
            21 => Some(Dma1ItrigSel::AcmpDma),
            24 => Some(Dma1ItrigSel::AdcDmac),
            28 => Some(Dma1ItrigSel::FlexspiRx),
            29 => Some(Dma1ItrigSel::FlexspiTx),
            _ => None,
        }
    }
    #[doc = "NSGPIOPINT0_INT0"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int0(&self) -> bool {
        *self == Dma1ItrigSel::Nsgpiopint0Int0
    }
    #[doc = "NSGPIOPINT0_INT1"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int1(&self) -> bool {
        *self == Dma1ItrigSel::Nsgpiopint0Int1
    }
    #[doc = "NSGPIOPINT0_INT2"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int2(&self) -> bool {
        *self == Dma1ItrigSel::Nsgpiopint0Int2
    }
    #[doc = "NSGPIOPINT0_INT3"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int3(&self) -> bool {
        *self == Dma1ItrigSel::Nsgpiopint0Int3
    }
    #[doc = "CT32BIT0_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit0_dmareq_m0(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit0DmareqM0
    }
    #[doc = "CT32BIT0_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit0_dmareq_m1(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit0DmareqM1
    }
    #[doc = "CT32BIT1_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit1_dmareq_m0(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit1DmareqM0
    }
    #[doc = "CT32BIT1_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit1_dmareq_m1(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit1DmareqM1
    }
    #[doc = "CT32BIT2_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit2_dmareq_m0(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit2DmareqM0
    }
    #[doc = "CT32BIT2_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit2_dmareq_m1(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit2DmareqM1
    }
    #[doc = "CT32BIT3_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit3_dmareq_m0(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit3DmareqM0
    }
    #[doc = "CT32BIT3_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit3_dmareq_m1(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit3DmareqM1
    }
    #[doc = "CT32BIT4_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit4_dmareq_m0(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit4DmareqM0
    }
    #[doc = "CT32BIT4_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit4_dmareq_m1(&self) -> bool {
        *self == Dma1ItrigSel::Ct32bit4DmareqM1
    }
    #[doc = "DMAC1_TRIGOUT_A"]
    #[inline(always)]
    pub fn is_dmac1_trigout_a(&self) -> bool {
        *self == Dma1ItrigSel::Dmac1TrigoutA
    }
    #[doc = "DMAC1_TRIGOUT_B"]
    #[inline(always)]
    pub fn is_dmac1_trigout_b(&self) -> bool {
        *self == Dma1ItrigSel::Dmac1TrigoutB
    }
    #[doc = "DMAC1_TRIGOUT_C"]
    #[inline(always)]
    pub fn is_dmac1_trigout_c(&self) -> bool {
        *self == Dma1ItrigSel::Dmac1TrigoutC
    }
    #[doc = "DMAC0_TRIGOUT_D"]
    #[inline(always)]
    pub fn is_dmac1_trigout_d(&self) -> bool {
        *self == Dma1ItrigSel::Dmac1TrigoutD
    }
    #[doc = "SCT0_DMAC0"]
    #[inline(always)]
    pub fn is_sct0_dmac0(&self) -> bool {
        *self == Dma1ItrigSel::Sct0Dmac0
    }
    #[doc = "SCT0_DMAC1"]
    #[inline(always)]
    pub fn is_sct0_dmac1(&self) -> bool {
        *self == Dma1ItrigSel::Sct0Dmac1
    }
    #[doc = "HASHCRYPT_OUT_DMA"]
    #[inline(always)]
    pub fn is_hashcrypt_out_dma(&self) -> bool {
        *self == Dma1ItrigSel::HashcryptOutDma
    }
    #[doc = "ACMP_DMA"]
    #[inline(always)]
    pub fn is_acmp_dma(&self) -> bool {
        *self == Dma1ItrigSel::AcmpDma
    }
    #[doc = "ADC_DMAC"]
    #[inline(always)]
    pub fn is_adc_dmac(&self) -> bool {
        *self == Dma1ItrigSel::AdcDmac
    }
    #[doc = "FLEXSPI_RX"]
    #[inline(always)]
    pub fn is_flexspi_rx(&self) -> bool {
        *self == Dma1ItrigSel::FlexspiRx
    }
    #[doc = "FLEXSPI_TX"]
    #[inline(always)]
    pub fn is_flexspi_tx(&self) -> bool {
        *self == Dma1ItrigSel::FlexspiTx
    }
}
#[doc = "Field `DMA1_ITRIG_SEL` writer - DMA Input Triggers(n) Selection. 18:1 Selection for each. . ."]
pub type Dma1ItrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma1ItrigSel>;
impl<'a, REG> Dma1ItrigSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NSGPIOPINT0_INT0"]
    #[inline(always)]
    pub fn nsgpiopint0_int0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Nsgpiopint0Int0)
    }
    #[doc = "NSGPIOPINT0_INT1"]
    #[inline(always)]
    pub fn nsgpiopint0_int1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Nsgpiopint0Int1)
    }
    #[doc = "NSGPIOPINT0_INT2"]
    #[inline(always)]
    pub fn nsgpiopint0_int2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Nsgpiopint0Int2)
    }
    #[doc = "NSGPIOPINT0_INT3"]
    #[inline(always)]
    pub fn nsgpiopint0_int3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Nsgpiopint0Int3)
    }
    #[doc = "CT32BIT0_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit0_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit0DmareqM0)
    }
    #[doc = "CT32BIT0_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit0_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit0DmareqM1)
    }
    #[doc = "CT32BIT1_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit1_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit1DmareqM0)
    }
    #[doc = "CT32BIT1_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit1_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit1DmareqM1)
    }
    #[doc = "CT32BIT2_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit2_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit2DmareqM0)
    }
    #[doc = "CT32BIT2_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit2_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit2DmareqM1)
    }
    #[doc = "CT32BIT3_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit3_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit3DmareqM0)
    }
    #[doc = "CT32BIT3_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit3_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit3DmareqM1)
    }
    #[doc = "CT32BIT4_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit4_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit4DmareqM0)
    }
    #[doc = "CT32BIT4_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit4_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Ct32bit4DmareqM1)
    }
    #[doc = "DMAC1_TRIGOUT_A"]
    #[inline(always)]
    pub fn dmac1_trigout_a(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Dmac1TrigoutA)
    }
    #[doc = "DMAC1_TRIGOUT_B"]
    #[inline(always)]
    pub fn dmac1_trigout_b(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Dmac1TrigoutB)
    }
    #[doc = "DMAC1_TRIGOUT_C"]
    #[inline(always)]
    pub fn dmac1_trigout_c(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Dmac1TrigoutC)
    }
    #[doc = "DMAC0_TRIGOUT_D"]
    #[inline(always)]
    pub fn dmac1_trigout_d(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Dmac1TrigoutD)
    }
    #[doc = "SCT0_DMAC0"]
    #[inline(always)]
    pub fn sct0_dmac0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Sct0Dmac0)
    }
    #[doc = "SCT0_DMAC1"]
    #[inline(always)]
    pub fn sct0_dmac1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::Sct0Dmac1)
    }
    #[doc = "HASHCRYPT_OUT_DMA"]
    #[inline(always)]
    pub fn hashcrypt_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::HashcryptOutDma)
    }
    #[doc = "ACMP_DMA"]
    #[inline(always)]
    pub fn acmp_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::AcmpDma)
    }
    #[doc = "ADC_DMAC"]
    #[inline(always)]
    pub fn adc_dmac(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::AdcDmac)
    }
    #[doc = "FLEXSPI_RX"]
    #[inline(always)]
    pub fn flexspi_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::FlexspiRx)
    }
    #[doc = "FLEXSPI_TX"]
    #[inline(always)]
    pub fn flexspi_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1ItrigSel::FlexspiTx)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA Input Triggers(n) Selection. 18:1 Selection for each. . ."]
    #[inline(always)]
    pub fn dma1_itrig_sel(&self) -> Dma1ItrigSelR {
        Dma1ItrigSelR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1_ITRIG_SEL")
            .field("dma1_itrig_sel", &self.dma1_itrig_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA Input Triggers(n) Selection. 18:1 Selection for each. . ."]
    #[inline(always)]
    #[must_use]
    pub fn dma1_itrig_sel(&mut self) -> Dma1ItrigSelW<Dmac1ItrigSelSpec> {
        Dma1ItrigSelW::new(self, 0)
    }
}
#[doc = "DMAC1 Input Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_itrig_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_itrig_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac1ItrigSelSpec;
impl crate::RegisterSpec for Dmac1ItrigSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac1_itrig_sel::R`](R) reader structure"]
impl crate::Readable for Dmac1ItrigSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac1_itrig_sel::W`](W) writer structure"]
impl crate::Writable for Dmac1ItrigSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC1_ITRIG_SEL[%s]
to value 0x1f"]
impl crate::Resettable for Dmac1ItrigSelSpec {
    const RESET_VALUE: u32 = 0x1f;
}
