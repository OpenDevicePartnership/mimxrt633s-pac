#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Fn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fn {
    #[doc = "0: Fn bit in the CR register is written 0 (default)."]
    Fn0 = 0,
    #[doc = "1: Fn bit in the CR register is written 1."]
    Fn1 = 1,
}
impl From<Fn> for u8 {
    #[inline(always)]
    fn from(variant: Fn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fn {
    type Ux = u8;
}
impl crate::IsEnum for Fn {}
#[doc = "Field `Fn` reader - Fn"]
pub type FnR = crate::FieldReader<Fn>;
impl FnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fn> {
        match self.bits {
            0 => Some(Fn::Fn0),
            1 => Some(Fn::Fn1),
            _ => None,
        }
    }
    #[doc = "Fn bit in the CR register is written 0 (default)."]
    #[inline(always)]
    pub fn is_fn_0(&self) -> bool {
        *self == Fn::Fn0
    }
    #[doc = "Fn bit in the CR register is written 1."]
    #[inline(always)]
    pub fn is_fn_1(&self) -> bool {
        *self == Fn::Fn1
    }
}
#[doc = "EP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep {
    #[doc = "0: The MUA side event is not pending (default)."]
    Ep0 = 0,
    #[doc = "1: The MUA side event is pending."]
    Ep1 = 1,
}
impl From<Ep> for bool {
    #[inline(always)]
    fn from(variant: Ep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP` reader - EP"]
pub type EpR = crate::BitReader<Ep>;
impl EpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep {
        match self.bits {
            false => Ep::Ep0,
            true => Ep::Ep1,
        }
    }
    #[doc = "The MUA side event is not pending (default)."]
    #[inline(always)]
    pub fn is_ep_0(&self) -> bool {
        *self == Ep::Ep0
    }
    #[doc = "The MUA side event is pending."]
    #[inline(always)]
    pub fn is_ep_1(&self) -> bool {
        *self == Ep::Ep1
    }
}
#[doc = "PM\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pm {
    #[doc = "0: The MUB processor is in Run Mode."]
    Run = 0,
    #[doc = "1: The MUB processor is in WAIT Mode."]
    Wait = 1,
}
impl From<Pm> for u8 {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pm {
    type Ux = u8;
}
impl crate::IsEnum for Pm {}
#[doc = "Field `PM` reader - PM"]
pub type PmR = crate::FieldReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pm> {
        match self.bits {
            0 => Some(Pm::Run),
            1 => Some(Pm::Wait),
            _ => None,
        }
    }
    #[doc = "The MUB processor is in Run Mode."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Pm::Run
    }
    #[doc = "The MUB processor is in WAIT Mode."]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Pm::Wait
    }
}
#[doc = "RS\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs {
    #[doc = "0: The MUB side of the MU is not in reset."]
    Rs0 = 0,
    #[doc = "1: The MUB side of the MU is in reset."]
    Rs1 = 1,
}
impl From<Rs> for bool {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - RS"]
pub type RsR = crate::BitReader<Rs>;
impl RsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            false => Rs::Rs0,
            true => Rs::Rs1,
        }
    }
    #[doc = "The MUB side of the MU is not in reset."]
    #[inline(always)]
    pub fn is_rs_0(&self) -> bool {
        *self == Rs::Rs0
    }
    #[doc = "The MUB side of the MU is in reset."]
    #[inline(always)]
    pub fn is_rs_1(&self) -> bool {
        *self == Rs::Rs1
    }
}
#[doc = "FUP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fup {
    #[doc = "0: No flags updated, initiated by the MUA, in progress (default)"]
    Fup0 = 0,
    #[doc = "1: MUA initiated flags update, processing"]
    Fup1 = 1,
}
impl From<Fup> for bool {
    #[inline(always)]
    fn from(variant: Fup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUP` reader - FUP"]
pub type FupR = crate::BitReader<Fup>;
impl FupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fup {
        match self.bits {
            false => Fup::Fup0,
            true => Fup::Fup1,
        }
    }
    #[doc = "No flags updated, initiated by the MUA, in progress (default)"]
    #[inline(always)]
    pub fn is_fup_0(&self) -> bool {
        *self == Fup::Fup0
    }
    #[doc = "MUA initiated flags update, processing"]
    #[inline(always)]
    pub fn is_fup_1(&self) -> bool {
        *self == Fup::Fup1
    }
}
#[doc = "BRDIP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdip {
    #[doc = "0: Processor B-side did not exit reset"]
    Rdip0 = 0,
    #[doc = "1: Processor B-side exited from reset"]
    Rdip1 = 1,
}
impl From<Rdip> for bool {
    #[inline(always)]
    fn from(variant: Rdip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIP` reader - BRDIP"]
pub type RdipR = crate::BitReader<Rdip>;
impl RdipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdip {
        match self.bits {
            false => Rdip::Rdip0,
            true => Rdip::Rdip1,
        }
    }
    #[doc = "Processor B-side did not exit reset"]
    #[inline(always)]
    pub fn is_rdip_0(&self) -> bool {
        *self == Rdip::Rdip0
    }
    #[doc = "Processor B-side exited from reset"]
    #[inline(always)]
    pub fn is_rdip_1(&self) -> bool {
        *self == Rdip::Rdip1
    }
}
#[doc = "Field `RDIP` writer - BRDIP"]
pub type RdipW<'a, REG> = crate::BitWriter<'a, REG, Rdip>;
impl<'a, REG> RdipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor B-side did not exit reset"]
    #[inline(always)]
    pub fn rdip_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdip::Rdip0)
    }
    #[doc = "Processor B-side exited from reset"]
    #[inline(always)]
    pub fn rdip_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdip::Rdip1)
    }
}
#[doc = "RAIP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raip {
    #[doc = "0: Processor B-side did not enter reset"]
    Raip0 = 0,
    #[doc = "1: Processor B-side entered reset"]
    Raip1 = 1,
}
impl From<Raip> for bool {
    #[inline(always)]
    fn from(variant: Raip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAIP` reader - RAIP"]
pub type RaipR = crate::BitReader<Raip>;
impl RaipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Raip {
        match self.bits {
            false => Raip::Raip0,
            true => Raip::Raip1,
        }
    }
    #[doc = "Processor B-side did not enter reset"]
    #[inline(always)]
    pub fn is_raip_0(&self) -> bool {
        *self == Raip::Raip0
    }
    #[doc = "Processor B-side entered reset"]
    #[inline(always)]
    pub fn is_raip_1(&self) -> bool {
        *self == Raip::Raip1
    }
}
#[doc = "Field `RAIP` writer - RAIP"]
pub type RaipW<'a, REG> = crate::BitWriter<'a, REG, Raip>;
impl<'a, REG> RaipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Processor B-side did not enter reset"]
    #[inline(always)]
    pub fn raip_0(self) -> &'a mut crate::W<REG> {
        self.variant(Raip::Raip0)
    }
    #[doc = "Processor B-side entered reset"]
    #[inline(always)]
    pub fn raip_1(self) -> &'a mut crate::W<REG> {
        self.variant(Raip::Raip1)
    }
}
#[doc = "TEn\n\nValue on reset: 15"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ten {
    #[doc = "0: MUA TRn register is not empty."]
    Ten0 = 0,
    #[doc = "1: MUA TRn register is empty (default)."]
    Ten1 = 1,
}
impl From<Ten> for u8 {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ten {
    type Ux = u8;
}
impl crate::IsEnum for Ten {}
#[doc = "Field `TEn` reader - TEn"]
pub type TenR = crate::FieldReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ten> {
        match self.bits {
            0 => Some(Ten::Ten0),
            1 => Some(Ten::Ten1),
            _ => None,
        }
    }
    #[doc = "MUA TRn register is not empty."]
    #[inline(always)]
    pub fn is_ten_0(&self) -> bool {
        *self == Ten::Ten0
    }
    #[doc = "MUA TRn register is empty (default)."]
    #[inline(always)]
    pub fn is_ten_1(&self) -> bool {
        *self == Ten::Ten1
    }
}
#[doc = "RFn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfn {
    #[doc = "0: MUA RRn register is not full (default)."]
    Rfn0 = 0,
    #[doc = "1: MUA RRn register has received data from MUB TRn register and is ready to be read by the MUA."]
    Rfn1 = 1,
}
impl From<Rfn> for u8 {
    #[inline(always)]
    fn from(variant: Rfn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfn {
    type Ux = u8;
}
impl crate::IsEnum for Rfn {}
#[doc = "Field `RFn` reader - RFn"]
pub type RfnR = crate::FieldReader<Rfn>;
impl RfnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rfn> {
        match self.bits {
            0 => Some(Rfn::Rfn0),
            1 => Some(Rfn::Rfn1),
            _ => None,
        }
    }
    #[doc = "MUA RRn register is not full (default)."]
    #[inline(always)]
    pub fn is_rfn_0(&self) -> bool {
        *self == Rfn::Rfn0
    }
    #[doc = "MUA RRn register has received data from MUB TRn register and is ready to be read by the MUA."]
    #[inline(always)]
    pub fn is_rfn_1(&self) -> bool {
        *self == Rfn::Rfn1
    }
}
#[doc = "GIPn\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gipn {
    #[doc = "0: MUA general purpose interrupt n is not pending. (default)"]
    Gipn0 = 0,
    #[doc = "1: MUA general purpose interrupt n is pending."]
    Gipn1 = 1,
}
impl From<Gipn> for u8 {
    #[inline(always)]
    fn from(variant: Gipn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gipn {
    type Ux = u8;
}
impl crate::IsEnum for Gipn {}
#[doc = "Field `GIPn` reader - GIPn"]
pub type GipnR = crate::FieldReader<Gipn>;
impl GipnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gipn> {
        match self.bits {
            0 => Some(Gipn::Gipn0),
            1 => Some(Gipn::Gipn1),
            _ => None,
        }
    }
    #[doc = "MUA general purpose interrupt n is not pending. (default)"]
    #[inline(always)]
    pub fn is_gipn_0(&self) -> bool {
        *self == Gipn::Gipn0
    }
    #[doc = "MUA general purpose interrupt n is pending."]
    #[inline(always)]
    pub fn is_gipn_1(&self) -> bool {
        *self == Gipn::Gipn1
    }
}
#[doc = "Field `GIPn` writer - GIPn"]
pub type GipnW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gipn>;
impl<'a, REG> GipnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MUA general purpose interrupt n is not pending. (default)"]
    #[inline(always)]
    pub fn gipn_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gipn::Gipn0)
    }
    #[doc = "MUA general purpose interrupt n is pending."]
    #[inline(always)]
    pub fn gipn_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gipn::Gipn1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Fn"]
    #[inline(always)]
    pub fn fn_(&self) -> FnR {
        FnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - RS"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FUP"]
    #[inline(always)]
    pub fn fup(&self) -> FupR {
        FupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRDIP"]
    #[inline(always)]
    pub fn rdip(&self) -> RdipR {
        RdipR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RAIP"]
    #[inline(always)]
    pub fn raip(&self) -> RaipR {
        RaipR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 20:23 - TEn"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RFn"]
    #[inline(always)]
    pub fn rfn(&self) -> RfnR {
        RfnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GIPn"]
    #[inline(always)]
    pub fn gipn(&self) -> GipnR {
        GipnR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("fn_", &self.fn_())
            .field("ep", &self.ep())
            .field("pm", &self.pm())
            .field("rs", &self.rs())
            .field("fup", &self.fup())
            .field("rdip", &self.rdip())
            .field("raip", &self.raip())
            .field("ten", &self.ten())
            .field("rfn", &self.rfn())
            .field("gipn", &self.gipn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - BRDIP"]
    #[inline(always)]
    #[must_use]
    pub fn rdip(&mut self) -> RdipW<SrSpec> {
        RdipW::new(self, 9)
    }
    #[doc = "Bit 10 - RAIP"]
    #[inline(always)]
    #[must_use]
    pub fn raip(&mut self) -> RaipW<SrSpec> {
        RaipW::new(self, 10)
    }
    #[doc = "Bits 28:31 - GIPn"]
    #[inline(always)]
    #[must_use]
    pub fn gipn(&mut self) -> GipnW<SrSpec> {
        GipnW::new(self, 28)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x00f0_0080"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x00f0_0080;
}
