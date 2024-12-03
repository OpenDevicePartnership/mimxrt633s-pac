#[doc = "Register `KEYENABLE` reader"]
pub type R = crate::R<KeyenableSpec>;
#[doc = "Register `KEYENABLE` writer"]
pub type W = crate::W<KeyenableSpec>;
#[doc = "Key 0\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key0 {
    #[doc = "0: Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    Disabled0 = 0,
    #[doc = "1: Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    Disabled1 = 1,
    #[doc = "2: Data coming from the PUF Index 0 interface are shifted in the KEY0 register."]
    Enabled = 2,
    #[doc = "3: Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    Disabled3 = 3,
}
impl From<Key0> for u8 {
    #[inline(always)]
    fn from(variant: Key0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key0 {
    type Ux = u8;
}
impl crate::IsEnum for Key0 {}
#[doc = "Field `KEY0` reader - Key 0"]
pub type Key0R = crate::FieldReader<Key0>;
impl Key0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Key0 {
        match self.bits {
            0 => Key0::Disabled0,
            1 => Key0::Disabled1,
            2 => Key0::Enabled,
            3 => Key0::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == Key0::Disabled0
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == Key0::Disabled1
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY0 register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Key0::Enabled
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == Key0::Disabled3
    }
}
#[doc = "Field `KEY0` writer - Key 0"]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key0, crate::Safe>;
impl<'a, REG> Key0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Disabled0)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Disabled1)
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY0 register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Enabled)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Disabled3)
    }
}
#[doc = "Key 1\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key1 {
    #[doc = "0: Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    Disabled0 = 0,
    #[doc = "1: Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    Disabled1 = 1,
    #[doc = "2: Data coming from the PUF Index 0 interface are shifted in the KEY1 register."]
    Enabled = 2,
    #[doc = "3: Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    Disabled3 = 3,
}
impl From<Key1> for u8 {
    #[inline(always)]
    fn from(variant: Key1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key1 {
    type Ux = u8;
}
impl crate::IsEnum for Key1 {}
#[doc = "Field `KEY1` reader - Key 1"]
pub type Key1R = crate::FieldReader<Key1>;
impl Key1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Key1 {
        match self.bits {
            0 => Key1::Disabled0,
            1 => Key1::Disabled1,
            2 => Key1::Enabled,
            3 => Key1::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == Key1::Disabled0
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == Key1::Disabled1
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY1 register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Key1::Enabled
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == Key1::Disabled3
    }
}
#[doc = "Field `KEY1` writer - Key 1"]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key1, crate::Safe>;
impl<'a, REG> Key1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Disabled0)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Disabled1)
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY1 register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Enabled)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Disabled3)
    }
}
#[doc = "Key 2\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key2 {
    #[doc = "0: Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    Disabled0 = 0,
    #[doc = "1: Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    Disabled1 = 1,
    #[doc = "2: Data coming from the PUF Index 0 interface are shifted in the KEY2 register."]
    Enabled = 2,
    #[doc = "3: Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    Disabled3 = 3,
}
impl From<Key2> for u8 {
    #[inline(always)]
    fn from(variant: Key2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key2 {
    type Ux = u8;
}
impl crate::IsEnum for Key2 {}
#[doc = "Field `KEY2` reader - Key 2"]
pub type Key2R = crate::FieldReader<Key2>;
impl Key2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Key2 {
        match self.bits {
            0 => Key2::Disabled0,
            1 => Key2::Disabled1,
            2 => Key2::Enabled,
            3 => Key2::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == Key2::Disabled0
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == Key2::Disabled1
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY2 register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Key2::Enabled
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == Key2::Disabled3
    }
}
#[doc = "Field `KEY2` writer - Key 2"]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key2, crate::Safe>;
impl<'a, REG> Key2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Disabled0)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Disabled1)
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY2 register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Enabled)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Disabled3)
    }
}
#[doc = "Key 3\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key3 {
    #[doc = "0: Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    Disabled0 = 0,
    #[doc = "1: Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    Disabled1 = 1,
    #[doc = "2: Data coming from the PUF Index 0 interface are shifted in the KEY3 register."]
    Enabled = 2,
    #[doc = "3: Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    Disabled3 = 3,
}
impl From<Key3> for u8 {
    #[inline(always)]
    fn from(variant: Key3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key3 {
    type Ux = u8;
}
impl crate::IsEnum for Key3 {}
#[doc = "Field `KEY3` reader - Key 3"]
pub type Key3R = crate::FieldReader<Key3>;
impl Key3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Key3 {
        match self.bits {
            0 => Key3::Disabled0,
            1 => Key3::Disabled1,
            2 => Key3::Enabled,
            3 => Key3::Disabled3,
            _ => unreachable!(),
        }
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn is_disabled_0(&self) -> bool {
        *self == Key3::Disabled0
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn is_disabled_1(&self) -> bool {
        *self == Key3::Disabled1
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY3 register."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Key3::Enabled
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn is_disabled_3(&self) -> bool {
        *self == Key3::Disabled3
    }
}
#[doc = "Field `KEY3` writer - Key 3"]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key3, crate::Safe>;
impl<'a, REG> Key3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn disabled_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Disabled0)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn disabled_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Disabled1)
    }
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY3 register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Enabled)
    }
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    #[inline(always)]
    pub fn disabled_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Disabled3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Key 0"]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Key 1"]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Key 2"]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Key 3"]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYENABLE")
            .field("key0", &self.key0())
            .field("key1", &self.key1())
            .field("key2", &self.key2())
            .field("key3", &self.key3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Key 0"]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<KeyenableSpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Key 1"]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<KeyenableSpec> {
        Key1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Key 2"]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<KeyenableSpec> {
        Key2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Key 3"]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<KeyenableSpec> {
        Key3W::new(self, 6)
    }
}
#[doc = "Key Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`keyenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyenableSpec;
impl crate::RegisterSpec for KeyenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyenable::R`](R) reader structure"]
impl crate::Readable for KeyenableSpec {}
#[doc = "`write(|w| ..)` method takes [`keyenable::W`](W) writer structure"]
impl crate::Writable for KeyenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYENABLE to value 0x55"]
impl crate::Resettable for KeyenableSpec {
    const RESET_VALUE: u32 = 0x55;
}
