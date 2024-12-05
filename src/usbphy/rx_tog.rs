#[doc = "Register `RX_TOG` reader"]
pub type R = crate::R<RxTogSpec>;
#[doc = "Register `RX_TOG` writer"]
pub type W = crate::W<RxTogSpec>;
#[doc = "The ENVADJ field adjusts the trip point for the envelope detector\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Envadj {
    #[doc = "0: Trip-Level Voltage is 0.1000 V"]
    Envadj0 = 0,
    #[doc = "1: Trip-Level Voltage is 0.1125 V"]
    Envadj1 = 1,
    #[doc = "2: Trip-Level Voltage is 0.1250 V"]
    Envadj2 = 2,
    #[doc = "3: Trip-Level Voltage is 0.0875 V"]
    Envadj3 = 3,
}
impl From<Envadj> for u8 {
    #[inline(always)]
    fn from(variant: Envadj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Envadj {
    type Ux = u8;
}
impl crate::IsEnum for Envadj {}
#[doc = "Field `ENVADJ` reader - The ENVADJ field adjusts the trip point for the envelope detector"]
pub type EnvadjR = crate::FieldReader<Envadj>;
impl EnvadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Envadj> {
        match self.bits {
            0 => Some(Envadj::Envadj0),
            1 => Some(Envadj::Envadj1),
            2 => Some(Envadj::Envadj2),
            3 => Some(Envadj::Envadj3),
            _ => None,
        }
    }
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn is_envadj_0(&self) -> bool {
        *self == Envadj::Envadj0
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn is_envadj_1(&self) -> bool {
        *self == Envadj::Envadj1
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn is_envadj_2(&self) -> bool {
        *self == Envadj::Envadj2
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn is_envadj_3(&self) -> bool {
        *self == Envadj::Envadj3
    }
}
#[doc = "Field `ENVADJ` writer - The ENVADJ field adjusts the trip point for the envelope detector"]
pub type EnvadjW<'a, REG> = crate::FieldWriter<'a, REG, 3, Envadj>;
impl<'a, REG> EnvadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn envadj_0(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Envadj0)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn envadj_1(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Envadj1)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn envadj_2(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Envadj2)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn envadj_3(self) -> &'a mut crate::W<REG> {
        self.variant(Envadj::Envadj3)
    }
}
#[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Disconadj {
    #[doc = "0: Trip-Level Voltage is 0.56875 V"]
    Disconadj0 = 0,
    #[doc = "1: Trip-Level Voltage is 0.55000 V"]
    Disconadj1 = 1,
    #[doc = "2: Trip-Level Voltage is 0.58125 V"]
    Disconadj2 = 2,
    #[doc = "3: Trip-Level Voltage is 0.60000 V"]
    Disconadj3 = 3,
}
impl From<Disconadj> for u8 {
    #[inline(always)]
    fn from(variant: Disconadj) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Disconadj {
    type Ux = u8;
}
impl crate::IsEnum for Disconadj {}
#[doc = "Field `DISCONADJ` reader - The DISCONADJ field adjusts the trip point for the disconnect detector."]
pub type DisconadjR = crate::FieldReader<Disconadj>;
impl DisconadjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Disconadj> {
        match self.bits {
            0 => Some(Disconadj::Disconadj0),
            1 => Some(Disconadj::Disconadj1),
            2 => Some(Disconadj::Disconadj2),
            3 => Some(Disconadj::Disconadj3),
            _ => None,
        }
    }
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn is_disconadj_0(&self) -> bool {
        *self == Disconadj::Disconadj0
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn is_disconadj_1(&self) -> bool {
        *self == Disconadj::Disconadj1
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn is_disconadj_2(&self) -> bool {
        *self == Disconadj::Disconadj2
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn is_disconadj_3(&self) -> bool {
        *self == Disconadj::Disconadj3
    }
}
#[doc = "Field `DISCONADJ` writer - The DISCONADJ field adjusts the trip point for the disconnect detector."]
pub type DisconadjW<'a, REG> = crate::FieldWriter<'a, REG, 3, Disconadj>;
impl<'a, REG> DisconadjW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn disconadj_0(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Disconadj0)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn disconadj_1(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Disconadj1)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn disconadj_2(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Disconadj2)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn disconadj_3(self) -> &'a mut crate::W<REG> {
        self.variant(Disconadj::Disconadj3)
    }
}
#[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdbypass {
    #[doc = "0: Normal operation."]
    Rxdbypass0 = 0,
    #[doc = "1: Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    Rxdbypass1 = 1,
}
impl From<Rxdbypass> for bool {
    #[inline(always)]
    fn from(variant: Rxdbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDBYPASS` reader - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
pub type RxdbypassR = crate::BitReader<Rxdbypass>;
impl RxdbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdbypass {
        match self.bits {
            false => Rxdbypass::Rxdbypass0,
            true => Rxdbypass::Rxdbypass1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_rxdbypass_0(&self) -> bool {
        *self == Rxdbypass::Rxdbypass0
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn is_rxdbypass_1(&self) -> bool {
        *self == Rxdbypass::Rxdbypass1
    }
}
#[doc = "Field `RXDBYPASS` writer - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
pub type RxdbypassW<'a, REG> = crate::BitWriter<'a, REG, Rxdbypass>;
impl<'a, REG> RxdbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn rxdbypass_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdbypass::Rxdbypass0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn rxdbypass_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdbypass::Rxdbypass1)
    }
}
impl R {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&self) -> EnvadjR {
        EnvadjR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&self) -> DisconadjR {
        DisconadjR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RxdbypassR {
        RxdbypassR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_TOG")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .field("rxdbypass", &self.rxdbypass())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&mut self) -> EnvadjW<RxTogSpec> {
        EnvadjW::new(self, 0)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&mut self) -> DisconadjW<RxTogSpec> {
        DisconadjW::new(self, 4)
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&mut self) -> RxdbypassW<RxTogSpec> {
        RxdbypassW::new(self, 22)
    }
}
#[doc = "USB PHY Receiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_tog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_tog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxTogSpec;
impl crate::RegisterSpec for RxTogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_tog::R`](R) reader structure"]
impl crate::Readable for RxTogSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_tog::W`](W) writer structure"]
impl crate::Writable for RxTogSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_TOG to value 0"]
impl crate::Resettable for RxTogSpec {
    const RESET_VALUE: u32 = 0;
}
