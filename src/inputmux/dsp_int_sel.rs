#[doc = "Register `DSP_INT_SEL[%s]` reader"]
pub type R = crate::R<DspIntSelSpec>;
#[doc = "Register `DSP_INT_SEL[%s]` writer"]
pub type W = crate::W<DspIntSelSpec>;
#[doc = "DSP Input(n) Selection. 34:1 Selection for each. . .\n\nValue on reset: 63"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DspIntSel {
    #[doc = "0: FLEXCOMM0"]
    Flexcomm0 = 0,
    #[doc = "1: FLEXCOMM1"]
    Flexcomm1 = 1,
    #[doc = "2: FLEXCOMM2"]
    Flexcomm2 = 2,
    #[doc = "3: FLEXCOMM3"]
    Flexcomm3 = 3,
    #[doc = "4: FLEXCOMM4"]
    Flexcomm4 = 4,
    #[doc = "5: FLEXCOMM5"]
    Flexcomm5 = 5,
    #[doc = "6: FLEXCOMM6"]
    Flexcomm6 = 6,
    #[doc = "7: FLEXCOMM7"]
    Flexcomm7 = 7,
    #[doc = "8: GPIO_INT0_IRQ0"]
    GpioInt0Irq0 = 8,
    #[doc = "9: GPIO_INT0_IRQ1"]
    GpioInt0Irq1 = 9,
    #[doc = "10: GPIO_INT0_IRQ2"]
    GpioInt0Irq2 = 10,
    #[doc = "11: GPIO_INT0_IRQ3"]
    GpioInt0Irq3 = 11,
    #[doc = "12: GPIO_INT0_IRQ4"]
    GpioInt0Irq4 = 12,
    #[doc = "13: GPIO_INT0_IRQ5"]
    GpioInt0Irq5 = 13,
    #[doc = "14: GPIO_INT0_IRQ6"]
    GpioInt0Irq6 = 14,
    #[doc = "15: GPIO_INT0_IRQ7"]
    GpioInt0Irq7 = 15,
    #[doc = "16: NSHSGPIO_INT0"]
    NshsgpioInt0 = 16,
    #[doc = "17: NSHSGPIO_INT1"]
    NshsgpioInt1 = 17,
    #[doc = "18: WDT1"]
    Wdt1 = 18,
    #[doc = "19: DMAC0"]
    Dmac0 = 19,
    #[doc = "20: DMAC1"]
    Dmac1 = 20,
    #[doc = "21: MU"]
    Mu = 21,
    #[doc = "22: UTICK0"]
    Utick0 = 22,
    #[doc = "23: MRT0"]
    Mrt0 = 23,
    #[doc = "24: OS_EVENT_TIMER or OS_EVENT_WAKEUP"]
    OsEventTimerOrOsEventWakeup = 24,
    #[doc = "25: CT32BIT0"]
    Ct32bit0 = 25,
    #[doc = "26: CT32BIT1"]
    Ct32bit1 = 26,
    #[doc = "27: CT32BIT2"]
    Ct32bit2 = 27,
    #[doc = "28: CT32BIT3"]
    Ct32bit3 = 28,
    #[doc = "29: CT32BIT4"]
    Ct32bit4 = 29,
    #[doc = "30: RTC_LITE0_ALARM or RTC_LITE0_WAKEUP"]
    RtcLite0AlarmOrRtcLite0Wakeup = 30,
    #[doc = "31: I3C0"]
    I3c0 = 31,
    #[doc = "32: DMIC0"]
    Dmic0 = 32,
    #[doc = "33: HWVAD0"]
    Hwvad0 = 33,
    #[doc = "34: FLEXSPI"]
    Flexspi = 34,
}
impl From<DspIntSel> for u8 {
    #[inline(always)]
    fn from(variant: DspIntSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DspIntSel {
    type Ux = u8;
}
impl crate::IsEnum for DspIntSel {}
#[doc = "Field `DSP_INT_SEL` reader - DSP Input(n) Selection. 34:1 Selection for each. . ."]
pub type DspIntSelR = crate::FieldReader<DspIntSel>;
impl DspIntSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DspIntSel> {
        match self.bits {
            0 => Some(DspIntSel::Flexcomm0),
            1 => Some(DspIntSel::Flexcomm1),
            2 => Some(DspIntSel::Flexcomm2),
            3 => Some(DspIntSel::Flexcomm3),
            4 => Some(DspIntSel::Flexcomm4),
            5 => Some(DspIntSel::Flexcomm5),
            6 => Some(DspIntSel::Flexcomm6),
            7 => Some(DspIntSel::Flexcomm7),
            8 => Some(DspIntSel::GpioInt0Irq0),
            9 => Some(DspIntSel::GpioInt0Irq1),
            10 => Some(DspIntSel::GpioInt0Irq2),
            11 => Some(DspIntSel::GpioInt0Irq3),
            12 => Some(DspIntSel::GpioInt0Irq4),
            13 => Some(DspIntSel::GpioInt0Irq5),
            14 => Some(DspIntSel::GpioInt0Irq6),
            15 => Some(DspIntSel::GpioInt0Irq7),
            16 => Some(DspIntSel::NshsgpioInt0),
            17 => Some(DspIntSel::NshsgpioInt1),
            18 => Some(DspIntSel::Wdt1),
            19 => Some(DspIntSel::Dmac0),
            20 => Some(DspIntSel::Dmac1),
            21 => Some(DspIntSel::Mu),
            22 => Some(DspIntSel::Utick0),
            23 => Some(DspIntSel::Mrt0),
            24 => Some(DspIntSel::OsEventTimerOrOsEventWakeup),
            25 => Some(DspIntSel::Ct32bit0),
            26 => Some(DspIntSel::Ct32bit1),
            27 => Some(DspIntSel::Ct32bit2),
            28 => Some(DspIntSel::Ct32bit3),
            29 => Some(DspIntSel::Ct32bit4),
            30 => Some(DspIntSel::RtcLite0AlarmOrRtcLite0Wakeup),
            31 => Some(DspIntSel::I3c0),
            32 => Some(DspIntSel::Dmic0),
            33 => Some(DspIntSel::Hwvad0),
            34 => Some(DspIntSel::Flexspi),
            _ => None,
        }
    }
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == DspIntSel::Flexcomm0
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == DspIntSel::Flexcomm1
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == DspIntSel::Flexcomm2
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == DspIntSel::Flexcomm3
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == DspIntSel::Flexcomm4
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == DspIntSel::Flexcomm5
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == DspIntSel::Flexcomm6
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == DspIntSel::Flexcomm7
    }
    #[doc = "GPIO_INT0_IRQ0"]
    #[inline(always)]
    pub fn is_gpio_int0_irq0(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq0
    }
    #[doc = "GPIO_INT0_IRQ1"]
    #[inline(always)]
    pub fn is_gpio_int0_irq1(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq1
    }
    #[doc = "GPIO_INT0_IRQ2"]
    #[inline(always)]
    pub fn is_gpio_int0_irq2(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq2
    }
    #[doc = "GPIO_INT0_IRQ3"]
    #[inline(always)]
    pub fn is_gpio_int0_irq3(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq3
    }
    #[doc = "GPIO_INT0_IRQ4"]
    #[inline(always)]
    pub fn is_gpio_int0_irq4(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq4
    }
    #[doc = "GPIO_INT0_IRQ5"]
    #[inline(always)]
    pub fn is_gpio_int0_irq5(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq5
    }
    #[doc = "GPIO_INT0_IRQ6"]
    #[inline(always)]
    pub fn is_gpio_int0_irq6(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq6
    }
    #[doc = "GPIO_INT0_IRQ7"]
    #[inline(always)]
    pub fn is_gpio_int0_irq7(&self) -> bool {
        *self == DspIntSel::GpioInt0Irq7
    }
    #[doc = "NSHSGPIO_INT0"]
    #[inline(always)]
    pub fn is_nshsgpio_int0(&self) -> bool {
        *self == DspIntSel::NshsgpioInt0
    }
    #[doc = "NSHSGPIO_INT1"]
    #[inline(always)]
    pub fn is_nshsgpio_int1(&self) -> bool {
        *self == DspIntSel::NshsgpioInt1
    }
    #[doc = "WDT1"]
    #[inline(always)]
    pub fn is_wdt1(&self) -> bool {
        *self == DspIntSel::Wdt1
    }
    #[doc = "DMAC0"]
    #[inline(always)]
    pub fn is_dmac0(&self) -> bool {
        *self == DspIntSel::Dmac0
    }
    #[doc = "DMAC1"]
    #[inline(always)]
    pub fn is_dmac1(&self) -> bool {
        *self == DspIntSel::Dmac1
    }
    #[doc = "MU"]
    #[inline(always)]
    pub fn is_mu(&self) -> bool {
        *self == DspIntSel::Mu
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub fn is_utick0(&self) -> bool {
        *self == DspIntSel::Utick0
    }
    #[doc = "MRT0"]
    #[inline(always)]
    pub fn is_mrt0(&self) -> bool {
        *self == DspIntSel::Mrt0
    }
    #[doc = "OS_EVENT_TIMER or OS_EVENT_WAKEUP"]
    #[inline(always)]
    pub fn is_os_event_timer_or_os_event_wakeup(&self) -> bool {
        *self == DspIntSel::OsEventTimerOrOsEventWakeup
    }
    #[doc = "CT32BIT0"]
    #[inline(always)]
    pub fn is_ct32bit0(&self) -> bool {
        *self == DspIntSel::Ct32bit0
    }
    #[doc = "CT32BIT1"]
    #[inline(always)]
    pub fn is_ct32bit1(&self) -> bool {
        *self == DspIntSel::Ct32bit1
    }
    #[doc = "CT32BIT2"]
    #[inline(always)]
    pub fn is_ct32bit2(&self) -> bool {
        *self == DspIntSel::Ct32bit2
    }
    #[doc = "CT32BIT3"]
    #[inline(always)]
    pub fn is_ct32bit3(&self) -> bool {
        *self == DspIntSel::Ct32bit3
    }
    #[doc = "CT32BIT4"]
    #[inline(always)]
    pub fn is_ct32bit4(&self) -> bool {
        *self == DspIntSel::Ct32bit4
    }
    #[doc = "RTC_LITE0_ALARM or RTC_LITE0_WAKEUP"]
    #[inline(always)]
    pub fn is_rtc_lite0_alarm_or_rtc_lite0_wakeup(&self) -> bool {
        *self == DspIntSel::RtcLite0AlarmOrRtcLite0Wakeup
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub fn is_i3c0(&self) -> bool {
        *self == DspIntSel::I3c0
    }
    #[doc = "DMIC0"]
    #[inline(always)]
    pub fn is_dmic0(&self) -> bool {
        *self == DspIntSel::Dmic0
    }
    #[doc = "HWVAD0"]
    #[inline(always)]
    pub fn is_hwvad0(&self) -> bool {
        *self == DspIntSel::Hwvad0
    }
    #[doc = "FLEXSPI"]
    #[inline(always)]
    pub fn is_flexspi(&self) -> bool {
        *self == DspIntSel::Flexspi
    }
}
#[doc = "Field `DSP_INT_SEL` writer - DSP Input(n) Selection. 34:1 Selection for each. . ."]
pub type DspIntSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, DspIntSel>;
impl<'a, REG> DspIntSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLEXCOMM0"]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm0)
    }
    #[doc = "FLEXCOMM1"]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm1)
    }
    #[doc = "FLEXCOMM2"]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm2)
    }
    #[doc = "FLEXCOMM3"]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm3)
    }
    #[doc = "FLEXCOMM4"]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm4)
    }
    #[doc = "FLEXCOMM5"]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm5)
    }
    #[doc = "FLEXCOMM6"]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm6)
    }
    #[doc = "FLEXCOMM7"]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexcomm7)
    }
    #[doc = "GPIO_INT0_IRQ0"]
    #[inline(always)]
    pub fn gpio_int0_irq0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq0)
    }
    #[doc = "GPIO_INT0_IRQ1"]
    #[inline(always)]
    pub fn gpio_int0_irq1(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq1)
    }
    #[doc = "GPIO_INT0_IRQ2"]
    #[inline(always)]
    pub fn gpio_int0_irq2(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq2)
    }
    #[doc = "GPIO_INT0_IRQ3"]
    #[inline(always)]
    pub fn gpio_int0_irq3(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq3)
    }
    #[doc = "GPIO_INT0_IRQ4"]
    #[inline(always)]
    pub fn gpio_int0_irq4(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq4)
    }
    #[doc = "GPIO_INT0_IRQ5"]
    #[inline(always)]
    pub fn gpio_int0_irq5(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq5)
    }
    #[doc = "GPIO_INT0_IRQ6"]
    #[inline(always)]
    pub fn gpio_int0_irq6(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq6)
    }
    #[doc = "GPIO_INT0_IRQ7"]
    #[inline(always)]
    pub fn gpio_int0_irq7(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::GpioInt0Irq7)
    }
    #[doc = "NSHSGPIO_INT0"]
    #[inline(always)]
    pub fn nshsgpio_int0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::NshsgpioInt0)
    }
    #[doc = "NSHSGPIO_INT1"]
    #[inline(always)]
    pub fn nshsgpio_int1(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::NshsgpioInt1)
    }
    #[doc = "WDT1"]
    #[inline(always)]
    pub fn wdt1(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Wdt1)
    }
    #[doc = "DMAC0"]
    #[inline(always)]
    pub fn dmac0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Dmac0)
    }
    #[doc = "DMAC1"]
    #[inline(always)]
    pub fn dmac1(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Dmac1)
    }
    #[doc = "MU"]
    #[inline(always)]
    pub fn mu(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Mu)
    }
    #[doc = "UTICK0"]
    #[inline(always)]
    pub fn utick0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Utick0)
    }
    #[doc = "MRT0"]
    #[inline(always)]
    pub fn mrt0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Mrt0)
    }
    #[doc = "OS_EVENT_TIMER or OS_EVENT_WAKEUP"]
    #[inline(always)]
    pub fn os_event_timer_or_os_event_wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::OsEventTimerOrOsEventWakeup)
    }
    #[doc = "CT32BIT0"]
    #[inline(always)]
    pub fn ct32bit0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Ct32bit0)
    }
    #[doc = "CT32BIT1"]
    #[inline(always)]
    pub fn ct32bit1(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Ct32bit1)
    }
    #[doc = "CT32BIT2"]
    #[inline(always)]
    pub fn ct32bit2(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Ct32bit2)
    }
    #[doc = "CT32BIT3"]
    #[inline(always)]
    pub fn ct32bit3(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Ct32bit3)
    }
    #[doc = "CT32BIT4"]
    #[inline(always)]
    pub fn ct32bit4(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Ct32bit4)
    }
    #[doc = "RTC_LITE0_ALARM or RTC_LITE0_WAKEUP"]
    #[inline(always)]
    pub fn rtc_lite0_alarm_or_rtc_lite0_wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::RtcLite0AlarmOrRtcLite0Wakeup)
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub fn i3c0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::I3c0)
    }
    #[doc = "DMIC0"]
    #[inline(always)]
    pub fn dmic0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Dmic0)
    }
    #[doc = "HWVAD0"]
    #[inline(always)]
    pub fn hwvad0(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Hwvad0)
    }
    #[doc = "FLEXSPI"]
    #[inline(always)]
    pub fn flexspi(self) -> &'a mut crate::W<REG> {
        self.variant(DspIntSel::Flexspi)
    }
}
impl R {
    #[doc = "Bits 0:5 - DSP Input(n) Selection. 34:1 Selection for each. . ."]
    #[inline(always)]
    pub fn dsp_int_sel(&self) -> DspIntSelR {
        DspIntSelR::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSP_INT_SEL")
            .field("dsp_int_sel", &self.dsp_int_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - DSP Input(n) Selection. 34:1 Selection for each. . ."]
    #[inline(always)]
    pub fn dsp_int_sel(&mut self) -> DspIntSelW<DspIntSelSpec> {
        DspIntSelW::new(self, 0)
    }
}
#[doc = "DSP Interrupt Input Multiplexers N\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp_int_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp_int_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspIntSelSpec;
impl crate::RegisterSpec for DspIntSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_int_sel::R`](R) reader structure"]
impl crate::Readable for DspIntSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp_int_sel::W`](W) writer structure"]
impl crate::Writable for DspIntSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP_INT_SEL[%s]
to value 0x3f"]
impl crate::Resettable for DspIntSelSpec {
    const RESET_VALUE: u32 = 0x3f;
}
