#[doc = "Register `CAP_SEL%s` reader"]
pub type R = crate::R<CapSelSpec>;
#[doc = "Register `CAP_SEL%s` writer"]
pub type W = crate::W<CapSelSpec>;
#[doc = "Counter Timer m, Capture Port Input n 19:1 Mux Select. . .\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CapnSel {
    #[doc = "0: CT_INP0"]
    CtInp0 = 0,
    #[doc = "1: CT_INP1"]
    CtInp1 = 1,
    #[doc = "2: CT_INP2"]
    CtInp2 = 2,
    #[doc = "3: CT_INP3"]
    CtInp3 = 3,
    #[doc = "4: CT_INP4"]
    CtInp4 = 4,
    #[doc = "5: CT_INP5"]
    CtInp5 = 5,
    #[doc = "6: CT_INP6"]
    CtInp6 = 6,
    #[doc = "7: CT_INP7"]
    CtInp7 = 7,
    #[doc = "8: CT_INP8"]
    CtInp8 = 8,
    #[doc = "9: CT_INP9"]
    CtInp9 = 9,
    #[doc = "10: CT_INP10"]
    CtInp10 = 10,
    #[doc = "11: CT_INP11"]
    CtInp11 = 11,
    #[doc = "12: CT_INP12"]
    CtInp12 = 12,
    #[doc = "13: CT_INP13"]
    CtInp13 = 13,
    #[doc = "14: CT_INP14"]
    CtInp14 = 14,
    #[doc = "15: CT_INP15"]
    CtInp15 = 15,
    #[doc = "16: SHARED I2S0_WS"]
    SharedI2s0Ws = 16,
    #[doc = "17: SHARED I2S1_WS"]
    SharedI2s1Ws = 17,
    #[doc = "18: USB1_FRAME_TOGGLE"]
    Usb1FrameToggle = 18,
}
impl From<CapnSel> for u8 {
    #[inline(always)]
    fn from(variant: CapnSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CapnSel {
    type Ux = u8;
}
impl crate::IsEnum for CapnSel {}
#[doc = "Field `CAPn_SEL` reader - Counter Timer m, Capture Port Input n 19:1 Mux Select. . ."]
pub type CapnSelR = crate::FieldReader<CapnSel>;
impl CapnSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CapnSel> {
        match self.bits {
            0 => Some(CapnSel::CtInp0),
            1 => Some(CapnSel::CtInp1),
            2 => Some(CapnSel::CtInp2),
            3 => Some(CapnSel::CtInp3),
            4 => Some(CapnSel::CtInp4),
            5 => Some(CapnSel::CtInp5),
            6 => Some(CapnSel::CtInp6),
            7 => Some(CapnSel::CtInp7),
            8 => Some(CapnSel::CtInp8),
            9 => Some(CapnSel::CtInp9),
            10 => Some(CapnSel::CtInp10),
            11 => Some(CapnSel::CtInp11),
            12 => Some(CapnSel::CtInp12),
            13 => Some(CapnSel::CtInp13),
            14 => Some(CapnSel::CtInp14),
            15 => Some(CapnSel::CtInp15),
            16 => Some(CapnSel::SharedI2s0Ws),
            17 => Some(CapnSel::SharedI2s1Ws),
            18 => Some(CapnSel::Usb1FrameToggle),
            _ => None,
        }
    }
    #[doc = "CT_INP0"]
    #[inline(always)]
    pub fn is_ct_inp0(&self) -> bool {
        *self == CapnSel::CtInp0
    }
    #[doc = "CT_INP1"]
    #[inline(always)]
    pub fn is_ct_inp1(&self) -> bool {
        *self == CapnSel::CtInp1
    }
    #[doc = "CT_INP2"]
    #[inline(always)]
    pub fn is_ct_inp2(&self) -> bool {
        *self == CapnSel::CtInp2
    }
    #[doc = "CT_INP3"]
    #[inline(always)]
    pub fn is_ct_inp3(&self) -> bool {
        *self == CapnSel::CtInp3
    }
    #[doc = "CT_INP4"]
    #[inline(always)]
    pub fn is_ct_inp4(&self) -> bool {
        *self == CapnSel::CtInp4
    }
    #[doc = "CT_INP5"]
    #[inline(always)]
    pub fn is_ct_inp5(&self) -> bool {
        *self == CapnSel::CtInp5
    }
    #[doc = "CT_INP6"]
    #[inline(always)]
    pub fn is_ct_inp6(&self) -> bool {
        *self == CapnSel::CtInp6
    }
    #[doc = "CT_INP7"]
    #[inline(always)]
    pub fn is_ct_inp7(&self) -> bool {
        *self == CapnSel::CtInp7
    }
    #[doc = "CT_INP8"]
    #[inline(always)]
    pub fn is_ct_inp8(&self) -> bool {
        *self == CapnSel::CtInp8
    }
    #[doc = "CT_INP9"]
    #[inline(always)]
    pub fn is_ct_inp9(&self) -> bool {
        *self == CapnSel::CtInp9
    }
    #[doc = "CT_INP10"]
    #[inline(always)]
    pub fn is_ct_inp10(&self) -> bool {
        *self == CapnSel::CtInp10
    }
    #[doc = "CT_INP11"]
    #[inline(always)]
    pub fn is_ct_inp11(&self) -> bool {
        *self == CapnSel::CtInp11
    }
    #[doc = "CT_INP12"]
    #[inline(always)]
    pub fn is_ct_inp12(&self) -> bool {
        *self == CapnSel::CtInp12
    }
    #[doc = "CT_INP13"]
    #[inline(always)]
    pub fn is_ct_inp13(&self) -> bool {
        *self == CapnSel::CtInp13
    }
    #[doc = "CT_INP14"]
    #[inline(always)]
    pub fn is_ct_inp14(&self) -> bool {
        *self == CapnSel::CtInp14
    }
    #[doc = "CT_INP15"]
    #[inline(always)]
    pub fn is_ct_inp15(&self) -> bool {
        *self == CapnSel::CtInp15
    }
    #[doc = "SHARED I2S0_WS"]
    #[inline(always)]
    pub fn is_shared_i2s0_ws(&self) -> bool {
        *self == CapnSel::SharedI2s0Ws
    }
    #[doc = "SHARED I2S1_WS"]
    #[inline(always)]
    pub fn is_shared_i2s1_ws(&self) -> bool {
        *self == CapnSel::SharedI2s1Ws
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn is_usb1_frame_toggle(&self) -> bool {
        *self == CapnSel::Usb1FrameToggle
    }
}
#[doc = "Field `CAPn_SEL` writer - Counter Timer m, Capture Port Input n 19:1 Mux Select. . ."]
pub type CapnSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, CapnSel>;
impl<'a, REG> CapnSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CT_INP0"]
    #[inline(always)]
    pub fn ct_inp0(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp0)
    }
    #[doc = "CT_INP1"]
    #[inline(always)]
    pub fn ct_inp1(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp1)
    }
    #[doc = "CT_INP2"]
    #[inline(always)]
    pub fn ct_inp2(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp2)
    }
    #[doc = "CT_INP3"]
    #[inline(always)]
    pub fn ct_inp3(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp3)
    }
    #[doc = "CT_INP4"]
    #[inline(always)]
    pub fn ct_inp4(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp4)
    }
    #[doc = "CT_INP5"]
    #[inline(always)]
    pub fn ct_inp5(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp5)
    }
    #[doc = "CT_INP6"]
    #[inline(always)]
    pub fn ct_inp6(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp6)
    }
    #[doc = "CT_INP7"]
    #[inline(always)]
    pub fn ct_inp7(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp7)
    }
    #[doc = "CT_INP8"]
    #[inline(always)]
    pub fn ct_inp8(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp8)
    }
    #[doc = "CT_INP9"]
    #[inline(always)]
    pub fn ct_inp9(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp9)
    }
    #[doc = "CT_INP10"]
    #[inline(always)]
    pub fn ct_inp10(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp10)
    }
    #[doc = "CT_INP11"]
    #[inline(always)]
    pub fn ct_inp11(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp11)
    }
    #[doc = "CT_INP12"]
    #[inline(always)]
    pub fn ct_inp12(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp12)
    }
    #[doc = "CT_INP13"]
    #[inline(always)]
    pub fn ct_inp13(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp13)
    }
    #[doc = "CT_INP14"]
    #[inline(always)]
    pub fn ct_inp14(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp14)
    }
    #[doc = "CT_INP15"]
    #[inline(always)]
    pub fn ct_inp15(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::CtInp15)
    }
    #[doc = "SHARED I2S0_WS"]
    #[inline(always)]
    pub fn shared_i2s0_ws(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::SharedI2s0Ws)
    }
    #[doc = "SHARED I2S1_WS"]
    #[inline(always)]
    pub fn shared_i2s1_ws(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::SharedI2s1Ws)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn usb1_frame_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(CapnSel::Usb1FrameToggle)
    }
}
impl R {
    #[doc = "Bits 0:4 - Counter Timer m, Capture Port Input n 19:1 Mux Select. . ."]
    #[inline(always)]
    pub fn capn_sel(&self) -> CapnSelR {
        CapnSelR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_SEL")
            .field("capn_sel", &self.capn_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Counter Timer m, Capture Port Input n 19:1 Mux Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn capn_sel(&mut self) -> CapnSelW<CapSelSpec> {
        CapnSelW::new(self, 0)
    }
}
#[doc = "CT32BIT N Counter Timer Capture Trigger Multiplexers M\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapSelSpec;
impl crate::RegisterSpec for CapSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_sel::R`](R) reader structure"]
impl crate::Readable for CapSelSpec {}
#[doc = "`write(|w| ..)` method takes [`cap_sel::W`](W) writer structure"]
impl crate::Writable for CapSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_SEL%s to value 0x1f"]
impl crate::Resettable for CapSelSpec {
    const RESET_VALUE: u32 = 0x1f;
}
