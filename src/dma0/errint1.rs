#[doc = "Register `ERRINT1` reader"]
pub type R = crate::R<Errint1Spec>;
#[doc = "Register `ERRINT1` writer"]
pub type W = crate::W<Errint1Spec>;
#[doc = "Error Interrupt flag for DMA channel 32.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Err32 {
    #[doc = "0: The Error Interrupt is not active for DMA channel 32."]
    NotActive = 0,
    #[doc = "1: The Error Interrupt is pending for DMA channel 32."]
    Pending = 1,
}
impl From<Err32> for bool {
    #[inline(always)]
    fn from(variant: Err32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR32` reader - Error Interrupt flag for DMA channel 32."]
pub type Err32R = crate::BitReader<Err32>;
impl Err32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Err32 {
        match self.bits {
            false => Err32::NotActive,
            true => Err32::Pending,
        }
    }
    #[doc = "The Error Interrupt is not active for DMA channel 32."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Err32::NotActive
    }
    #[doc = "The Error Interrupt is pending for DMA channel 32."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Err32::Pending
    }
}
#[doc = "Field `ERR32` writer - Error Interrupt flag for DMA channel 32."]
pub type Err32W<'a, REG> = crate::BitWriter<'a, REG, Err32>;
impl<'a, REG> Err32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Error Interrupt is not active for DMA channel 32."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Err32::NotActive)
    }
    #[doc = "The Error Interrupt is pending for DMA channel 32."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(Err32::Pending)
    }
}
#[doc = "Additional error Interrupt flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Err63_33 {
    #[doc = "0: The Error Interrupt is not active for the relevant DMA channel."]
    NotActive = 0,
    #[doc = "1: The Error Interrupt is pending for the relevant DMA channel."]
    Pending = 1,
}
impl From<Err63_33> for u32 {
    #[inline(always)]
    fn from(variant: Err63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Err63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Err63_33 {}
#[doc = "Field `ERR63_33` reader - Additional error Interrupt flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Err63_33R = crate::FieldReader<Err63_33>;
impl Err63_33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Err63_33> {
        match self.bits {
            0 => Some(Err63_33::NotActive),
            1 => Some(Err63_33::Pending),
            _ => None,
        }
    }
    #[doc = "The Error Interrupt is not active for the relevant DMA channel."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Err63_33::NotActive
    }
    #[doc = "The Error Interrupt is pending for the relevant DMA channel."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Err63_33::Pending
    }
}
#[doc = "Field `ERR63_33` writer - Additional error Interrupt flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Err63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Err63_33>;
impl<'a, REG> Err63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The Error Interrupt is not active for the relevant DMA channel."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Err63_33::NotActive)
    }
    #[doc = "The Error Interrupt is pending for the relevant DMA channel."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(Err63_33::Pending)
    }
}
impl R {
    #[doc = "Bit 0 - Error Interrupt flag for DMA channel 32."]
    #[inline(always)]
    pub fn err32(&self) -> Err32R {
        Err32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Additional error Interrupt flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn err63_33(&self) -> Err63_33R {
        Err63_33R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERRINT1")
            .field("err32", &self.err32())
            .field("err63_33", &self.err63_33())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Error Interrupt flag for DMA channel 32."]
    #[inline(always)]
    pub fn err32(&mut self) -> Err32W<Errint1Spec> {
        Err32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional error Interrupt flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn err63_33(&mut self) -> Err63_33W<Errint1Spec> {
        Err63_33W::new(self, 1)
    }
}
#[doc = "Error Interrupt status for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`errint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Errint1Spec;
impl crate::RegisterSpec for Errint1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errint1::R`](R) reader structure"]
impl crate::Readable for Errint1Spec {}
#[doc = "`write(|w| ..)` method takes [`errint1::W`](W) writer structure"]
impl crate::Writable for Errint1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRINT1 to value 0"]
impl crate::Resettable for Errint1Spec {
    const RESET_VALUE: u32 = 0;
}
