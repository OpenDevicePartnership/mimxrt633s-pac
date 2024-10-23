#[doc = "Register `FLEXSPI0_REGION3_RULE0` reader"]
pub type R = crate::R<Flexspi0Region3Rule0Spec>;
#[doc = "Register `FLEXSPI0_REGION3_RULE0` writer"]
pub type W = crate::W<Flexspi0Region3Rule0Spec>;
#[doc = "Rule 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule0 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule0> for u8 {
    #[inline(always)]
    fn from(variant: Rule0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule0 {
    type Ux = u8;
}
impl crate::IsEnum for Rule0 {}
#[doc = "Field `RULE0` reader - Rule 0"]
pub type Rule0R = crate::FieldReader<Rule0>;
impl Rule0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule0 {
        match self.bits {
            0 => Rule0::NonsecureNonprivUserAllowed,
            1 => Rule0::NonsecurePrivUserAllowed,
            2 => Rule0::SecureNonprivUserAllowed,
            3 => Rule0::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule0::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule0::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule0::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule0::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE0` writer - Rule 0"]
pub type Rule0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule0, crate::Safe>;
impl<'a, REG> Rule0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule0::SecurePrivUserAllowed)
    }
}
#[doc = "Rule 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule1 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule1> for u8 {
    #[inline(always)]
    fn from(variant: Rule1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule1 {
    type Ux = u8;
}
impl crate::IsEnum for Rule1 {}
#[doc = "Field `RULE1` reader - Rule 1"]
pub type Rule1R = crate::FieldReader<Rule1>;
impl Rule1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule1 {
        match self.bits {
            0 => Rule1::NonsecureNonprivUserAllowed,
            1 => Rule1::NonsecurePrivUserAllowed,
            2 => Rule1::SecureNonprivUserAllowed,
            3 => Rule1::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule1::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule1::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule1::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule1::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE1` writer - Rule 1"]
pub type Rule1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule1, crate::Safe>;
impl<'a, REG> Rule1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule1::SecurePrivUserAllowed)
    }
}
#[doc = "Rule 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule2 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule2> for u8 {
    #[inline(always)]
    fn from(variant: Rule2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule2 {
    type Ux = u8;
}
impl crate::IsEnum for Rule2 {}
#[doc = "Field `RULE2` reader - Rule 2"]
pub type Rule2R = crate::FieldReader<Rule2>;
impl Rule2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule2 {
        match self.bits {
            0 => Rule2::NonsecureNonprivUserAllowed,
            1 => Rule2::NonsecurePrivUserAllowed,
            2 => Rule2::SecureNonprivUserAllowed,
            3 => Rule2::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule2::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule2::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule2::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule2::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE2` writer - Rule 2"]
pub type Rule2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule2, crate::Safe>;
impl<'a, REG> Rule2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule2::SecurePrivUserAllowed)
    }
}
#[doc = "Rule 3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule3 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule3> for u8 {
    #[inline(always)]
    fn from(variant: Rule3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule3 {
    type Ux = u8;
}
impl crate::IsEnum for Rule3 {}
#[doc = "Field `RULE3` reader - Rule 3"]
pub type Rule3R = crate::FieldReader<Rule3>;
impl Rule3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule3 {
        match self.bits {
            0 => Rule3::NonsecureNonprivUserAllowed,
            1 => Rule3::NonsecurePrivUserAllowed,
            2 => Rule3::SecureNonprivUserAllowed,
            3 => Rule3::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule3::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule3::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule3::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule3::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE3` writer - Rule 3"]
pub type Rule3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule3, crate::Safe>;
impl<'a, REG> Rule3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule3::SecurePrivUserAllowed)
    }
}
impl R {
    #[doc = "Bits 0:1 - Rule 0"]
    #[inline(always)]
    pub fn rule0(&self) -> Rule0R {
        Rule0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Rule 1"]
    #[inline(always)]
    pub fn rule1(&self) -> Rule1R {
        Rule1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Rule 2"]
    #[inline(always)]
    pub fn rule2(&self) -> Rule2R {
        Rule2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Rule 3"]
    #[inline(always)]
    pub fn rule3(&self) -> Rule3R {
        Rule3R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI0_REGION3_RULE0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Rule 0"]
    #[inline(always)]
    #[must_use]
    pub fn rule0(&mut self) -> Rule0W<Flexspi0Region3Rule0Spec> {
        Rule0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Rule 1"]
    #[inline(always)]
    #[must_use]
    pub fn rule1(&mut self) -> Rule1W<Flexspi0Region3Rule0Spec> {
        Rule1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Rule 2"]
    #[inline(always)]
    #[must_use]
    pub fn rule2(&mut self) -> Rule2W<Flexspi0Region3Rule0Spec> {
        Rule2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Rule 3"]
    #[inline(always)]
    #[must_use]
    pub fn rule3(&mut self) -> Rule3W<Flexspi0Region3Rule0Spec> {
        Rule3W::new(self, 12)
    }
}
#[doc = "FLEXSPI0 Region 3 Rule 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexspi0_region3_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexspi0_region3_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flexspi0Region3Rule0Spec;
impl crate::RegisterSpec for Flexspi0Region3Rule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexspi0_region3_rule0::R`](R) reader structure"]
impl crate::Readable for Flexspi0Region3Rule0Spec {}
#[doc = "`write(|w| ..)` method takes [`flexspi0_region3_rule0::W`](W) writer structure"]
impl crate::Writable for Flexspi0Region3Rule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLEXSPI0_REGION3_RULE0 to value 0"]
impl crate::Resettable for Flexspi0Region3Rule0Spec {
    const RESET_VALUE: u32 = 0;
}
