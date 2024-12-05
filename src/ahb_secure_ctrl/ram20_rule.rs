#[doc = "Register `RAM20_RULE[%s]` reader"]
pub type R = crate::R<Ram20RuleSpec>;
#[doc = "Register `RAM20_RULE[%s]` writer"]
pub type W = crate::W<Ram20RuleSpec>;
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
#[doc = "Rule 4\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule4 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule4> for u8 {
    #[inline(always)]
    fn from(variant: Rule4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule4 {
    type Ux = u8;
}
impl crate::IsEnum for Rule4 {}
#[doc = "Field `RULE4` reader - Rule 4"]
pub type Rule4R = crate::FieldReader<Rule4>;
impl Rule4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule4 {
        match self.bits {
            0 => Rule4::NonsecureNonprivUserAllowed,
            1 => Rule4::NonsecurePrivUserAllowed,
            2 => Rule4::SecureNonprivUserAllowed,
            3 => Rule4::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule4::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule4::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule4::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule4::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE4` writer - Rule 4"]
pub type Rule4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule4, crate::Safe>;
impl<'a, REG> Rule4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule4::SecurePrivUserAllowed)
    }
}
#[doc = "Rule 5\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule5 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule5> for u8 {
    #[inline(always)]
    fn from(variant: Rule5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule5 {
    type Ux = u8;
}
impl crate::IsEnum for Rule5 {}
#[doc = "Field `RULE5` reader - Rule 5"]
pub type Rule5R = crate::FieldReader<Rule5>;
impl Rule5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule5 {
        match self.bits {
            0 => Rule5::NonsecureNonprivUserAllowed,
            1 => Rule5::NonsecurePrivUserAllowed,
            2 => Rule5::SecureNonprivUserAllowed,
            3 => Rule5::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule5::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule5::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule5::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule5::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE5` writer - Rule 5"]
pub type Rule5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule5, crate::Safe>;
impl<'a, REG> Rule5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule5::SecurePrivUserAllowed)
    }
}
#[doc = "Rule 6\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule6 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule6> for u8 {
    #[inline(always)]
    fn from(variant: Rule6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule6 {
    type Ux = u8;
}
impl crate::IsEnum for Rule6 {}
#[doc = "Field `RULE6` reader - Rule 6"]
pub type Rule6R = crate::FieldReader<Rule6>;
impl Rule6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule6 {
        match self.bits {
            0 => Rule6::NonsecureNonprivUserAllowed,
            1 => Rule6::NonsecurePrivUserAllowed,
            2 => Rule6::SecureNonprivUserAllowed,
            3 => Rule6::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule6::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule6::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule6::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule6::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE6` writer - Rule 6"]
pub type Rule6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule6, crate::Safe>;
impl<'a, REG> Rule6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule6::SecurePrivUserAllowed)
    }
}
#[doc = "Rule 7\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rule7 {
    #[doc = "0: Non-secure and non-privilege user access allowed"]
    NonsecureNonprivUserAllowed = 0,
    #[doc = "1: Non-secure and privilege access allowed"]
    NonsecurePrivUserAllowed = 1,
    #[doc = "2: Secure and non-privilege user access allowed"]
    SecureNonprivUserAllowed = 2,
    #[doc = "3: Secure and privilege user access allowed"]
    SecurePrivUserAllowed = 3,
}
impl From<Rule7> for u8 {
    #[inline(always)]
    fn from(variant: Rule7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rule7 {
    type Ux = u8;
}
impl crate::IsEnum for Rule7 {}
#[doc = "Field `RULE7` reader - Rule 7"]
pub type Rule7R = crate::FieldReader<Rule7>;
impl Rule7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rule7 {
        match self.bits {
            0 => Rule7::NonsecureNonprivUserAllowed,
            1 => Rule7::NonsecurePrivUserAllowed,
            2 => Rule7::SecureNonprivUserAllowed,
            3 => Rule7::SecurePrivUserAllowed,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule7::NonsecureNonprivUserAllowed
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn is_nonsecure_priv_user_allowed(&self) -> bool {
        *self == Rule7::NonsecurePrivUserAllowed
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_nonpriv_user_allowed(&self) -> bool {
        *self == Rule7::SecureNonprivUserAllowed
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn is_secure_priv_user_allowed(&self) -> bool {
        *self == Rule7::SecurePrivUserAllowed
    }
}
#[doc = "Field `RULE7` writer - Rule 7"]
pub type Rule7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Rule7, crate::Safe>;
impl<'a, REG> Rule7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn nonsecure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::NonsecureNonprivUserAllowed)
    }
    #[doc = "Non-secure and privilege access allowed"]
    #[inline(always)]
    pub fn nonsecure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::NonsecurePrivUserAllowed)
    }
    #[doc = "Secure and non-privilege user access allowed"]
    #[inline(always)]
    pub fn secure_nonpriv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::SecureNonprivUserAllowed)
    }
    #[doc = "Secure and privilege user access allowed"]
    #[inline(always)]
    pub fn secure_priv_user_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Rule7::SecurePrivUserAllowed)
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
    #[doc = "Bits 16:17 - Rule 4"]
    #[inline(always)]
    pub fn rule4(&self) -> Rule4R {
        Rule4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Rule 5"]
    #[inline(always)]
    pub fn rule5(&self) -> Rule5R {
        Rule5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Rule 6"]
    #[inline(always)]
    pub fn rule6(&self) -> Rule6R {
        Rule6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Rule 7"]
    #[inline(always)]
    pub fn rule7(&self) -> Rule7R {
        Rule7R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAM20_RULE")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Rule 0"]
    #[inline(always)]
    pub fn rule0(&mut self) -> Rule0W<Ram20RuleSpec> {
        Rule0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Rule 1"]
    #[inline(always)]
    pub fn rule1(&mut self) -> Rule1W<Ram20RuleSpec> {
        Rule1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Rule 2"]
    #[inline(always)]
    pub fn rule2(&mut self) -> Rule2W<Ram20RuleSpec> {
        Rule2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Rule 3"]
    #[inline(always)]
    pub fn rule3(&mut self) -> Rule3W<Ram20RuleSpec> {
        Rule3W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Rule 4"]
    #[inline(always)]
    pub fn rule4(&mut self) -> Rule4W<Ram20RuleSpec> {
        Rule4W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Rule 5"]
    #[inline(always)]
    pub fn rule5(&mut self) -> Rule5W<Ram20RuleSpec> {
        Rule5W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Rule 6"]
    #[inline(always)]
    pub fn rule6(&mut self) -> Rule6W<Ram20RuleSpec> {
        Rule6W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Rule 7"]
    #[inline(always)]
    pub fn rule7(&mut self) -> Rule7W<Ram20RuleSpec> {
        Rule7W::new(self, 28)
    }
}
#[doc = "SRAM Partition 20 Rule(n) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ram20_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram20_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram20RuleSpec;
impl crate::RegisterSpec for Ram20RuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram20_rule::R`](R) reader structure"]
impl crate::Readable for Ram20RuleSpec {}
#[doc = "`write(|w| ..)` method takes [`ram20_rule::W`](W) writer structure"]
impl crate::Writable for Ram20RuleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM20_RULE[%s]
to value 0"]
impl crate::Resettable for Ram20RuleSpec {
    const RESET_VALUE: u32 = 0;
}
