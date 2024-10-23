#[doc = "Register `DMAC0_OTRIG_SEL%s` reader"]
pub type R = crate::R<Dmac0OtrigSelSpec>;
#[doc = "Register `DMAC0_OTRIG_SEL%s` writer"]
pub type W = crate::W<Dmac0OtrigSelSpec>;
#[doc = "DMAC0 Output Triggers Select for A, B, C, D IE.,DMAC0_OTRIG_A, DMAC0_OTRIG_B, DMAC0_OTRIG_C, DMAC0_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . .\n\nValue on reset: 63"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmac0OtrigSel {
    #[doc = "0: DMAC0_OTRIG_CH0"]
    Dmac0OtrigCh0 = 0,
    #[doc = "1: DMAC0_OTRIG_CH1"]
    Dmac0OtrigCh1 = 1,
    #[doc = "2: DMAC0_OTRIG_CH2"]
    Dmac0OtrigCh2 = 2,
    #[doc = "3: DMAC0_OTRIG_CH3"]
    Dmac0OtrigCh3 = 3,
    #[doc = "4: DMAC0_OTRIG_CH4"]
    Dmac0OtrigCh4 = 4,
    #[doc = "5: DMAC0_OTRIG_CH5"]
    Dmac0OtrigCh5 = 5,
    #[doc = "6: DMAC0_OTRIG_CH6"]
    Dmac0OtrigCh6 = 6,
    #[doc = "7: DMAC0_OTRIG_CH7"]
    Dmac0OtrigCh7 = 7,
    #[doc = "8: DMAC0_OTRIG_CH8"]
    Dmac0OtrigCh8 = 8,
    #[doc = "9: DMAC0_OTRIG_CH9"]
    Dmac0OtrigCh9 = 9,
    #[doc = "10: DMAC0_OTRIG_CH10"]
    Dmac0OtrigCh10 = 10,
    #[doc = "11: DMAC0_OTRIG_CH11"]
    Dmac0OtrigCh11 = 11,
    #[doc = "12: DMAC0_OTRIG_CH12"]
    Dmac0OtrigCh12 = 12,
    #[doc = "13: DMAC0_OTRIG_CH13"]
    Dmac0OtrigCh13 = 13,
    #[doc = "14: DMAC0_OTRIG_CH14"]
    Dmac0OtrigCh14 = 14,
    #[doc = "15: DMAC0_OTRIG_CH15"]
    Dmac0OtrigCh15 = 15,
    #[doc = "16: DMAC0_OTRIG_CH16"]
    Dmac0OtrigCh16 = 16,
    #[doc = "17: DMAC0_OTRIG_CH17"]
    Dmac0OtrigCh17 = 17,
    #[doc = "18: DMAC0_OTRIG_CH18"]
    Dmac0OtrigCh18 = 18,
    #[doc = "19: DMAC0_OTRIG_CH19"]
    Dmac0OtrigCh19 = 19,
    #[doc = "20: DMAC0_OTRIG_CH20"]
    Dmac0OtrigCh20 = 20,
    #[doc = "21: DMAC0_OTRIG_CH21"]
    Dmac0OtrigCh21 = 21,
    #[doc = "22: DMAC0_OTRIG_CH22"]
    Dmac0OtrigCh22 = 22,
    #[doc = "23: DMAC0_OTRIG_CH23"]
    Dmac0OtrigCh23 = 23,
    #[doc = "24: DMAC0_OTRIG_CH24"]
    Dmac0OtrigCh24 = 24,
    #[doc = "25: DMAC0_OTRIG_CH25"]
    Dmac0OtrigCh25 = 25,
    #[doc = "26: DMAC0_OTRIG_CH26"]
    Dmac0OtrigCh26 = 26,
    #[doc = "27: DMAC0_OTRIG_CH27"]
    Dmac0OtrigCh27 = 27,
    #[doc = "28: DMAC0_OTRIG_CH28"]
    Dmac0OtrigCh28 = 28,
    #[doc = "29: DMAC0_OTRIG_CH29"]
    Dmac0OtrigCh29 = 29,
    #[doc = "30: DMAC0_OTRIG_CH30"]
    Dmac0OtrigCh30 = 30,
    #[doc = "31: DMAC0_OTRIG_CH31"]
    Dmac0OtrigCh31 = 31,
    #[doc = "32: DMAC0_OTRIG_CH32"]
    Dmac0OtrigCh32 = 32,
}
impl From<Dmac0OtrigSel> for u8 {
    #[inline(always)]
    fn from(variant: Dmac0OtrigSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmac0OtrigSel {
    type Ux = u8;
}
impl crate::IsEnum for Dmac0OtrigSel {}
#[doc = "Field `DMAC0_OTRIG_SEL` reader - DMAC0 Output Triggers Select for A, B, C, D IE.,DMAC0_OTRIG_A, DMAC0_OTRIG_B, DMAC0_OTRIG_C, DMAC0_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
pub type Dmac0OtrigSelR = crate::FieldReader<Dmac0OtrigSel>;
impl Dmac0OtrigSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmac0OtrigSel> {
        match self.bits {
            0 => Some(Dmac0OtrigSel::Dmac0OtrigCh0),
            1 => Some(Dmac0OtrigSel::Dmac0OtrigCh1),
            2 => Some(Dmac0OtrigSel::Dmac0OtrigCh2),
            3 => Some(Dmac0OtrigSel::Dmac0OtrigCh3),
            4 => Some(Dmac0OtrigSel::Dmac0OtrigCh4),
            5 => Some(Dmac0OtrigSel::Dmac0OtrigCh5),
            6 => Some(Dmac0OtrigSel::Dmac0OtrigCh6),
            7 => Some(Dmac0OtrigSel::Dmac0OtrigCh7),
            8 => Some(Dmac0OtrigSel::Dmac0OtrigCh8),
            9 => Some(Dmac0OtrigSel::Dmac0OtrigCh9),
            10 => Some(Dmac0OtrigSel::Dmac0OtrigCh10),
            11 => Some(Dmac0OtrigSel::Dmac0OtrigCh11),
            12 => Some(Dmac0OtrigSel::Dmac0OtrigCh12),
            13 => Some(Dmac0OtrigSel::Dmac0OtrigCh13),
            14 => Some(Dmac0OtrigSel::Dmac0OtrigCh14),
            15 => Some(Dmac0OtrigSel::Dmac0OtrigCh15),
            16 => Some(Dmac0OtrigSel::Dmac0OtrigCh16),
            17 => Some(Dmac0OtrigSel::Dmac0OtrigCh17),
            18 => Some(Dmac0OtrigSel::Dmac0OtrigCh18),
            19 => Some(Dmac0OtrigSel::Dmac0OtrigCh19),
            20 => Some(Dmac0OtrigSel::Dmac0OtrigCh20),
            21 => Some(Dmac0OtrigSel::Dmac0OtrigCh21),
            22 => Some(Dmac0OtrigSel::Dmac0OtrigCh22),
            23 => Some(Dmac0OtrigSel::Dmac0OtrigCh23),
            24 => Some(Dmac0OtrigSel::Dmac0OtrigCh24),
            25 => Some(Dmac0OtrigSel::Dmac0OtrigCh25),
            26 => Some(Dmac0OtrigSel::Dmac0OtrigCh26),
            27 => Some(Dmac0OtrigSel::Dmac0OtrigCh27),
            28 => Some(Dmac0OtrigSel::Dmac0OtrigCh28),
            29 => Some(Dmac0OtrigSel::Dmac0OtrigCh29),
            30 => Some(Dmac0OtrigSel::Dmac0OtrigCh30),
            31 => Some(Dmac0OtrigSel::Dmac0OtrigCh31),
            32 => Some(Dmac0OtrigSel::Dmac0OtrigCh32),
            _ => None,
        }
    }
    #[doc = "DMAC0_OTRIG_CH0"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch0(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh0
    }
    #[doc = "DMAC0_OTRIG_CH1"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch1(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh1
    }
    #[doc = "DMAC0_OTRIG_CH2"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch2(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh2
    }
    #[doc = "DMAC0_OTRIG_CH3"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch3(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh3
    }
    #[doc = "DMAC0_OTRIG_CH4"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch4(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh4
    }
    #[doc = "DMAC0_OTRIG_CH5"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch5(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh5
    }
    #[doc = "DMAC0_OTRIG_CH6"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch6(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh6
    }
    #[doc = "DMAC0_OTRIG_CH7"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch7(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh7
    }
    #[doc = "DMAC0_OTRIG_CH8"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch8(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh8
    }
    #[doc = "DMAC0_OTRIG_CH9"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch9(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh9
    }
    #[doc = "DMAC0_OTRIG_CH10"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch10(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh10
    }
    #[doc = "DMAC0_OTRIG_CH11"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch11(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh11
    }
    #[doc = "DMAC0_OTRIG_CH12"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch12(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh12
    }
    #[doc = "DMAC0_OTRIG_CH13"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch13(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh13
    }
    #[doc = "DMAC0_OTRIG_CH14"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch14(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh14
    }
    #[doc = "DMAC0_OTRIG_CH15"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch15(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh15
    }
    #[doc = "DMAC0_OTRIG_CH16"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch16(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh16
    }
    #[doc = "DMAC0_OTRIG_CH17"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch17(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh17
    }
    #[doc = "DMAC0_OTRIG_CH18"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch18(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh18
    }
    #[doc = "DMAC0_OTRIG_CH19"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch19(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh19
    }
    #[doc = "DMAC0_OTRIG_CH20"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch20(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh20
    }
    #[doc = "DMAC0_OTRIG_CH21"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch21(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh21
    }
    #[doc = "DMAC0_OTRIG_CH22"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch22(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh22
    }
    #[doc = "DMAC0_OTRIG_CH23"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch23(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh23
    }
    #[doc = "DMAC0_OTRIG_CH24"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch24(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh24
    }
    #[doc = "DMAC0_OTRIG_CH25"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch25(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh25
    }
    #[doc = "DMAC0_OTRIG_CH26"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch26(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh26
    }
    #[doc = "DMAC0_OTRIG_CH27"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch27(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh27
    }
    #[doc = "DMAC0_OTRIG_CH28"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch28(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh28
    }
    #[doc = "DMAC0_OTRIG_CH29"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch29(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh29
    }
    #[doc = "DMAC0_OTRIG_CH30"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch30(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh30
    }
    #[doc = "DMAC0_OTRIG_CH31"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch31(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh31
    }
    #[doc = "DMAC0_OTRIG_CH32"]
    #[inline(always)]
    pub fn is_dmac0_otrig_ch32(&self) -> bool {
        *self == Dmac0OtrigSel::Dmac0OtrigCh32
    }
}
#[doc = "Field `DMAC0_OTRIG_SEL` writer - DMAC0 Output Triggers Select for A, B, C, D IE.,DMAC0_OTRIG_A, DMAC0_OTRIG_B, DMAC0_OTRIG_C, DMAC0_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
pub type Dmac0OtrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, Dmac0OtrigSel>;
impl<'a, REG> Dmac0OtrigSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMAC0_OTRIG_CH0"]
    #[inline(always)]
    pub fn dmac0_otrig_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh0)
    }
    #[doc = "DMAC0_OTRIG_CH1"]
    #[inline(always)]
    pub fn dmac0_otrig_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh1)
    }
    #[doc = "DMAC0_OTRIG_CH2"]
    #[inline(always)]
    pub fn dmac0_otrig_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh2)
    }
    #[doc = "DMAC0_OTRIG_CH3"]
    #[inline(always)]
    pub fn dmac0_otrig_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh3)
    }
    #[doc = "DMAC0_OTRIG_CH4"]
    #[inline(always)]
    pub fn dmac0_otrig_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh4)
    }
    #[doc = "DMAC0_OTRIG_CH5"]
    #[inline(always)]
    pub fn dmac0_otrig_ch5(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh5)
    }
    #[doc = "DMAC0_OTRIG_CH6"]
    #[inline(always)]
    pub fn dmac0_otrig_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh6)
    }
    #[doc = "DMAC0_OTRIG_CH7"]
    #[inline(always)]
    pub fn dmac0_otrig_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh7)
    }
    #[doc = "DMAC0_OTRIG_CH8"]
    #[inline(always)]
    pub fn dmac0_otrig_ch8(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh8)
    }
    #[doc = "DMAC0_OTRIG_CH9"]
    #[inline(always)]
    pub fn dmac0_otrig_ch9(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh9)
    }
    #[doc = "DMAC0_OTRIG_CH10"]
    #[inline(always)]
    pub fn dmac0_otrig_ch10(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh10)
    }
    #[doc = "DMAC0_OTRIG_CH11"]
    #[inline(always)]
    pub fn dmac0_otrig_ch11(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh11)
    }
    #[doc = "DMAC0_OTRIG_CH12"]
    #[inline(always)]
    pub fn dmac0_otrig_ch12(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh12)
    }
    #[doc = "DMAC0_OTRIG_CH13"]
    #[inline(always)]
    pub fn dmac0_otrig_ch13(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh13)
    }
    #[doc = "DMAC0_OTRIG_CH14"]
    #[inline(always)]
    pub fn dmac0_otrig_ch14(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh14)
    }
    #[doc = "DMAC0_OTRIG_CH15"]
    #[inline(always)]
    pub fn dmac0_otrig_ch15(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh15)
    }
    #[doc = "DMAC0_OTRIG_CH16"]
    #[inline(always)]
    pub fn dmac0_otrig_ch16(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh16)
    }
    #[doc = "DMAC0_OTRIG_CH17"]
    #[inline(always)]
    pub fn dmac0_otrig_ch17(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh17)
    }
    #[doc = "DMAC0_OTRIG_CH18"]
    #[inline(always)]
    pub fn dmac0_otrig_ch18(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh18)
    }
    #[doc = "DMAC0_OTRIG_CH19"]
    #[inline(always)]
    pub fn dmac0_otrig_ch19(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh19)
    }
    #[doc = "DMAC0_OTRIG_CH20"]
    #[inline(always)]
    pub fn dmac0_otrig_ch20(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh20)
    }
    #[doc = "DMAC0_OTRIG_CH21"]
    #[inline(always)]
    pub fn dmac0_otrig_ch21(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh21)
    }
    #[doc = "DMAC0_OTRIG_CH22"]
    #[inline(always)]
    pub fn dmac0_otrig_ch22(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh22)
    }
    #[doc = "DMAC0_OTRIG_CH23"]
    #[inline(always)]
    pub fn dmac0_otrig_ch23(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh23)
    }
    #[doc = "DMAC0_OTRIG_CH24"]
    #[inline(always)]
    pub fn dmac0_otrig_ch24(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh24)
    }
    #[doc = "DMAC0_OTRIG_CH25"]
    #[inline(always)]
    pub fn dmac0_otrig_ch25(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh25)
    }
    #[doc = "DMAC0_OTRIG_CH26"]
    #[inline(always)]
    pub fn dmac0_otrig_ch26(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh26)
    }
    #[doc = "DMAC0_OTRIG_CH27"]
    #[inline(always)]
    pub fn dmac0_otrig_ch27(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh27)
    }
    #[doc = "DMAC0_OTRIG_CH28"]
    #[inline(always)]
    pub fn dmac0_otrig_ch28(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh28)
    }
    #[doc = "DMAC0_OTRIG_CH29"]
    #[inline(always)]
    pub fn dmac0_otrig_ch29(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh29)
    }
    #[doc = "DMAC0_OTRIG_CH30"]
    #[inline(always)]
    pub fn dmac0_otrig_ch30(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh30)
    }
    #[doc = "DMAC0_OTRIG_CH31"]
    #[inline(always)]
    pub fn dmac0_otrig_ch31(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh31)
    }
    #[doc = "DMAC0_OTRIG_CH32"]
    #[inline(always)]
    pub fn dmac0_otrig_ch32(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac0OtrigSel::Dmac0OtrigCh32)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMAC0 Output Triggers Select for A, B, C, D IE.,DMAC0_OTRIG_A, DMAC0_OTRIG_B, DMAC0_OTRIG_C, DMAC0_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    pub fn dmac0_otrig_sel(&self) -> Dmac0OtrigSelR {
        Dmac0OtrigSelR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0_OTRIG_SEL")
            .field("dmac0_otrig_sel", &self.dmac0_otrig_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - DMAC0 Output Triggers Select for A, B, C, D IE.,DMAC0_OTRIG_A, DMAC0_OTRIG_B, DMAC0_OTRIG_C, DMAC0_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    #[must_use]
    pub fn dmac0_otrig_sel(&mut self) -> Dmac0OtrigSelW<Dmac0OtrigSelSpec> {
        Dmac0OtrigSelW::new(self, 0)
    }
}
#[doc = "DMAC0 Output Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac0_otrig_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0_otrig_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac0OtrigSelSpec;
impl crate::RegisterSpec for Dmac0OtrigSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac0_otrig_sel::R`](R) reader structure"]
impl crate::Readable for Dmac0OtrigSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac0_otrig_sel::W`](W) writer structure"]
impl crate::Writable for Dmac0OtrigSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC0_OTRIG_SEL%s to value 0x3f"]
impl crate::Resettable for Dmac0OtrigSelSpec {
    const RESET_VALUE: u32 = 0x3f;
}
