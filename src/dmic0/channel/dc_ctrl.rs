#[doc = "Register `DC_CTRL` reader"]
pub type R = crate::R<DcCtrlSpec>;
#[doc = "Register `DC_CTRL` writer"]
pub type W = crate::W<DcCtrlSpec>;
#[doc = "DC block filter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcpole {
    #[doc = "0: Flat response, no filter."]
    FlatResponse = 0,
    #[doc = "1: 155 Hz."]
    Hz155 = 1,
    #[doc = "2: 78 Hz."]
    Hz78 = 2,
    #[doc = "3: 39 Hz"]
    Hz39 = 3,
}
impl From<Dcpole> for u8 {
    #[inline(always)]
    fn from(variant: Dcpole) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcpole {
    type Ux = u8;
}
impl crate::IsEnum for Dcpole {}
#[doc = "Field `DCPOLE` reader - DC block filter"]
pub type DcpoleR = crate::FieldReader<Dcpole>;
impl DcpoleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcpole {
        match self.bits {
            0 => Dcpole::FlatResponse,
            1 => Dcpole::Hz155,
            2 => Dcpole::Hz78,
            3 => Dcpole::Hz39,
            _ => unreachable!(),
        }
    }
    #[doc = "Flat response, no filter."]
    #[inline(always)]
    pub fn is_flat_response(&self) -> bool {
        *self == Dcpole::FlatResponse
    }
    #[doc = "155 Hz."]
    #[inline(always)]
    pub fn is_hz_155(&self) -> bool {
        *self == Dcpole::Hz155
    }
    #[doc = "78 Hz."]
    #[inline(always)]
    pub fn is_hz_78(&self) -> bool {
        *self == Dcpole::Hz78
    }
    #[doc = "39 Hz"]
    #[inline(always)]
    pub fn is_hz_39(&self) -> bool {
        *self == Dcpole::Hz39
    }
}
#[doc = "Field `DCPOLE` writer - DC block filter"]
pub type DcpoleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dcpole, crate::Safe>;
impl<'a, REG> DcpoleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flat response, no filter."]
    #[inline(always)]
    pub fn flat_response(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpole::FlatResponse)
    }
    #[doc = "155 Hz."]
    #[inline(always)]
    pub fn hz_155(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpole::Hz155)
    }
    #[doc = "78 Hz."]
    #[inline(always)]
    pub fn hz_78(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpole::Hz78)
    }
    #[doc = "39 Hz"]
    #[inline(always)]
    pub fn hz_39(self) -> &'a mut crate::W<REG> {
        self.variant(Dcpole::Hz39)
    }
}
#[doc = "Field `DCGAIN` reader - Fine gain adjustment in the form of a number of bits to downshift."]
pub type DcgainR = crate::FieldReader;
#[doc = "Field `DCGAIN` writer - Fine gain adjustment in the form of a number of bits to downshift."]
pub type DcgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Selects 16-bit saturation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saturateat16bit {
    #[doc = "0: Results roll over if out range and do not saturate."]
    DoNotSaturate = 0,
    #[doc = "1: If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    Saturate = 1,
}
impl From<Saturateat16bit> for bool {
    #[inline(always)]
    fn from(variant: Saturateat16bit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SATURATEAT16BIT` reader - Selects 16-bit saturation."]
pub type Saturateat16bitR = crate::BitReader<Saturateat16bit>;
impl Saturateat16bitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Saturateat16bit {
        match self.bits {
            false => Saturateat16bit::DoNotSaturate,
            true => Saturateat16bit::Saturate,
        }
    }
    #[doc = "Results roll over if out range and do not saturate."]
    #[inline(always)]
    pub fn is_do_not_saturate(&self) -> bool {
        *self == Saturateat16bit::DoNotSaturate
    }
    #[doc = "If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    #[inline(always)]
    pub fn is_saturate(&self) -> bool {
        *self == Saturateat16bit::Saturate
    }
}
#[doc = "Field `SATURATEAT16BIT` writer - Selects 16-bit saturation."]
pub type Saturateat16bitW<'a, REG> = crate::BitWriter<'a, REG, Saturateat16bit>;
impl<'a, REG> Saturateat16bitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Results roll over if out range and do not saturate."]
    #[inline(always)]
    pub fn do_not_saturate(self) -> &'a mut crate::W<REG> {
        self.variant(Saturateat16bit::DoNotSaturate)
    }
    #[doc = "If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    #[inline(always)]
    pub fn saturate(self) -> &'a mut crate::W<REG> {
        self.variant(Saturateat16bit::Saturate)
    }
}
#[doc = "Sign extend.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signextend {
    #[doc = "0: The top byte of the FIFODATA register is always 0."]
    DoNotSignextend = 0,
    #[doc = "1: The top byte of the FIFODATA register is sign extended. This allows processing of 24-bit audio data on 32-bit machines."]
    Signextend = 1,
}
impl From<Signextend> for bool {
    #[inline(always)]
    fn from(variant: Signextend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIGNEXTEND` reader - Sign extend."]
pub type SignextendR = crate::BitReader<Signextend>;
impl SignextendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Signextend {
        match self.bits {
            false => Signextend::DoNotSignextend,
            true => Signextend::Signextend,
        }
    }
    #[doc = "The top byte of the FIFODATA register is always 0."]
    #[inline(always)]
    pub fn is_do_not_signextend(&self) -> bool {
        *self == Signextend::DoNotSignextend
    }
    #[doc = "The top byte of the FIFODATA register is sign extended. This allows processing of 24-bit audio data on 32-bit machines."]
    #[inline(always)]
    pub fn is_signextend(&self) -> bool {
        *self == Signextend::Signextend
    }
}
#[doc = "Field `SIGNEXTEND` writer - Sign extend."]
pub type SignextendW<'a, REG> = crate::BitWriter<'a, REG, Signextend>;
impl<'a, REG> SignextendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The top byte of the FIFODATA register is always 0."]
    #[inline(always)]
    pub fn do_not_signextend(self) -> &'a mut crate::W<REG> {
        self.variant(Signextend::DoNotSignextend)
    }
    #[doc = "The top byte of the FIFODATA register is sign extended. This allows processing of 24-bit audio data on 32-bit machines."]
    #[inline(always)]
    pub fn signextend(self) -> &'a mut crate::W<REG> {
        self.variant(Signextend::Signextend)
    }
}
impl R {
    #[doc = "Bits 0:1 - DC block filter"]
    #[inline(always)]
    pub fn dcpole(&self) -> DcpoleR {
        DcpoleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    pub fn dcgain(&self) -> DcgainR {
        DcgainR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selects 16-bit saturation."]
    #[inline(always)]
    pub fn saturateat16bit(&self) -> Saturateat16bitR {
        Saturateat16bitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sign extend."]
    #[inline(always)]
    pub fn signextend(&self) -> SignextendR {
        SignextendR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DC_CTRL")
            .field("dcpole", &self.dcpole())
            .field("dcgain", &self.dcgain())
            .field("saturateat16bit", &self.saturateat16bit())
            .field("signextend", &self.signextend())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - DC block filter"]
    #[inline(always)]
    #[must_use]
    pub fn dcpole(&mut self) -> DcpoleW<DcCtrlSpec> {
        DcpoleW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    #[must_use]
    pub fn dcgain(&mut self) -> DcgainW<DcCtrlSpec> {
        DcgainW::new(self, 4)
    }
    #[doc = "Bit 8 - Selects 16-bit saturation."]
    #[inline(always)]
    #[must_use]
    pub fn saturateat16bit(&mut self) -> Saturateat16bitW<DcCtrlSpec> {
        Saturateat16bitW::new(self, 8)
    }
    #[doc = "Bit 9 - Sign extend."]
    #[inline(always)]
    #[must_use]
    pub fn signextend(&mut self) -> SignextendW<DcCtrlSpec> {
        SignextendW::new(self, 9)
    }
}
#[doc = "DC Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcCtrlSpec;
impl crate::RegisterSpec for DcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_ctrl::R`](R) reader structure"]
impl crate::Readable for DcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_ctrl::W`](W) writer structure"]
impl crate::Writable for DcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_CTRL to value 0"]
impl crate::Resettable for DcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
