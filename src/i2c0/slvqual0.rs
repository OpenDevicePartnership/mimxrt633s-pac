#[doc = "Register `SLVQUAL0` reader"]
pub type R = crate::R<Slvqual0Spec>;
#[doc = "Register `SLVQUAL0` writer"]
pub type W = crate::W<Slvqual0Spec>;
#[doc = "Qualify mode for slave address 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qualmode0 {
    #[doc = "0: Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    Mask = 0,
    #[doc = "1: Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    Extend = 1,
}
impl From<Qualmode0> for bool {
    #[inline(always)]
    fn from(variant: Qualmode0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QUALMODE0` reader - Qualify mode for slave address 0."]
pub type Qualmode0R = crate::BitReader<Qualmode0>;
impl Qualmode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qualmode0 {
        match self.bits {
            false => Qualmode0::Mask,
            true => Qualmode0::Extend,
        }
    }
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Qualmode0::Mask
    }
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    #[inline(always)]
    pub fn is_extend(&self) -> bool {
        *self == Qualmode0::Extend
    }
}
#[doc = "Field `QUALMODE0` writer - Qualify mode for slave address 0."]
pub type Qualmode0W<'a, REG> = crate::BitWriter<'a, REG, Qualmode0>;
impl<'a, REG> Qualmode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Qualmode0::Mask)
    }
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    #[inline(always)]
    pub fn extend(self) -> &'a mut crate::W<REG> {
        self.variant(Qualmode0::Extend)
    }
}
#[doc = "Field `SLVQUAL0` reader - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
&lt;= received address &lt;= SLVQUAL0\\[7:1\\])."]
pub type Slvqual0R = crate::FieldReader;
#[doc = "Field `SLVQUAL0` writer - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
&lt;= received address &lt;= SLVQUAL0\\[7:1\\])."]
pub type Slvqual0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    pub fn qualmode0(&self) -> Qualmode0R {
        Qualmode0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
&lt;= received address &lt;= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub fn slvqual0(&self) -> Slvqual0R {
        Slvqual0R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVQUAL0")
            .field("qualmode0", &self.qualmode0())
            .field("slvqual0", &self.slvqual0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    #[must_use]
    pub fn qualmode0(&mut self) -> Qualmode0W<Slvqual0Spec> {
        Qualmode0W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
&lt;= received address &lt;= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn slvqual0(&mut self) -> Slvqual0W<Slvqual0Spec> {
        Slvqual0W::new(self, 1)
    }
}
#[doc = "Slave Qualification for address 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvqual0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvqual0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slvqual0Spec;
impl crate::RegisterSpec for Slvqual0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvqual0::R`](R) reader structure"]
impl crate::Readable for Slvqual0Spec {}
#[doc = "`write(|w| ..)` method takes [`slvqual0::W`](W) writer structure"]
impl crate::Writable for Slvqual0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLVQUAL0 to value 0"]
impl crate::Resettable for Slvqual0Spec {
    const RESET_VALUE: u32 = 0;
}
