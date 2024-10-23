#[doc = "Register `DMAC1_OTRIG_SEL%s` reader"]
pub type R = crate::R<Dmac1OtrigSelSpec>;
#[doc = "Register `DMAC1_OTRIG_SEL%s` writer"]
pub type W = crate::W<Dmac1OtrigSelSpec>;
#[doc = "DMA1 Output Triggers Select for A, B, C, D IE., DMA1_OTRIG_A, DMA1_OTRIG_B, DM1_OTRIG_C, DMA1_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . .\n\nValue on reset: 63"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmac1OtrigSel {
    #[doc = "0: DMAC1_OTRIG_CH0"]
    Dmac1OtrigCh0 = 0,
    #[doc = "1: DMAC1_OTRIG_CH1"]
    Dmac1OtrigCh1 = 1,
    #[doc = "2: DMAC1_OTRIG_CH2"]
    Dmac1OtrigCh2 = 2,
    #[doc = "3: DMAC1_OTRIG_CH3"]
    Dmac1OtrigCh3 = 3,
    #[doc = "4: DMAC1_OTRIG_CH4"]
    Dmac1OtrigCh4 = 4,
    #[doc = "5: DMAC1_OTRIG_CH5"]
    Dmac1OtrigCh5 = 5,
    #[doc = "6: DMAC1_OTRIG_CH6"]
    Dmac1OtrigCh6 = 6,
    #[doc = "7: DMAC1_OTRIG_CH7"]
    Dmac1OtrigCh7 = 7,
    #[doc = "8: DMAC1_OTRIG_CH8"]
    Dmac1OtrigCh8 = 8,
    #[doc = "9: DMAC1_OTRIG_CH9"]
    Dmac1OtrigCh9 = 9,
    #[doc = "10: DMAC1_OTRIG_CH10"]
    Dmac1OtrigCh10 = 10,
    #[doc = "11: DMAC1_OTRIG_CH11"]
    Dmac1OtrigCh11 = 11,
    #[doc = "12: DMAC1_OTRIG_CH12"]
    Dmac1OtrigCh12 = 12,
    #[doc = "13: DMAC1_OTRIG_CH13"]
    Dmac1OtrigCh13 = 13,
    #[doc = "14: DMAC1_OTRIG_CH14"]
    Dmac1OtrigCh14 = 14,
    #[doc = "15: DMAC1_OTRIG_CH15"]
    Dmac1OtrigCh15 = 15,
    #[doc = "16: DMAC1_OTRIG_CH16"]
    Dmac1OtrigCh16 = 16,
    #[doc = "17: DMAC1_OTRIG_CH17"]
    Dmac1OtrigCh17 = 17,
    #[doc = "18: DMAC1_OTRIG_CH18"]
    Dmac1OtrigCh18 = 18,
    #[doc = "19: DMAC1_OTRIG_CH19"]
    Dmac1OtrigCh19 = 19,
    #[doc = "20: DMAC1_OTRIG_CH20"]
    Dmac1OtrigCh20 = 20,
    #[doc = "21: DMAC1_OTRIG_CH21"]
    Dmac1OtrigCh21 = 21,
    #[doc = "22: DMAC1_OTRIG_CH22"]
    Dmac1OtrigCh22 = 22,
    #[doc = "23: DMAC1_OTRIG_CH23"]
    Dmac1OtrigCh23 = 23,
    #[doc = "24: DMAC1_OTRIG_CH24"]
    Dmac1OtrigCh24 = 24,
    #[doc = "25: DMAC1_OTRIG_CH25"]
    Dmac1OtrigCh25 = 25,
    #[doc = "26: DMAC1_OTRIG_CH26"]
    Dmac1OtrigCh26 = 26,
    #[doc = "27: DMAC1_OTRIG_CH27"]
    Dmac1OtrigCh27 = 27,
    #[doc = "28: DMAC1_OTRIG_CH28"]
    Dmac1OtrigCh28 = 28,
    #[doc = "29: DMAC1_OTRIG_CH29"]
    Dmac1OtrigCh29 = 29,
    #[doc = "30: DMAC1_OTRIG_CH30"]
    Dmac1OtrigCh30 = 30,
    #[doc = "31: DMAC1_OTRIG_CH31"]
    Dmac1OtrigCh31 = 31,
    #[doc = "32: DMAC1_OTRIG_CH32"]
    Dmac1OtrigCh32 = 32,
}
impl From<Dmac1OtrigSel> for u8 {
    #[inline(always)]
    fn from(variant: Dmac1OtrigSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmac1OtrigSel {
    type Ux = u8;
}
impl crate::IsEnum for Dmac1OtrigSel {}
#[doc = "Field `DMAC1_OTRIG_SEL` reader - DMA1 Output Triggers Select for A, B, C, D IE., DMA1_OTRIG_A, DMA1_OTRIG_B, DM1_OTRIG_C, DMA1_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
pub type Dmac1OtrigSelR = crate::FieldReader<Dmac1OtrigSel>;
impl Dmac1OtrigSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmac1OtrigSel> {
        match self.bits {
            0 => Some(Dmac1OtrigSel::Dmac1OtrigCh0),
            1 => Some(Dmac1OtrigSel::Dmac1OtrigCh1),
            2 => Some(Dmac1OtrigSel::Dmac1OtrigCh2),
            3 => Some(Dmac1OtrigSel::Dmac1OtrigCh3),
            4 => Some(Dmac1OtrigSel::Dmac1OtrigCh4),
            5 => Some(Dmac1OtrigSel::Dmac1OtrigCh5),
            6 => Some(Dmac1OtrigSel::Dmac1OtrigCh6),
            7 => Some(Dmac1OtrigSel::Dmac1OtrigCh7),
            8 => Some(Dmac1OtrigSel::Dmac1OtrigCh8),
            9 => Some(Dmac1OtrigSel::Dmac1OtrigCh9),
            10 => Some(Dmac1OtrigSel::Dmac1OtrigCh10),
            11 => Some(Dmac1OtrigSel::Dmac1OtrigCh11),
            12 => Some(Dmac1OtrigSel::Dmac1OtrigCh12),
            13 => Some(Dmac1OtrigSel::Dmac1OtrigCh13),
            14 => Some(Dmac1OtrigSel::Dmac1OtrigCh14),
            15 => Some(Dmac1OtrigSel::Dmac1OtrigCh15),
            16 => Some(Dmac1OtrigSel::Dmac1OtrigCh16),
            17 => Some(Dmac1OtrigSel::Dmac1OtrigCh17),
            18 => Some(Dmac1OtrigSel::Dmac1OtrigCh18),
            19 => Some(Dmac1OtrigSel::Dmac1OtrigCh19),
            20 => Some(Dmac1OtrigSel::Dmac1OtrigCh20),
            21 => Some(Dmac1OtrigSel::Dmac1OtrigCh21),
            22 => Some(Dmac1OtrigSel::Dmac1OtrigCh22),
            23 => Some(Dmac1OtrigSel::Dmac1OtrigCh23),
            24 => Some(Dmac1OtrigSel::Dmac1OtrigCh24),
            25 => Some(Dmac1OtrigSel::Dmac1OtrigCh25),
            26 => Some(Dmac1OtrigSel::Dmac1OtrigCh26),
            27 => Some(Dmac1OtrigSel::Dmac1OtrigCh27),
            28 => Some(Dmac1OtrigSel::Dmac1OtrigCh28),
            29 => Some(Dmac1OtrigSel::Dmac1OtrigCh29),
            30 => Some(Dmac1OtrigSel::Dmac1OtrigCh30),
            31 => Some(Dmac1OtrigSel::Dmac1OtrigCh31),
            32 => Some(Dmac1OtrigSel::Dmac1OtrigCh32),
            _ => None,
        }
    }
    #[doc = "DMAC1_OTRIG_CH0"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch0(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh0
    }
    #[doc = "DMAC1_OTRIG_CH1"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch1(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh1
    }
    #[doc = "DMAC1_OTRIG_CH2"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch2(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh2
    }
    #[doc = "DMAC1_OTRIG_CH3"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch3(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh3
    }
    #[doc = "DMAC1_OTRIG_CH4"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch4(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh4
    }
    #[doc = "DMAC1_OTRIG_CH5"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch5(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh5
    }
    #[doc = "DMAC1_OTRIG_CH6"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch6(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh6
    }
    #[doc = "DMAC1_OTRIG_CH7"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch7(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh7
    }
    #[doc = "DMAC1_OTRIG_CH8"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch8(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh8
    }
    #[doc = "DMAC1_OTRIG_CH9"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch9(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh9
    }
    #[doc = "DMAC1_OTRIG_CH10"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch10(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh10
    }
    #[doc = "DMAC1_OTRIG_CH11"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch11(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh11
    }
    #[doc = "DMAC1_OTRIG_CH12"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch12(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh12
    }
    #[doc = "DMAC1_OTRIG_CH13"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch13(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh13
    }
    #[doc = "DMAC1_OTRIG_CH14"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch14(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh14
    }
    #[doc = "DMAC1_OTRIG_CH15"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch15(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh15
    }
    #[doc = "DMAC1_OTRIG_CH16"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch16(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh16
    }
    #[doc = "DMAC1_OTRIG_CH17"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch17(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh17
    }
    #[doc = "DMAC1_OTRIG_CH18"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch18(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh18
    }
    #[doc = "DMAC1_OTRIG_CH19"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch19(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh19
    }
    #[doc = "DMAC1_OTRIG_CH20"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch20(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh20
    }
    #[doc = "DMAC1_OTRIG_CH21"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch21(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh21
    }
    #[doc = "DMAC1_OTRIG_CH22"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch22(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh22
    }
    #[doc = "DMAC1_OTRIG_CH23"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch23(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh23
    }
    #[doc = "DMAC1_OTRIG_CH24"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch24(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh24
    }
    #[doc = "DMAC1_OTRIG_CH25"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch25(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh25
    }
    #[doc = "DMAC1_OTRIG_CH26"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch26(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh26
    }
    #[doc = "DMAC1_OTRIG_CH27"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch27(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh27
    }
    #[doc = "DMAC1_OTRIG_CH28"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch28(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh28
    }
    #[doc = "DMAC1_OTRIG_CH29"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch29(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh29
    }
    #[doc = "DMAC1_OTRIG_CH30"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch30(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh30
    }
    #[doc = "DMAC1_OTRIG_CH31"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch31(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh31
    }
    #[doc = "DMAC1_OTRIG_CH32"]
    #[inline(always)]
    pub fn is_dmac1_otrig_ch32(&self) -> bool {
        *self == Dmac1OtrigSel::Dmac1OtrigCh32
    }
}
#[doc = "Field `DMAC1_OTRIG_SEL` writer - DMA1 Output Triggers Select for A, B, C, D IE., DMA1_OTRIG_A, DMA1_OTRIG_B, DM1_OTRIG_C, DMA1_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
pub type Dmac1OtrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, Dmac1OtrigSel>;
impl<'a, REG> Dmac1OtrigSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMAC1_OTRIG_CH0"]
    #[inline(always)]
    pub fn dmac1_otrig_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh0)
    }
    #[doc = "DMAC1_OTRIG_CH1"]
    #[inline(always)]
    pub fn dmac1_otrig_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh1)
    }
    #[doc = "DMAC1_OTRIG_CH2"]
    #[inline(always)]
    pub fn dmac1_otrig_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh2)
    }
    #[doc = "DMAC1_OTRIG_CH3"]
    #[inline(always)]
    pub fn dmac1_otrig_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh3)
    }
    #[doc = "DMAC1_OTRIG_CH4"]
    #[inline(always)]
    pub fn dmac1_otrig_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh4)
    }
    #[doc = "DMAC1_OTRIG_CH5"]
    #[inline(always)]
    pub fn dmac1_otrig_ch5(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh5)
    }
    #[doc = "DMAC1_OTRIG_CH6"]
    #[inline(always)]
    pub fn dmac1_otrig_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh6)
    }
    #[doc = "DMAC1_OTRIG_CH7"]
    #[inline(always)]
    pub fn dmac1_otrig_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh7)
    }
    #[doc = "DMAC1_OTRIG_CH8"]
    #[inline(always)]
    pub fn dmac1_otrig_ch8(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh8)
    }
    #[doc = "DMAC1_OTRIG_CH9"]
    #[inline(always)]
    pub fn dmac1_otrig_ch9(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh9)
    }
    #[doc = "DMAC1_OTRIG_CH10"]
    #[inline(always)]
    pub fn dmac1_otrig_ch10(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh10)
    }
    #[doc = "DMAC1_OTRIG_CH11"]
    #[inline(always)]
    pub fn dmac1_otrig_ch11(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh11)
    }
    #[doc = "DMAC1_OTRIG_CH12"]
    #[inline(always)]
    pub fn dmac1_otrig_ch12(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh12)
    }
    #[doc = "DMAC1_OTRIG_CH13"]
    #[inline(always)]
    pub fn dmac1_otrig_ch13(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh13)
    }
    #[doc = "DMAC1_OTRIG_CH14"]
    #[inline(always)]
    pub fn dmac1_otrig_ch14(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh14)
    }
    #[doc = "DMAC1_OTRIG_CH15"]
    #[inline(always)]
    pub fn dmac1_otrig_ch15(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh15)
    }
    #[doc = "DMAC1_OTRIG_CH16"]
    #[inline(always)]
    pub fn dmac1_otrig_ch16(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh16)
    }
    #[doc = "DMAC1_OTRIG_CH17"]
    #[inline(always)]
    pub fn dmac1_otrig_ch17(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh17)
    }
    #[doc = "DMAC1_OTRIG_CH18"]
    #[inline(always)]
    pub fn dmac1_otrig_ch18(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh18)
    }
    #[doc = "DMAC1_OTRIG_CH19"]
    #[inline(always)]
    pub fn dmac1_otrig_ch19(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh19)
    }
    #[doc = "DMAC1_OTRIG_CH20"]
    #[inline(always)]
    pub fn dmac1_otrig_ch20(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh20)
    }
    #[doc = "DMAC1_OTRIG_CH21"]
    #[inline(always)]
    pub fn dmac1_otrig_ch21(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh21)
    }
    #[doc = "DMAC1_OTRIG_CH22"]
    #[inline(always)]
    pub fn dmac1_otrig_ch22(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh22)
    }
    #[doc = "DMAC1_OTRIG_CH23"]
    #[inline(always)]
    pub fn dmac1_otrig_ch23(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh23)
    }
    #[doc = "DMAC1_OTRIG_CH24"]
    #[inline(always)]
    pub fn dmac1_otrig_ch24(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh24)
    }
    #[doc = "DMAC1_OTRIG_CH25"]
    #[inline(always)]
    pub fn dmac1_otrig_ch25(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh25)
    }
    #[doc = "DMAC1_OTRIG_CH26"]
    #[inline(always)]
    pub fn dmac1_otrig_ch26(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh26)
    }
    #[doc = "DMAC1_OTRIG_CH27"]
    #[inline(always)]
    pub fn dmac1_otrig_ch27(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh27)
    }
    #[doc = "DMAC1_OTRIG_CH28"]
    #[inline(always)]
    pub fn dmac1_otrig_ch28(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh28)
    }
    #[doc = "DMAC1_OTRIG_CH29"]
    #[inline(always)]
    pub fn dmac1_otrig_ch29(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh29)
    }
    #[doc = "DMAC1_OTRIG_CH30"]
    #[inline(always)]
    pub fn dmac1_otrig_ch30(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh30)
    }
    #[doc = "DMAC1_OTRIG_CH31"]
    #[inline(always)]
    pub fn dmac1_otrig_ch31(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh31)
    }
    #[doc = "DMAC1_OTRIG_CH32"]
    #[inline(always)]
    pub fn dmac1_otrig_ch32(self) -> &'a mut crate::W<REG> {
        self.variant(Dmac1OtrigSel::Dmac1OtrigCh32)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA1 Output Triggers Select for A, B, C, D IE., DMA1_OTRIG_A, DMA1_OTRIG_B, DM1_OTRIG_C, DMA1_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    pub fn dmac1_otrig_sel(&self) -> Dmac1OtrigSelR {
        Dmac1OtrigSelR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1_OTRIG_SEL")
            .field("dmac1_otrig_sel", &self.dmac1_otrig_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA1 Output Triggers Select for A, B, C, D IE., DMA1_OTRIG_A, DMA1_OTRIG_B, DM1_OTRIG_C, DMA1_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    #[must_use]
    pub fn dmac1_otrig_sel(&mut self) -> Dmac1OtrigSelW<Dmac1OtrigSelSpec> {
        Dmac1OtrigSelW::new(self, 0)
    }
}
#[doc = "DMAC1 Output Trigger Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac1_otrig_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_otrig_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac1OtrigSelSpec;
impl crate::RegisterSpec for Dmac1OtrigSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac1_otrig_sel::R`](R) reader structure"]
impl crate::Readable for Dmac1OtrigSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac1_otrig_sel::W`](W) writer structure"]
impl crate::Writable for Dmac1OtrigSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC1_OTRIG_SEL%s to value 0x3f"]
impl crate::Resettable for Dmac1OtrigSelSpec {
    const RESET_VALUE: u32 = 0x3f;
}
