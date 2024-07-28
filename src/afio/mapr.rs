#[doc = "Register `MAPR` reader"]
pub type R = crate::R<MaprSpec>;
#[doc = "Register `MAPR` writer"]
pub type W = crate::W<MaprSpec>;
#[doc = "Field `SPI1_REMAP` reader - SPI1 remapping"]
pub type Spi1RemapR = crate::BitReader;
#[doc = "Field `SPI1_REMAP` writer - SPI1 remapping"]
pub type Spi1RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_REMAP` reader - I2C1 remapping"]
pub type I2c1RemapR = crate::BitReader;
#[doc = "Field `I2C1_REMAP` writer - I2C1 remapping"]
pub type I2c1RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_REMAP` reader - USART1 remapping"]
pub type Usart1RemapR = crate::BitReader;
#[doc = "Field `USART1_REMAP` writer - USART1 remapping"]
pub type Usart1RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2_REMAP` reader - USART2 remapping"]
pub type Usart2RemapR = crate::BitReader;
#[doc = "Field `USART2_REMAP` writer - USART2 remapping"]
pub type Usart2RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3_REMAP` reader - USART3 remapping"]
pub type Usart3RemapR = crate::FieldReader;
#[doc = "Field `USART3_REMAP` writer - USART3 remapping"]
pub type Usart3RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM1_REMAP` reader - TIM1 remapping"]
pub type Tim1RemapR = crate::FieldReader;
#[doc = "Field `TIM1_REMAP` writer - TIM1 remapping"]
pub type Tim1RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM2_REMAP` reader - TIM2 remapping"]
pub type Tim2RemapR = crate::FieldReader;
#[doc = "Field `TIM2_REMAP` writer - TIM2 remapping"]
pub type Tim2RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM3_REMAP` reader - TIM3 remapping"]
pub type Tim3RemapR = crate::FieldReader;
#[doc = "Field `TIM3_REMAP` writer - TIM3 remapping"]
pub type Tim3RemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM4_REMAP` reader - TIM4 remapping"]
pub type Tim4RemapR = crate::BitReader;
#[doc = "Field `TIM4_REMAP` writer - TIM4 remapping"]
pub type Tim4RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN_REMAP` reader - CAN1 remapping"]
pub type CanRemapR = crate::FieldReader;
#[doc = "Field `CAN_REMAP` writer - CAN1 remapping"]
pub type CanRemapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD01_REMAP` reader - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type Pd01RemapR = crate::BitReader;
#[doc = "Field `PD01_REMAP` writer - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type Pd01RemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5CH4_IREMAP` reader - Set and cleared by software"]
pub type Tim5ch4IremapR = crate::BitReader;
#[doc = "Field `TIM5CH4_IREMAP` writer - Set and cleared by software"]
pub type Tim5ch4IremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETRGINJ_REMAP` reader - ADC 1 External trigger injected conversion remapping"]
pub type Adc1EtrginjRemapR = crate::BitReader;
#[doc = "Field `ADC1_ETRGINJ_REMAP` writer - ADC 1 External trigger injected conversion remapping"]
pub type Adc1EtrginjRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETRGREG_REMAP` reader - ADC 1 external trigger regular conversion remapping"]
pub type Adc1EtrgregRemapR = crate::BitReader;
#[doc = "Field `ADC1_ETRGREG_REMAP` writer - ADC 1 external trigger regular conversion remapping"]
pub type Adc1EtrgregRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETRGINJ_REMAP` reader - ADC 2 external trigger injected conversion remapping"]
pub type Adc2EtrginjRemapR = crate::BitReader;
#[doc = "Field `ADC2_ETRGINJ_REMAP` writer - ADC 2 external trigger injected conversion remapping"]
pub type Adc2EtrginjRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETRGREG_REMAP` reader - ADC 2 external trigger regular conversion remapping"]
pub type Adc2EtrgregRemapR = crate::BitReader;
#[doc = "Field `ADC2_ETRGREG_REMAP` writer - ADC 2 external trigger regular conversion remapping"]
pub type Adc2EtrgregRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJ_CFG` writer - Serial wire JTAG configuration"]
pub type SwjCfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1_remap(&self) -> Spi1RemapR {
        Spi1RemapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1_remap(&self) -> I2c1RemapR {
        I2c1RemapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&self) -> Usart1RemapR {
        Usart1RemapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&self) -> Usart2RemapR {
        Usart2RemapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3_remap(&self) -> Usart3RemapR {
        Usart3RemapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1_remap(&self) -> Tim1RemapR {
        Tim1RemapR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2_remap(&self) -> Tim2RemapR {
        Tim2RemapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3_remap(&self) -> Tim3RemapR {
        Tim3RemapR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    pub fn tim4_remap(&self) -> Tim4RemapR {
        Tim4RemapR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    pub fn can_remap(&self) -> CanRemapR {
        CanRemapR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_remap(&self) -> Pd01RemapR {
        Pd01RemapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    pub fn tim5ch4_iremap(&self) -> Tim5ch4IremapR {
        Tim5ch4IremapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&self) -> Adc1EtrginjRemapR {
        Adc1EtrginjRemapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&self) -> Adc1EtrgregRemapR {
        Adc1EtrgregRemapR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC 2 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc2_etrginj_remap(&self) -> Adc2EtrginjRemapR {
        Adc2EtrginjRemapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc2_etrgreg_remap(&self) -> Adc2EtrgregRemapR {
        Adc2EtrgregRemapR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_remap(&mut self) -> Spi1RemapW<MaprSpec> {
        Spi1RemapW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_remap(&mut self) -> I2c1RemapW<MaprSpec> {
        I2c1RemapW::new(self, 1)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_remap(&mut self) -> Usart1RemapW<MaprSpec> {
        Usart1RemapW::new(self, 2)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart2_remap(&mut self) -> Usart2RemapW<MaprSpec> {
        Usart2RemapW::new(self, 3)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_remap(&mut self) -> Usart3RemapW<MaprSpec> {
        Usart3RemapW::new(self, 4)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_remap(&mut self) -> Tim1RemapW<MaprSpec> {
        Tim1RemapW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim2_remap(&mut self) -> Tim2RemapW<MaprSpec> {
        Tim2RemapW::new(self, 8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim3_remap(&mut self) -> Tim3RemapW<MaprSpec> {
        Tim3RemapW::new(self, 10)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim4_remap(&mut self) -> Tim4RemapW<MaprSpec> {
        Tim4RemapW::new(self, 12)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can_remap(&mut self) -> CanRemapW<MaprSpec> {
        CanRemapW::new(self, 13)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_remap(&mut self) -> Pd01RemapW<MaprSpec> {
        Pd01RemapW::new(self, 15)
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn tim5ch4_iremap(&mut self) -> Tim5ch4IremapW<MaprSpec> {
        Tim5ch4IremapW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrginj_remap(&mut self) -> Adc1EtrginjRemapW<MaprSpec> {
        Adc1EtrginjRemapW::new(self, 17)
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrgreg_remap(&mut self) -> Adc1EtrgregRemapW<MaprSpec> {
        Adc1EtrgregRemapW::new(self, 18)
    }
    #[doc = "Bit 19 - ADC 2 external trigger injected conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etrginj_remap(&mut self) -> Adc2EtrginjRemapW<MaprSpec> {
        Adc2EtrginjRemapW::new(self, 19)
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etrgreg_remap(&mut self) -> Adc2EtrgregRemapW<MaprSpec> {
        Adc2EtrgregRemapW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swj_cfg(&mut self) -> SwjCfgW<MaprSpec> {
        SwjCfgW::new(self, 24)
    }
}
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)\n\nYou can [`read`](crate::Reg::read) this register and get [`mapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaprSpec;
impl crate::RegisterSpec for MaprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mapr::R`](R) reader structure"]
impl crate::Readable for MaprSpec {}
#[doc = "`write(|w| ..)` method takes [`mapr::W`](W) writer structure"]
impl crate::Writable for MaprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAPR to value 0"]
impl crate::Resettable for MaprSpec {
    const RESET_VALUE: u32 = 0;
}
