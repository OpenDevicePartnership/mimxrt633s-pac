#[doc = "Register `MSTTIME` reader"]
pub type R = crate::R<MsttimeSpec>;
#[doc = "Register `MSTTIME` writer"]
pub type W = crate::W<MsttimeSpec>;
#[doc = "Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW.\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mstscllow {
    #[doc = "0: 2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    Clocks2 = 0,
    #[doc = "1: 3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    Clocks3 = 1,
    #[doc = "2: 4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    Clocks4 = 2,
    #[doc = "3: 5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    Clocks5 = 3,
    #[doc = "4: 6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    Clocks6 = 4,
    #[doc = "5: 7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    Clocks7 = 5,
    #[doc = "6: 8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    Clocks8 = 6,
    #[doc = "7: 9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    Clocks9 = 7,
}
impl From<Mstscllow> for u8 {
    #[inline(always)]
    fn from(variant: Mstscllow) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mstscllow {
    type Ux = u8;
}
impl crate::IsEnum for Mstscllow {}
#[doc = "Field `MSTSCLLOW` reader - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
pub type MstscllowR = crate::FieldReader<Mstscllow>;
impl MstscllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstscllow {
        match self.bits {
            0 => Mstscllow::Clocks2,
            1 => Mstscllow::Clocks3,
            2 => Mstscllow::Clocks4,
            3 => Mstscllow::Clocks5,
            4 => Mstscllow::Clocks6,
            5 => Mstscllow::Clocks7,
            6 => Mstscllow::Clocks8,
            7 => Mstscllow::Clocks9,
            _ => unreachable!(),
        }
    }
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_2(&self) -> bool {
        *self == Mstscllow::Clocks2
    }
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_3(&self) -> bool {
        *self == Mstscllow::Clocks3
    }
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_4(&self) -> bool {
        *self == Mstscllow::Clocks4
    }
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_5(&self) -> bool {
        *self == Mstscllow::Clocks5
    }
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_6(&self) -> bool {
        *self == Mstscllow::Clocks6
    }
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_7(&self) -> bool {
        *self == Mstscllow::Clocks7
    }
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_8(&self) -> bool {
        *self == Mstscllow::Clocks8
    }
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_9(&self) -> bool {
        *self == Mstscllow::Clocks9
    }
}
#[doc = "Field `MSTSCLLOW` writer - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
pub type MstscllowW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mstscllow, crate::Safe>;
impl<'a, REG> MstscllowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 clocks. Minimum SCL low time is 2 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks2)
    }
    #[doc = "3 clocks. Minimum SCL low time is 3 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks3)
    }
    #[doc = "4 clocks. Minimum SCL low time is 4 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks4)
    }
    #[doc = "5 clocks. Minimum SCL low time is 5 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks5)
    }
    #[doc = "6 clocks. Minimum SCL low time is 6 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks6)
    }
    #[doc = "7 clocks. Minimum SCL low time is 7 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks7)
    }
    #[doc = "8 clocks. Minimum SCL low time is 8 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_8(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks8)
    }
    #[doc = "9 clocks. Minimum SCL low time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_9(self) -> &'a mut crate::W<REG> {
        self.variant(Mstscllow::Clocks9)
    }
}
#[doc = "Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH.\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mstsclhigh {
    #[doc = "0: 2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    Clocks2 = 0,
    #[doc = "1: 3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    Clocks3 = 1,
    #[doc = "2: 4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    Clocks4 = 2,
    #[doc = "3: 5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    Clocks5 = 3,
    #[doc = "4: 6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    Clocks6 = 4,
    #[doc = "5: 7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    Clocks7 = 5,
    #[doc = "6: 8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    Clocks8 = 6,
    #[doc = "7: 9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    Clocks9 = 7,
}
impl From<Mstsclhigh> for u8 {
    #[inline(always)]
    fn from(variant: Mstsclhigh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mstsclhigh {
    type Ux = u8;
}
impl crate::IsEnum for Mstsclhigh {}
#[doc = "Field `MSTSCLHIGH` reader - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
pub type MstsclhighR = crate::FieldReader<Mstsclhigh>;
impl MstsclhighR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstsclhigh {
        match self.bits {
            0 => Mstsclhigh::Clocks2,
            1 => Mstsclhigh::Clocks3,
            2 => Mstsclhigh::Clocks4,
            3 => Mstsclhigh::Clocks5,
            4 => Mstsclhigh::Clocks6,
            5 => Mstsclhigh::Clocks7,
            6 => Mstsclhigh::Clocks8,
            7 => Mstsclhigh::Clocks9,
            _ => unreachable!(),
        }
    }
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_2(&self) -> bool {
        *self == Mstsclhigh::Clocks2
    }
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    #[inline(always)]
    pub fn is_clocks_3(&self) -> bool {
        *self == Mstsclhigh::Clocks3
    }
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_4(&self) -> bool {
        *self == Mstsclhigh::Clocks4
    }
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_5(&self) -> bool {
        *self == Mstsclhigh::Clocks5
    }
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_6(&self) -> bool {
        *self == Mstsclhigh::Clocks6
    }
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_7(&self) -> bool {
        *self == Mstsclhigh::Clocks7
    }
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_8(&self) -> bool {
        *self == Mstsclhigh::Clocks8
    }
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn is_clocks_9(&self) -> bool {
        *self == Mstsclhigh::Clocks9
    }
}
#[doc = "Field `MSTSCLHIGH` writer - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
pub type MstsclhighW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mstsclhigh, crate::Safe>;
impl<'a, REG> MstsclhighW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 clocks. Minimum SCL high time is 2 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks2)
    }
    #[doc = "3 clocks. Minimum SCL high time is 3 clocks of the I2C clock pre-divider ."]
    #[inline(always)]
    pub fn clocks_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks3)
    }
    #[doc = "4 clocks. Minimum SCL high time is 4 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_4(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks4)
    }
    #[doc = "5 clocks. Minimum SCL high time is 5 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_5(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks5)
    }
    #[doc = "6 clocks. Minimum SCL high time is 6 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_6(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks6)
    }
    #[doc = "7 clocks. Minimum SCL high time is 7 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_7(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks7)
    }
    #[doc = "8 clocks. Minimum SCL high time is 8 clock of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_8(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks8)
    }
    #[doc = "9 clocks. Minimum SCL high time is 9 clocks of the I2C clock pre-divider."]
    #[inline(always)]
    pub fn clocks_9(self) -> &'a mut crate::W<REG> {
        self.variant(Mstsclhigh::Clocks9)
    }
}
impl R {
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline(always)]
    pub fn mstscllow(&self) -> MstscllowR {
        MstscllowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline(always)]
    pub fn mstsclhigh(&self) -> MstsclhighR {
        MstsclhighR::new(((self.bits >> 4) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTTIME")
            .field("mstscllow", &self.mstscllow())
            .field("mstsclhigh", &self.mstsclhigh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline(always)]
    pub fn mstscllow(&mut self) -> MstscllowW<MsttimeSpec> {
        MstscllowW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline(always)]
    pub fn mstsclhigh(&mut self) -> MstsclhighW<MsttimeSpec> {
        MstsclhighW::new(self, 4)
    }
}
#[doc = "Master timing configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`msttime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msttime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsttimeSpec;
impl crate::RegisterSpec for MsttimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msttime::R`](R) reader structure"]
impl crate::Readable for MsttimeSpec {}
#[doc = "`write(|w| ..)` method takes [`msttime::W`](W) writer structure"]
impl crate::Writable for MsttimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTTIME to value 0x77"]
impl crate::Resettable for MsttimeSpec {
    const RESET_VALUE: u32 = 0x77;
}
