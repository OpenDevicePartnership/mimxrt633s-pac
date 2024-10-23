#[doc = "Register `SCT0_IN_SEL%s` reader"]
pub type R = crate::R<Sct0InSelSpec>;
#[doc = "Register `SCT0_IN_SEL%s` writer"]
pub type W = crate::W<Sct0InSelSpec>;
#[doc = "SCT0 Input(n) Selection. 24:1 Selection for each. . .\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SctInSel {
    #[doc = "0: SCT0_PIN_INP0"]
    Sct0PinInp0 = 0,
    #[doc = "1: SCT0_PIN_INP1"]
    Sct0PinInp1 = 1,
    #[doc = "2: SCT0_PIN_INP2"]
    Sct0PinInp2 = 2,
    #[doc = "3: SCT0_PIN_INP3"]
    Sct0PinInp3 = 3,
    #[doc = "4: SCT0_PIN_INP4"]
    Sct0PinInp4 = 4,
    #[doc = "5: SCT0_PIN_INP5"]
    Sct0PinInp5 = 5,
    #[doc = "6: SCT0_PIN_INP6"]
    Sct0PinInp6 = 6,
    #[doc = "7: SCT0_PIN_INP7"]
    Sct0PinInp7 = 7,
    #[doc = "8: CT32BIT0_MAT0"]
    Ct32bit0Mat0 = 8,
    #[doc = "9: CT32BIT1_MAT0"]
    Ct32bit1Mat0 = 9,
    #[doc = "10: CT32BIT2_MAT0"]
    Ct32bit2Mat0 = 10,
    #[doc = "11: CT32BIT3_MAT0"]
    Ct32bit3Mat0 = 11,
    #[doc = "12: CT32BIT4_MAT0"]
    Ct32bit4Mat0 = 12,
    #[doc = "13: ADCIRQ"]
    Adcirq = 13,
    #[doc = "14: GPIOINT_BMATCH"]
    GpiointBmatch = 14,
    #[doc = "15: USB1_FRAME_TOGGLE"]
    Usb1FrameToggle = 15,
    #[doc = "16: CMP0_OUT"]
    Cmp0Out = 16,
    #[doc = "17: SHARED I2S0_SCLK"]
    SharedI2s0Sclk = 17,
    #[doc = "18: SHARED I2S1_SCLK"]
    SharedI2s1Sclk = 18,
    #[doc = "19: SHARED I2S0_WS"]
    SharedI2s0Ws = 19,
    #[doc = "20: SHARED I2S1_WS"]
    SharedI2s1Ws = 20,
    #[doc = "21: MCLK"]
    Mclk = 21,
    #[doc = "22: ARM_TXEV"]
    ArmTxev = 22,
    #[doc = "23: DEBUG_HALTED"]
    DebugHalted = 23,
}
impl From<SctInSel> for u8 {
    #[inline(always)]
    fn from(variant: SctInSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SctInSel {
    type Ux = u8;
}
impl crate::IsEnum for SctInSel {}
#[doc = "Field `SCT_IN_SEL` reader - SCT0 Input(n) Selection. 24:1 Selection for each. . ."]
pub type SctInSelR = crate::FieldReader<SctInSel>;
impl SctInSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SctInSel> {
        match self.bits {
            0 => Some(SctInSel::Sct0PinInp0),
            1 => Some(SctInSel::Sct0PinInp1),
            2 => Some(SctInSel::Sct0PinInp2),
            3 => Some(SctInSel::Sct0PinInp3),
            4 => Some(SctInSel::Sct0PinInp4),
            5 => Some(SctInSel::Sct0PinInp5),
            6 => Some(SctInSel::Sct0PinInp6),
            7 => Some(SctInSel::Sct0PinInp7),
            8 => Some(SctInSel::Ct32bit0Mat0),
            9 => Some(SctInSel::Ct32bit1Mat0),
            10 => Some(SctInSel::Ct32bit2Mat0),
            11 => Some(SctInSel::Ct32bit3Mat0),
            12 => Some(SctInSel::Ct32bit4Mat0),
            13 => Some(SctInSel::Adcirq),
            14 => Some(SctInSel::GpiointBmatch),
            15 => Some(SctInSel::Usb1FrameToggle),
            16 => Some(SctInSel::Cmp0Out),
            17 => Some(SctInSel::SharedI2s0Sclk),
            18 => Some(SctInSel::SharedI2s1Sclk),
            19 => Some(SctInSel::SharedI2s0Ws),
            20 => Some(SctInSel::SharedI2s1Ws),
            21 => Some(SctInSel::Mclk),
            22 => Some(SctInSel::ArmTxev),
            23 => Some(SctInSel::DebugHalted),
            _ => None,
        }
    }
    #[doc = "SCT0_PIN_INP0"]
    #[inline(always)]
    pub fn is_sct0_pin_inp0(&self) -> bool {
        *self == SctInSel::Sct0PinInp0
    }
    #[doc = "SCT0_PIN_INP1"]
    #[inline(always)]
    pub fn is_sct0_pin_inp1(&self) -> bool {
        *self == SctInSel::Sct0PinInp1
    }
    #[doc = "SCT0_PIN_INP2"]
    #[inline(always)]
    pub fn is_sct0_pin_inp2(&self) -> bool {
        *self == SctInSel::Sct0PinInp2
    }
    #[doc = "SCT0_PIN_INP3"]
    #[inline(always)]
    pub fn is_sct0_pin_inp3(&self) -> bool {
        *self == SctInSel::Sct0PinInp3
    }
    #[doc = "SCT0_PIN_INP4"]
    #[inline(always)]
    pub fn is_sct0_pin_inp4(&self) -> bool {
        *self == SctInSel::Sct0PinInp4
    }
    #[doc = "SCT0_PIN_INP5"]
    #[inline(always)]
    pub fn is_sct0_pin_inp5(&self) -> bool {
        *self == SctInSel::Sct0PinInp5
    }
    #[doc = "SCT0_PIN_INP6"]
    #[inline(always)]
    pub fn is_sct0_pin_inp6(&self) -> bool {
        *self == SctInSel::Sct0PinInp6
    }
    #[doc = "SCT0_PIN_INP7"]
    #[inline(always)]
    pub fn is_sct0_pin_inp7(&self) -> bool {
        *self == SctInSel::Sct0PinInp7
    }
    #[doc = "CT32BIT0_MAT0"]
    #[inline(always)]
    pub fn is_ct32bit0_mat0(&self) -> bool {
        *self == SctInSel::Ct32bit0Mat0
    }
    #[doc = "CT32BIT1_MAT0"]
    #[inline(always)]
    pub fn is_ct32bit1_mat0(&self) -> bool {
        *self == SctInSel::Ct32bit1Mat0
    }
    #[doc = "CT32BIT2_MAT0"]
    #[inline(always)]
    pub fn is_ct32bit2_mat0(&self) -> bool {
        *self == SctInSel::Ct32bit2Mat0
    }
    #[doc = "CT32BIT3_MAT0"]
    #[inline(always)]
    pub fn is_ct32bit3_mat0(&self) -> bool {
        *self == SctInSel::Ct32bit3Mat0
    }
    #[doc = "CT32BIT4_MAT0"]
    #[inline(always)]
    pub fn is_ct32bit4_mat0(&self) -> bool {
        *self == SctInSel::Ct32bit4Mat0
    }
    #[doc = "ADCIRQ"]
    #[inline(always)]
    pub fn is_adcirq(&self) -> bool {
        *self == SctInSel::Adcirq
    }
    #[doc = "GPIOINT_BMATCH"]
    #[inline(always)]
    pub fn is_gpioint_bmatch(&self) -> bool {
        *self == SctInSel::GpiointBmatch
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn is_usb1_frame_toggle(&self) -> bool {
        *self == SctInSel::Usb1FrameToggle
    }
    #[doc = "CMP0_OUT"]
    #[inline(always)]
    pub fn is_cmp0_out(&self) -> bool {
        *self == SctInSel::Cmp0Out
    }
    #[doc = "SHARED I2S0_SCLK"]
    #[inline(always)]
    pub fn is_shared_i2s0_sclk(&self) -> bool {
        *self == SctInSel::SharedI2s0Sclk
    }
    #[doc = "SHARED I2S1_SCLK"]
    #[inline(always)]
    pub fn is_shared_i2s1_sclk(&self) -> bool {
        *self == SctInSel::SharedI2s1Sclk
    }
    #[doc = "SHARED I2S0_WS"]
    #[inline(always)]
    pub fn is_shared_i2s0_ws(&self) -> bool {
        *self == SctInSel::SharedI2s0Ws
    }
    #[doc = "SHARED I2S1_WS"]
    #[inline(always)]
    pub fn is_shared_i2s1_ws(&self) -> bool {
        *self == SctInSel::SharedI2s1Ws
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn is_mclk(&self) -> bool {
        *self == SctInSel::Mclk
    }
    #[doc = "ARM_TXEV"]
    #[inline(always)]
    pub fn is_arm_txev(&self) -> bool {
        *self == SctInSel::ArmTxev
    }
    #[doc = "DEBUG_HALTED"]
    #[inline(always)]
    pub fn is_debug_halted(&self) -> bool {
        *self == SctInSel::DebugHalted
    }
}
#[doc = "Field `SCT_IN_SEL` writer - SCT0 Input(n) Selection. 24:1 Selection for each. . ."]
pub type SctInSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, SctInSel>;
impl<'a, REG> SctInSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCT0_PIN_INP0"]
    #[inline(always)]
    pub fn sct0_pin_inp0(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp0)
    }
    #[doc = "SCT0_PIN_INP1"]
    #[inline(always)]
    pub fn sct0_pin_inp1(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp1)
    }
    #[doc = "SCT0_PIN_INP2"]
    #[inline(always)]
    pub fn sct0_pin_inp2(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp2)
    }
    #[doc = "SCT0_PIN_INP3"]
    #[inline(always)]
    pub fn sct0_pin_inp3(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp3)
    }
    #[doc = "SCT0_PIN_INP4"]
    #[inline(always)]
    pub fn sct0_pin_inp4(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp4)
    }
    #[doc = "SCT0_PIN_INP5"]
    #[inline(always)]
    pub fn sct0_pin_inp5(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp5)
    }
    #[doc = "SCT0_PIN_INP6"]
    #[inline(always)]
    pub fn sct0_pin_inp6(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp6)
    }
    #[doc = "SCT0_PIN_INP7"]
    #[inline(always)]
    pub fn sct0_pin_inp7(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Sct0PinInp7)
    }
    #[doc = "CT32BIT0_MAT0"]
    #[inline(always)]
    pub fn ct32bit0_mat0(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Ct32bit0Mat0)
    }
    #[doc = "CT32BIT1_MAT0"]
    #[inline(always)]
    pub fn ct32bit1_mat0(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Ct32bit1Mat0)
    }
    #[doc = "CT32BIT2_MAT0"]
    #[inline(always)]
    pub fn ct32bit2_mat0(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Ct32bit2Mat0)
    }
    #[doc = "CT32BIT3_MAT0"]
    #[inline(always)]
    pub fn ct32bit3_mat0(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Ct32bit3Mat0)
    }
    #[doc = "CT32BIT4_MAT0"]
    #[inline(always)]
    pub fn ct32bit4_mat0(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Ct32bit4Mat0)
    }
    #[doc = "ADCIRQ"]
    #[inline(always)]
    pub fn adcirq(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Adcirq)
    }
    #[doc = "GPIOINT_BMATCH"]
    #[inline(always)]
    pub fn gpioint_bmatch(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::GpiointBmatch)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn usb1_frame_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Usb1FrameToggle)
    }
    #[doc = "CMP0_OUT"]
    #[inline(always)]
    pub fn cmp0_out(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Cmp0Out)
    }
    #[doc = "SHARED I2S0_SCLK"]
    #[inline(always)]
    pub fn shared_i2s0_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::SharedI2s0Sclk)
    }
    #[doc = "SHARED I2S1_SCLK"]
    #[inline(always)]
    pub fn shared_i2s1_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::SharedI2s1Sclk)
    }
    #[doc = "SHARED I2S0_WS"]
    #[inline(always)]
    pub fn shared_i2s0_ws(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::SharedI2s0Ws)
    }
    #[doc = "SHARED I2S1_WS"]
    #[inline(always)]
    pub fn shared_i2s1_ws(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::SharedI2s1Ws)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn mclk(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::Mclk)
    }
    #[doc = "ARM_TXEV"]
    #[inline(always)]
    pub fn arm_txev(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::ArmTxev)
    }
    #[doc = "DEBUG_HALTED"]
    #[inline(always)]
    pub fn debug_halted(self) -> &'a mut crate::W<REG> {
        self.variant(SctInSel::DebugHalted)
    }
}
impl R {
    #[doc = "Bits 0:4 - SCT0 Input(n) Selection. 24:1 Selection for each. . ."]
    #[inline(always)]
    pub fn sct_in_sel(&self) -> SctInSelR {
        SctInSelR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCT0_IN_SEL")
            .field("sct_in_sel", &self.sct_in_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - SCT0 Input(n) Selection. 24:1 Selection for each. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sct_in_sel(&mut self) -> SctInSelW<Sct0InSelSpec> {
        SctInSelW::new(self, 0)
    }
}
#[doc = "SCT Peripheral Input Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`sct0_in_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sct0_in_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sct0InSelSpec;
impl crate::RegisterSpec for Sct0InSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sct0_in_sel::R`](R) reader structure"]
impl crate::Readable for Sct0InSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sct0_in_sel::W`](W) writer structure"]
impl crate::Writable for Sct0InSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCT0_IN_SEL%s to value 0x1f"]
impl crate::Resettable for Sct0InSelSpec {
    const RESET_VALUE: u32 = 0x1f;
}
