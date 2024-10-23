#[doc = "Register `DMAC0_ITRIG_SEL[%s]` reader"]
pub type R = crate::R<Dmac0ItrigSelSpec>;
#[doc = "Register `DMAC0_ITRIG_SEL[%s]` writer"]
pub type W = crate::W<Dmac0ItrigSelSpec>;
#[doc = "DMA Input Triggers(n) Selection. 22:1 Selection for each. . .\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma0ItrigSel {
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
    #[doc = "14: DMAC0_TRIGOUT_A"]
    Dmac0TrigoutA = 14,
    #[doc = "15: DMAC0_TRIGOUT_B"]
    Dmac0TrigoutB = 15,
    #[doc = "16: DMAC0_TRIGOUT_C"]
    Dmac0TrigoutC = 16,
    #[doc = "17: DMAC0_TRIGOUT_D"]
    Dmac0TrigoutD = 17,
    #[doc = "18: SCT0_DMA0"]
    Sct0Dma0 = 18,
    #[doc = "19: SCT0_DMA1"]
    Sct0Dma1 = 19,
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
impl From<Dma0ItrigSel> for u8 {
    #[inline(always)]
    fn from(variant: Dma0ItrigSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma0ItrigSel {
    type Ux = u8;
}
impl crate::IsEnum for Dma0ItrigSel {}
#[doc = "Field `DMA0_ITRIG_SEL` reader - DMA Input Triggers(n) Selection. 22:1 Selection for each. . ."]
pub type Dma0ItrigSelR = crate::FieldReader<Dma0ItrigSel>;
impl Dma0ItrigSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dma0ItrigSel> {
        match self.bits {
            0 => Some(Dma0ItrigSel::Nsgpiopint0Int0),
            1 => Some(Dma0ItrigSel::Nsgpiopint0Int1),
            2 => Some(Dma0ItrigSel::Nsgpiopint0Int2),
            3 => Some(Dma0ItrigSel::Nsgpiopint0Int3),
            4 => Some(Dma0ItrigSel::Ct32bit0DmareqM0),
            5 => Some(Dma0ItrigSel::Ct32bit0DmareqM1),
            6 => Some(Dma0ItrigSel::Ct32bit1DmareqM0),
            7 => Some(Dma0ItrigSel::Ct32bit1DmareqM1),
            8 => Some(Dma0ItrigSel::Ct32bit2DmareqM0),
            9 => Some(Dma0ItrigSel::Ct32bit2DmareqM1),
            10 => Some(Dma0ItrigSel::Ct32bit3DmareqM0),
            11 => Some(Dma0ItrigSel::Ct32bit3DmareqM1),
            12 => Some(Dma0ItrigSel::Ct32bit4DmareqM0),
            13 => Some(Dma0ItrigSel::Ct32bit4DmareqM1),
            14 => Some(Dma0ItrigSel::Dmac0TrigoutA),
            15 => Some(Dma0ItrigSel::Dmac0TrigoutB),
            16 => Some(Dma0ItrigSel::Dmac0TrigoutC),
            17 => Some(Dma0ItrigSel::Dmac0TrigoutD),
            18 => Some(Dma0ItrigSel::Sct0Dma0),
            19 => Some(Dma0ItrigSel::Sct0Dma1),
            20 => Some(Dma0ItrigSel::HashcryptOutDma),
            21 => Some(Dma0ItrigSel::AcmpDma),
            24 => Some(Dma0ItrigSel::AdcDmac),
            28 => Some(Dma0ItrigSel::FlexspiRx),
            29 => Some(Dma0ItrigSel::FlexspiTx),
            _ => None,
        }
    }
    #[doc = "NSGPIOPINT0_INT0"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int0(&self) -> bool {
        *self == Dma0ItrigSel::Nsgpiopint0Int0
    }
    #[doc = "NSGPIOPINT0_INT1"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int1(&self) -> bool {
        *self == Dma0ItrigSel::Nsgpiopint0Int1
    }
    #[doc = "NSGPIOPINT0_INT2"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int2(&self) -> bool {
        *self == Dma0ItrigSel::Nsgpiopint0Int2
    }
    #[doc = "NSGPIOPINT0_INT3"]
    #[inline(always)]
    pub fn is_nsgpiopint0_int3(&self) -> bool {
        *self == Dma0ItrigSel::Nsgpiopint0Int3
    }
    #[doc = "CT32BIT0_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit0_dmareq_m0(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit0DmareqM0
    }
    #[doc = "CT32BIT0_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit0_dmareq_m1(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit0DmareqM1
    }
    #[doc = "CT32BIT1_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit1_dmareq_m0(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit1DmareqM0
    }
    #[doc = "CT32BIT1_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit1_dmareq_m1(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit1DmareqM1
    }
    #[doc = "CT32BIT2_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit2_dmareq_m0(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit2DmareqM0
    }
    #[doc = "CT32BIT2_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit2_dmareq_m1(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit2DmareqM1
    }
    #[doc = "CT32BIT3_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit3_dmareq_m0(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit3DmareqM0
    }
    #[doc = "CT32BIT3_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit3_dmareq_m1(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit3DmareqM1
    }
    #[doc = "CT32BIT4_DMAREQ_M0"]
    #[inline(always)]
    pub fn is_ct32bit4_dmareq_m0(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit4DmareqM0
    }
    #[doc = "CT32BIT4_DMAREQ_M1"]
    #[inline(always)]
    pub fn is_ct32bit4_dmareq_m1(&self) -> bool {
        *self == Dma0ItrigSel::Ct32bit4DmareqM1
    }
    #[doc = "DMAC0_TRIGOUT_A"]
    #[inline(always)]
    pub fn is_dmac0_trigout_a(&self) -> bool {
        *self == Dma0ItrigSel::Dmac0TrigoutA
    }
    #[doc = "DMAC0_TRIGOUT_B"]
    #[inline(always)]
    pub fn is_dmac0_trigout_b(&self) -> bool {
        *self == Dma0ItrigSel::Dmac0TrigoutB
    }
    #[doc = "DMAC0_TRIGOUT_C"]
    #[inline(always)]
    pub fn is_dmac0_trigout_c(&self) -> bool {
        *self == Dma0ItrigSel::Dmac0TrigoutC
    }
    #[doc = "DMAC0_TRIGOUT_D"]
    #[inline(always)]
    pub fn is_dmac0_trigout_d(&self) -> bool {
        *self == Dma0ItrigSel::Dmac0TrigoutD
    }
    #[doc = "SCT0_DMA0"]
    #[inline(always)]
    pub fn is_sct0_dma0(&self) -> bool {
        *self == Dma0ItrigSel::Sct0Dma0
    }
    #[doc = "SCT0_DMA1"]
    #[inline(always)]
    pub fn is_sct0_dma1(&self) -> bool {
        *self == Dma0ItrigSel::Sct0Dma1
    }
    #[doc = "HASHCRYPT_OUT_DMA"]
    #[inline(always)]
    pub fn is_hashcrypt_out_dma(&self) -> bool {
        *self == Dma0ItrigSel::HashcryptOutDma
    }
    #[doc = "ACMP_DMA"]
    #[inline(always)]
    pub fn is_acmp_dma(&self) -> bool {
        *self == Dma0ItrigSel::AcmpDma
    }
    #[doc = "ADC_DMAC"]
    #[inline(always)]
    pub fn is_adc_dmac(&self) -> bool {
        *self == Dma0ItrigSel::AdcDmac
    }
    #[doc = "FLEXSPI_RX"]
    #[inline(always)]
    pub fn is_flexspi_rx(&self) -> bool {
        *self == Dma0ItrigSel::FlexspiRx
    }
    #[doc = "FLEXSPI_TX"]
    #[inline(always)]
    pub fn is_flexspi_tx(&self) -> bool {
        *self == Dma0ItrigSel::FlexspiTx
    }
}
#[doc = "Field `DMA0_ITRIG_SEL` writer - DMA Input Triggers(n) Selection. 22:1 Selection for each. . ."]
pub type Dma0ItrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dma0ItrigSel>;
impl<'a, REG> Dma0ItrigSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NSGPIOPINT0_INT0"]
    #[inline(always)]
    pub fn nsgpiopint0_int0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Nsgpiopint0Int0)
    }
    #[doc = "NSGPIOPINT0_INT1"]
    #[inline(always)]
    pub fn nsgpiopint0_int1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Nsgpiopint0Int1)
    }
    #[doc = "NSGPIOPINT0_INT2"]
    #[inline(always)]
    pub fn nsgpiopint0_int2(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Nsgpiopint0Int2)
    }
    #[doc = "NSGPIOPINT0_INT3"]
    #[inline(always)]
    pub fn nsgpiopint0_int3(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Nsgpiopint0Int3)
    }
    #[doc = "CT32BIT0_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit0_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit0DmareqM0)
    }
    #[doc = "CT32BIT0_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit0_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit0DmareqM1)
    }
    #[doc = "CT32BIT1_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit1_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit1DmareqM0)
    }
    #[doc = "CT32BIT1_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit1_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit1DmareqM1)
    }
    #[doc = "CT32BIT2_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit2_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit2DmareqM0)
    }
    #[doc = "CT32BIT2_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit2_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit2DmareqM1)
    }
    #[doc = "CT32BIT3_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit3_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit3DmareqM0)
    }
    #[doc = "CT32BIT3_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit3_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit3DmareqM1)
    }
    #[doc = "CT32BIT4_DMAREQ_M0"]
    #[inline(always)]
    pub fn ct32bit4_dmareq_m0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit4DmareqM0)
    }
    #[doc = "CT32BIT4_DMAREQ_M1"]
    #[inline(always)]
    pub fn ct32bit4_dmareq_m1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Ct32bit4DmareqM1)
    }
    #[doc = "DMAC0_TRIGOUT_A"]
    #[inline(always)]
    pub fn dmac0_trigout_a(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Dmac0TrigoutA)
    }
    #[doc = "DMAC0_TRIGOUT_B"]
    #[inline(always)]
    pub fn dmac0_trigout_b(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Dmac0TrigoutB)
    }
    #[doc = "DMAC0_TRIGOUT_C"]
    #[inline(always)]
    pub fn dmac0_trigout_c(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Dmac0TrigoutC)
    }
    #[doc = "DMAC0_TRIGOUT_D"]
    #[inline(always)]
    pub fn dmac0_trigout_d(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Dmac0TrigoutD)
    }
    #[doc = "SCT0_DMA0"]
    #[inline(always)]
    pub fn sct0_dma0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Sct0Dma0)
    }
    #[doc = "SCT0_DMA1"]
    #[inline(always)]
    pub fn sct0_dma1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::Sct0Dma1)
    }
    #[doc = "HASHCRYPT_OUT_DMA"]
    #[inline(always)]
    pub fn hashcrypt_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::HashcryptOutDma)
    }
    #[doc = "ACMP_DMA"]
    #[inline(always)]
    pub fn acmp_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::AcmpDma)
    }
    #[doc = "ADC_DMAC"]
    #[inline(always)]
    pub fn adc_dmac(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::AdcDmac)
    }
    #[doc = "FLEXSPI_RX"]
    #[inline(always)]
    pub fn flexspi_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::FlexspiRx)
    }
    #[doc = "FLEXSPI_TX"]
    #[inline(always)]
    pub fn flexspi_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Dma0ItrigSel::FlexspiTx)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA Input Triggers(n) Selection. 22:1 Selection for each. . ."]
    #[inline(always)]
    pub fn dma0_itrig_sel(&self) -> Dma0ItrigSelR {
        Dma0ItrigSelR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0_ITRIG_SEL")
            .field("dma0_itrig_sel", &self.dma0_itrig_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA Input Triggers(n) Selection. 22:1 Selection for each. . ."]
    #[inline(always)]
    #[must_use]
    pub fn dma0_itrig_sel(&mut self) -> Dma0ItrigSelW<Dmac0ItrigSelSpec> {
        Dma0ItrigSelW::new(self, 0)
    }
}
#[doc = "DMAC0 Input Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_itrig_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_itrig_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac0ItrigSelSpec;
impl crate::RegisterSpec for Dmac0ItrigSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac0_itrig_sel::R`](R) reader structure"]
impl crate::Readable for Dmac0ItrigSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac0_itrig_sel::W`](W) writer structure"]
impl crate::Writable for Dmac0ItrigSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC0_ITRIG_SEL[%s]
to value 0x1f"]
impl crate::Resettable for Dmac0ItrigSelSpec {
    const RESET_VALUE: u32 = 0x1f;
}
