#[doc = "Register `KEYLOCK` reader"]
pub type R = crate::R<KeylockSpec>;
#[doc = "Register `KEYLOCK` writer"]
pub type W = crate::W<KeylockSpec>;
#[doc = "Key 0\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key0 {
    #[doc = "0: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    Key0lock0 = 0,
    #[doc = "1: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    Key0lock1 = 1,
    #[doc = "2: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is allowed."]
    Key0unlock = 2,
    #[doc = "3: Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    Key0lock3 = 3,
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
            0 => Key0::Key0lock0,
            1 => Key0::Key0lock1,
            2 => Key0::Key0unlock,
            3 => Key0::Key0lock3,
            _ => unreachable!(),
        }
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key0lock_0(&self) -> bool {
        *self == Key0::Key0lock0
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key0lock_1(&self) -> bool {
        *self == Key0::Key0lock1
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is allowed."]
    #[inline(always)]
    pub fn is_key0unlock(&self) -> bool {
        *self == Key0::Key0unlock
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key0lock_3(&self) -> bool {
        *self == Key0::Key0lock3
    }
}
#[doc = "Field `KEY0` writer - Key 0"]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key0, crate::Safe>;
impl<'a, REG> Key0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key0lock_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Key0lock0)
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key0lock_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Key0lock1)
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is allowed."]
    #[inline(always)]
    pub fn key0unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Key0unlock)
    }
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\]
and KEYRESET\\[KEY0\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key0lock_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key0::Key0lock3)
    }
}
#[doc = "Key 1\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key1 {
    #[doc = "0: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    Key1lock0 = 0,
    #[doc = "1: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    Key1lock1 = 1,
    #[doc = "2: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is allowed."]
    Key1unlock = 2,
    #[doc = "3: Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    Key1lock3 = 3,
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
            0 => Key1::Key1lock0,
            1 => Key1::Key1lock1,
            2 => Key1::Key1unlock,
            3 => Key1::Key1lock3,
            _ => unreachable!(),
        }
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key1lock_0(&self) -> bool {
        *self == Key1::Key1lock0
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key1lock_1(&self) -> bool {
        *self == Key1::Key1lock1
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is allowed."]
    #[inline(always)]
    pub fn is_key1unlock(&self) -> bool {
        *self == Key1::Key1unlock
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key1lock_3(&self) -> bool {
        *self == Key1::Key1lock3
    }
}
#[doc = "Field `KEY1` writer - Key 1"]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key1, crate::Safe>;
impl<'a, REG> Key1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key1lock_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Key1lock0)
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key1lock_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Key1lock1)
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is allowed."]
    #[inline(always)]
    pub fn key1unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Key1unlock)
    }
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\]
and KEYRESET\\[KEY1\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key1lock_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key1::Key1lock3)
    }
}
#[doc = "Key 2\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key2 {
    #[doc = "0: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    Key2lock0 = 0,
    #[doc = "1: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    Key2lock1 = 1,
    #[doc = "2: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is allowed."]
    Key2unlock = 2,
    #[doc = "3: Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    Key2lock3 = 3,
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
            0 => Key2::Key2lock0,
            1 => Key2::Key2lock1,
            2 => Key2::Key2unlock,
            3 => Key2::Key2lock3,
            _ => unreachable!(),
        }
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key2lock_0(&self) -> bool {
        *self == Key2::Key2lock0
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key2lock_1(&self) -> bool {
        *self == Key2::Key2lock1
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is allowed."]
    #[inline(always)]
    pub fn is_key2unlock(&self) -> bool {
        *self == Key2::Key2unlock
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key2lock_3(&self) -> bool {
        *self == Key2::Key2lock3
    }
}
#[doc = "Field `KEY2` writer - Key 2"]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key2, crate::Safe>;
impl<'a, REG> Key2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key2lock_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Key2lock0)
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key2lock_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Key2lock1)
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is allowed."]
    #[inline(always)]
    pub fn key2unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Key2unlock)
    }
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\]
and KEYRESET\\[KEY2\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key2lock_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key2::Key2lock3)
    }
}
#[doc = "Key 3\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key3 {
    #[doc = "0: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    Key3lock0 = 0,
    #[doc = "1: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    Key3lock1 = 1,
    #[doc = "2: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is allowed."]
    Key3unlock = 2,
    #[doc = "3: Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    Key3lock3 = 3,
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
            0 => Key3::Key3lock0,
            1 => Key3::Key3lock1,
            2 => Key3::Key3unlock,
            3 => Key3::Key3lock3,
            _ => unreachable!(),
        }
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key3lock_0(&self) -> bool {
        *self == Key3::Key3lock0
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key3lock_1(&self) -> bool {
        *self == Key3::Key3lock1
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is allowed."]
    #[inline(always)]
    pub fn is_key3unlock(&self) -> bool {
        *self == Key3::Key3unlock
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn is_key3lock_3(&self) -> bool {
        *self == Key3::Key3lock3
    }
}
#[doc = "Field `KEY3` writer - Key 3"]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Key3, crate::Safe>;
impl<'a, REG> Key3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key3lock_0(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Key3lock0)
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key3lock_1(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Key3lock1)
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is allowed."]
    #[inline(always)]
    pub fn key3unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Key3unlock)
    }
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\]
and KEYRESET\\[KEY3\\]
is NOT allowed."]
    #[inline(always)]
    pub fn key3lock_3(self) -> &'a mut crate::W<REG> {
        self.variant(Key3::Key3lock3)
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
        f.debug_struct("KEYLOCK")
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
    #[must_use]
    pub fn key0(&mut self) -> Key0W<KeylockSpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Key 1"]
    #[inline(always)]
    #[must_use]
    pub fn key1(&mut self) -> Key1W<KeylockSpec> {
        Key1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Key 2"]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> Key2W<KeylockSpec> {
        Key2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Key 3"]
    #[inline(always)]
    #[must_use]
    pub fn key3(&mut self) -> Key3W<KeylockSpec> {
        Key3W::new(self, 6)
    }
}
#[doc = "Key Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`keylock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keylock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeylockSpec;
impl crate::RegisterSpec for KeylockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keylock::R`](R) reader structure"]
impl crate::Readable for KeylockSpec {}
#[doc = "`write(|w| ..)` method takes [`keylock::W`](W) writer structure"]
impl crate::Writable for KeylockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYLOCK to value 0xaa"]
impl crate::Resettable for KeylockSpec {
    const RESET_VALUE: u32 = 0xaa;
}
