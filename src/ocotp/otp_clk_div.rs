#[doc = "Register `OTP_CLK_DIV` reader"]
pub type R = crate::R<OtpClkDivSpec>;
#[doc = "Register `OTP_CLK_DIV` writer"]
pub type W = crate::W<OtpClkDivSpec>;
#[doc = "Clock divider value by -1 encoding. It's used to generate the clock to OTP memory (otp_clk) with apb_clk. The maximum otp_clk frequency is 120Mhz. 0: Divide by 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Div {
    #[doc = "0: Divide by 1"]
    Div1 = 0,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "2: Divide by 3"]
    Div3 = 2,
    #[doc = "3: Divide by 4"]
    Div4 = 3,
    #[doc = "4: Divide by 5"]
    Div5 = 4,
    #[doc = "5: Divide by 6"]
    Div6 = 5,
    #[doc = "6: Divide by 7"]
    Div7 = 6,
    #[doc = "7: Divide by 8"]
    Div8 = 7,
    #[doc = "8: Divide by 9"]
    Div9 = 8,
    #[doc = "9: Divide by 10"]
    Div10 = 9,
    #[doc = "10: Divide by 11"]
    Div11 = 10,
    #[doc = "11: Divide by 12"]
    Div12 = 11,
    #[doc = "12: Divide by 13"]
    Div13 = 12,
    #[doc = "13: Divide by 14"]
    Div14 = 13,
    #[doc = "14: Divide by 15"]
    Div15 = 14,
    #[doc = "15: Divide by 16"]
    Div16 = 15,
}
impl From<Div> for u8 {
    #[inline(always)]
    fn from(variant: Div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Div {
    type Ux = u8;
}
impl crate::IsEnum for Div {}
#[doc = "Field `DIV` reader - Clock divider value by -1 encoding. It's used to generate the clock to OTP memory (otp_clk) with apb_clk. The maximum otp_clk frequency is 120Mhz. 0: Divide by 1"]
pub type DivR = crate::FieldReader<Div>;
impl DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Div {
        match self.bits {
            0 => Div::Div1,
            1 => Div::Div2,
            2 => Div::Div3,
            3 => Div::Div4,
            4 => Div::Div5,
            5 => Div::Div6,
            6 => Div::Div7,
            7 => Div::Div8,
            8 => Div::Div9,
            9 => Div::Div10,
            10 => Div::Div11,
            11 => Div::Div12,
            12 => Div::Div13,
            13 => Div::Div14,
            14 => Div::Div15,
            15 => Div::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == Div::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == Div::Div2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_div_3(&self) -> bool {
        *self == Div::Div3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == Div::Div4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_div_5(&self) -> bool {
        *self == Div::Div5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_div_6(&self) -> bool {
        *self == Div::Div6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_div_7(&self) -> bool {
        *self == Div::Div7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == Div::Div8
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn is_div_9(&self) -> bool {
        *self == Div::Div9
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn is_div_10(&self) -> bool {
        *self == Div::Div10
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn is_div_11(&self) -> bool {
        *self == Div::Div11
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn is_div_12(&self) -> bool {
        *self == Div::Div12
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn is_div_13(&self) -> bool {
        *self == Div::Div13
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn is_div_14(&self) -> bool {
        *self == Div::Div14
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn is_div_15(&self) -> bool {
        *self == Div::Div15
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == Div::Div16
    }
}
#[doc = "Field `DIV` writer - Clock divider value by -1 encoding. It's used to generate the clock to OTP memory (otp_clk) with apb_clk. The maximum otp_clk frequency is 120Mhz. 0: Divide by 1"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Div, crate::Safe>;
impl<'a, REG> DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn div_3(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn div_5(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div_6(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn div_7(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn div_9(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn div_10(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn div_11(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn div_12(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn div_13(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn div_14(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn div_15(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut crate::W<REG> {
        self.variant(Div::Div16)
    }
}
#[doc = "Field `RESET` reader - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT` reader - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
pub type HaltR = crate::BitReader;
#[doc = "Field `HALT` writer - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQFLAG` reader - Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
pub type ReqflagR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Clock divider value by -1 encoding. It's used to generate the clock to OTP memory (otp_clk) with apb_clk. The maximum otp_clk frequency is 120Mhz. 0: Divide by 1"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn reqflag(&self) -> ReqflagR {
        ReqflagR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_CLK_DIV")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock divider value by -1 encoding. It's used to generate the clock to OTP memory (otp_clk) with apb_clk. The maximum otp_clk frequency is 120Mhz. 0: Divide by 1"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<OtpClkDivSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 29 - Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<OtpClkDivSpec> {
        ResetW::new(self, 29)
    }
    #[doc = "Bit 30 - Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<OtpClkDivSpec> {
        HaltW::new(self, 30)
    }
}
#[doc = "OTP clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpClkDivSpec;
impl crate::RegisterSpec for OtpClkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_clk_div::R`](R) reader structure"]
impl crate::Readable for OtpClkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_clk_div::W`](W) writer structure"]
impl crate::Writable for OtpClkDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTP_CLK_DIV to value 0"]
impl crate::Resettable for OtpClkDivSpec {
    const RESET_VALUE: u32 = 0;
}
