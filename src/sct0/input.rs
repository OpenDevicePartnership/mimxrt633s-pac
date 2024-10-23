#[doc = "Register `INPUT` reader"]
pub type R = crate::R<InputSpec>;
#[doc = "Field `AIN0` reader - Input 0 state. Input 0 state on the last SCT clock edge."]
pub type Ain0R = crate::BitReader;
#[doc = "Field `AIN1` reader - Input 1 state. Input 1 state on the last SCT clock edge."]
pub type Ain1R = crate::BitReader;
#[doc = "Field `AIN2` reader - Input 2 state. Input 2 state on the last SCT clock edge."]
pub type Ain2R = crate::BitReader;
#[doc = "Field `AIN3` reader - Input 3 state. Input 3 state on the last SCT clock edge."]
pub type Ain3R = crate::BitReader;
#[doc = "Field `AIN4` reader - Input 4 state. Input 4 state on the last SCT clock edge."]
pub type Ain4R = crate::BitReader;
#[doc = "Field `AIN5` reader - Input 5 state. Input 5 state on the last SCT clock edge."]
pub type Ain5R = crate::BitReader;
#[doc = "Field `AIN6` reader - Input 6 state. Input 6 state on the last SCT clock edge."]
pub type Ain6R = crate::BitReader;
#[doc = "Field `AIN7` reader - Input 7 state. Input 7 state on the last SCT clock edge."]
pub type Ain7R = crate::BitReader;
#[doc = "Field `AIN8` reader - Input 8 state. Input 8 state on the last SCT clock edge."]
pub type Ain8R = crate::BitReader;
#[doc = "Field `AIN9` reader - Input 9 state. Input 9 state on the last SCT clock edge."]
pub type Ain9R = crate::BitReader;
#[doc = "Field `AIN10` reader - Input 10 state. Input 10 state on the last SCT clock edge."]
pub type Ain10R = crate::BitReader;
#[doc = "Field `AIN11` reader - Input 11 state. Input 11 state on the last SCT clock edge."]
pub type Ain11R = crate::BitReader;
#[doc = "Field `AIN12` reader - Input 12 state. Input 12 state on the last SCT clock edge."]
pub type Ain12R = crate::BitReader;
#[doc = "Field `AIN13` reader - Input 13 state. Input 13 state on the last SCT clock edge."]
pub type Ain13R = crate::BitReader;
#[doc = "Field `AIN14` reader - Input 14 state. Input 14 state on the last SCT clock edge."]
pub type Ain14R = crate::BitReader;
#[doc = "Field `AIN15` reader - Input 15 state. Input 15 state on the last SCT clock edge."]
pub type Ain15R = crate::BitReader;
#[doc = "Field `SIN0` reader - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
pub type Sin0R = crate::BitReader;
#[doc = "Field `SIN1` reader - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
pub type Sin1R = crate::BitReader;
#[doc = "Field `SIN2` reader - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
pub type Sin2R = crate::BitReader;
#[doc = "Field `SIN3` reader - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
pub type Sin3R = crate::BitReader;
#[doc = "Field `SIN4` reader - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
pub type Sin4R = crate::BitReader;
#[doc = "Field `SIN5` reader - Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
pub type Sin5R = crate::BitReader;
#[doc = "Field `SIN6` reader - Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
pub type Sin6R = crate::BitReader;
#[doc = "Field `SIN7` reader - Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
pub type Sin7R = crate::BitReader;
#[doc = "Field `SIN8` reader - Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
pub type Sin8R = crate::BitReader;
#[doc = "Field `SIN9` reader - Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
pub type Sin9R = crate::BitReader;
#[doc = "Field `SIN10` reader - Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
pub type Sin10R = crate::BitReader;
#[doc = "Field `SIN11` reader - Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
pub type Sin11R = crate::BitReader;
#[doc = "Field `SIN12` reader - Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
pub type Sin12R = crate::BitReader;
#[doc = "Field `SIN13` reader - Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
pub type Sin13R = crate::BitReader;
#[doc = "Field `SIN14` reader - Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
pub type Sin14R = crate::BitReader;
#[doc = "Field `SIN15` reader - Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
pub type Sin15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input 0 state. Input 0 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain0(&self) -> Ain0R {
        Ain0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input 1 state. Input 1 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain1(&self) -> Ain1R {
        Ain1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input 2 state. Input 2 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain2(&self) -> Ain2R {
        Ain2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input 3 state. Input 3 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain3(&self) -> Ain3R {
        Ain3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input 4 state. Input 4 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain4(&self) -> Ain4R {
        Ain4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input 5 state. Input 5 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain5(&self) -> Ain5R {
        Ain5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input 6 state. Input 6 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain6(&self) -> Ain6R {
        Ain6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input 7 state. Input 7 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain7(&self) -> Ain7R {
        Ain7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input 8 state. Input 8 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain8(&self) -> Ain8R {
        Ain8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input 9 state. Input 9 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain9(&self) -> Ain9R {
        Ain9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input 10 state. Input 10 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain10(&self) -> Ain10R {
        Ain10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input 11 state. Input 11 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain11(&self) -> Ain11R {
        Ain11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input 12 state. Input 12 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain12(&self) -> Ain12R {
        Ain12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input 13 state. Input 13 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain13(&self) -> Ain13R {
        Ain13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input 14 state. Input 14 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain14(&self) -> Ain14R {
        Ain14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input 15 state. Input 15 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain15(&self) -> Ain15R {
        Ain15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin0(&self) -> Sin0R {
        Sin0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin1(&self) -> Sin1R {
        Sin1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin2(&self) -> Sin2R {
        Sin2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin3(&self) -> Sin3R {
        Sin3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin4(&self) -> Sin4R {
        Sin4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin5(&self) -> Sin5R {
        Sin5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin6(&self) -> Sin6R {
        Sin6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin7(&self) -> Sin7R {
        Sin7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin8(&self) -> Sin8R {
        Sin8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin9(&self) -> Sin9R {
        Sin9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin10(&self) -> Sin10R {
        Sin10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin11(&self) -> Sin11R {
        Sin11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin12(&self) -> Sin12R {
        Sin12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin13(&self) -> Sin13R {
        Sin13R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin14(&self) -> Sin14R {
        Sin14R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin15(&self) -> Sin15R {
        Sin15R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUT")
            .field("ain0", &self.ain0())
            .field("ain1", &self.ain1())
            .field("ain2", &self.ain2())
            .field("ain3", &self.ain3())
            .field("ain4", &self.ain4())
            .field("ain5", &self.ain5())
            .field("ain6", &self.ain6())
            .field("ain7", &self.ain7())
            .field("ain8", &self.ain8())
            .field("ain9", &self.ain9())
            .field("ain10", &self.ain10())
            .field("ain11", &self.ain11())
            .field("ain12", &self.ain12())
            .field("ain13", &self.ain13())
            .field("ain14", &self.ain14())
            .field("ain15", &self.ain15())
            .field("sin0", &self.sin0())
            .field("sin1", &self.sin1())
            .field("sin2", &self.sin2())
            .field("sin3", &self.sin3())
            .field("sin4", &self.sin4())
            .field("sin5", &self.sin5())
            .field("sin6", &self.sin6())
            .field("sin7", &self.sin7())
            .field("sin8", &self.sin8())
            .field("sin9", &self.sin9())
            .field("sin10", &self.sin10())
            .field("sin11", &self.sin11())
            .field("sin12", &self.sin12())
            .field("sin13", &self.sin13())
            .field("sin14", &self.sin14())
            .field("sin15", &self.sin15())
            .finish()
    }
}
#[doc = "SCT input register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputSpec;
impl crate::RegisterSpec for InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for InputSpec {}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for InputSpec {
    const RESET_VALUE: u32 = 0;
}
