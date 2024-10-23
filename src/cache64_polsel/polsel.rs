#[doc = "Register `POLSEL` reader"]
pub type R = crate::R<PolselSpec>;
#[doc = "Register `POLSEL` writer"]
pub type W = crate::W<PolselSpec>;
#[doc = "Policy Select for Region 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg0Policy {
    #[doc = "0: Non-cache"]
    Reg0_00 = 0,
    #[doc = "1: Write-thru"]
    Reg0_01 = 1,
    #[doc = "2: Write-back"]
    Reg0_10 = 2,
    #[doc = "3: Invalid"]
    Reg0_11 = 3,
}
impl From<Reg0Policy> for u8 {
    #[inline(always)]
    fn from(variant: Reg0Policy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reg0Policy {
    type Ux = u8;
}
impl crate::IsEnum for Reg0Policy {}
#[doc = "Field `REG0_POLICY` reader - Policy Select for Region 0"]
pub type Reg0PolicyR = crate::FieldReader<Reg0Policy>;
impl Reg0PolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reg0Policy {
        match self.bits {
            0 => Reg0Policy::Reg0_00,
            1 => Reg0Policy::Reg0_01,
            2 => Reg0Policy::Reg0_10,
            3 => Reg0Policy::Reg0_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-cache"]
    #[inline(always)]
    pub fn is_reg0_00(&self) -> bool {
        *self == Reg0Policy::Reg0_00
    }
    #[doc = "Write-thru"]
    #[inline(always)]
    pub fn is_reg0_01(&self) -> bool {
        *self == Reg0Policy::Reg0_01
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn is_reg0_10(&self) -> bool {
        *self == Reg0Policy::Reg0_10
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_reg0_11(&self) -> bool {
        *self == Reg0Policy::Reg0_11
    }
}
#[doc = "Field `REG0_POLICY` writer - Policy Select for Region 0"]
pub type Reg0PolicyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Reg0Policy, crate::Safe>;
impl<'a, REG> Reg0PolicyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-cache"]
    #[inline(always)]
    pub fn reg0_00(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0Policy::Reg0_00)
    }
    #[doc = "Write-thru"]
    #[inline(always)]
    pub fn reg0_01(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0Policy::Reg0_01)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn reg0_10(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0Policy::Reg0_10)
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn reg0_11(self) -> &'a mut crate::W<REG> {
        self.variant(Reg0Policy::Reg0_11)
    }
}
#[doc = "Policy Select for Region 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg1Policy {
    #[doc = "0: Non-cache"]
    Reg1_00 = 0,
    #[doc = "1: Write-thru"]
    Reg1_01 = 1,
    #[doc = "2: Write-back"]
    Reg1_10 = 2,
    #[doc = "3: Invalid"]
    Reg1_11 = 3,
}
impl From<Reg1Policy> for u8 {
    #[inline(always)]
    fn from(variant: Reg1Policy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reg1Policy {
    type Ux = u8;
}
impl crate::IsEnum for Reg1Policy {}
#[doc = "Field `REG1_POLICY` reader - Policy Select for Region 0"]
pub type Reg1PolicyR = crate::FieldReader<Reg1Policy>;
impl Reg1PolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reg1Policy {
        match self.bits {
            0 => Reg1Policy::Reg1_00,
            1 => Reg1Policy::Reg1_01,
            2 => Reg1Policy::Reg1_10,
            3 => Reg1Policy::Reg1_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-cache"]
    #[inline(always)]
    pub fn is_reg1_00(&self) -> bool {
        *self == Reg1Policy::Reg1_00
    }
    #[doc = "Write-thru"]
    #[inline(always)]
    pub fn is_reg1_01(&self) -> bool {
        *self == Reg1Policy::Reg1_01
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn is_reg1_10(&self) -> bool {
        *self == Reg1Policy::Reg1_10
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_reg1_11(&self) -> bool {
        *self == Reg1Policy::Reg1_11
    }
}
#[doc = "Field `REG1_POLICY` writer - Policy Select for Region 0"]
pub type Reg1PolicyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Reg1Policy, crate::Safe>;
impl<'a, REG> Reg1PolicyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-cache"]
    #[inline(always)]
    pub fn reg1_00(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1Policy::Reg1_00)
    }
    #[doc = "Write-thru"]
    #[inline(always)]
    pub fn reg1_01(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1Policy::Reg1_01)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn reg1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1Policy::Reg1_10)
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn reg1_11(self) -> &'a mut crate::W<REG> {
        self.variant(Reg1Policy::Reg1_11)
    }
}
#[doc = "Policy Select for Region 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg02Policy {
    #[doc = "0: Non-cache"]
    Reg2_00 = 0,
    #[doc = "1: Write-thru"]
    Reg2_01 = 1,
    #[doc = "2: Write-back"]
    Reg2_10 = 2,
    #[doc = "3: Invalid"]
    Reg2_11 = 3,
}
impl From<Reg02Policy> for u8 {
    #[inline(always)]
    fn from(variant: Reg02Policy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reg02Policy {
    type Ux = u8;
}
impl crate::IsEnum for Reg02Policy {}
#[doc = "Field `REG02_POLICY` reader - Policy Select for Region 0"]
pub type Reg02PolicyR = crate::FieldReader<Reg02Policy>;
impl Reg02PolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reg02Policy {
        match self.bits {
            0 => Reg02Policy::Reg2_00,
            1 => Reg02Policy::Reg2_01,
            2 => Reg02Policy::Reg2_10,
            3 => Reg02Policy::Reg2_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-cache"]
    #[inline(always)]
    pub fn is_reg2_00(&self) -> bool {
        *self == Reg02Policy::Reg2_00
    }
    #[doc = "Write-thru"]
    #[inline(always)]
    pub fn is_reg2_01(&self) -> bool {
        *self == Reg02Policy::Reg2_01
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn is_reg2_10(&self) -> bool {
        *self == Reg02Policy::Reg2_10
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn is_reg2_11(&self) -> bool {
        *self == Reg02Policy::Reg2_11
    }
}
#[doc = "Field `REG02_POLICY` writer - Policy Select for Region 0"]
pub type Reg02PolicyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Reg02Policy, crate::Safe>;
impl<'a, REG> Reg02PolicyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-cache"]
    #[inline(always)]
    pub fn reg2_00(self) -> &'a mut crate::W<REG> {
        self.variant(Reg02Policy::Reg2_00)
    }
    #[doc = "Write-thru"]
    #[inline(always)]
    pub fn reg2_01(self) -> &'a mut crate::W<REG> {
        self.variant(Reg02Policy::Reg2_01)
    }
    #[doc = "Write-back"]
    #[inline(always)]
    pub fn reg2_10(self) -> &'a mut crate::W<REG> {
        self.variant(Reg02Policy::Reg2_10)
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn reg2_11(self) -> &'a mut crate::W<REG> {
        self.variant(Reg02Policy::Reg2_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Policy Select for Region 0"]
    #[inline(always)]
    pub fn reg0_policy(&self) -> Reg0PolicyR {
        Reg0PolicyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Policy Select for Region 0"]
    #[inline(always)]
    pub fn reg1_policy(&self) -> Reg1PolicyR {
        Reg1PolicyR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Policy Select for Region 0"]
    #[inline(always)]
    pub fn reg02_policy(&self) -> Reg02PolicyR {
        Reg02PolicyR::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLSEL")
            .field("reg0_policy", &self.reg0_policy())
            .field("reg1_policy", &self.reg1_policy())
            .field("reg02_policy", &self.reg02_policy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Policy Select for Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg0_policy(&mut self) -> Reg0PolicyW<PolselSpec> {
        Reg0PolicyW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Policy Select for Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg1_policy(&mut self) -> Reg1PolicyW<PolselSpec> {
        Reg1PolicyW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Policy Select for Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg02_policy(&mut self) -> Reg02PolicyW<PolselSpec> {
        Reg02PolicyW::new(self, 4)
    }
}
#[doc = "Policy Select\n\nYou can [`read`](crate::Reg::read) this register and get [`polsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolselSpec;
impl crate::RegisterSpec for PolselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polsel::R`](R) reader structure"]
impl crate::Readable for PolselSpec {}
#[doc = "`write(|w| ..)` method takes [`polsel::W`](W) writer structure"]
impl crate::Writable for PolselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLSEL to value 0"]
impl crate::Resettable for PolselSpec {
    const RESET_VALUE: u32 = 0;
}
