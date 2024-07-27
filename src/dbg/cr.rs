#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DBG_SLEEP` reader - DBG_SLEEP"]
pub type DbgSleepR = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - DBG_SLEEP"]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - DBG_STOP"]
pub type DbgStopR = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - DBG_STOP"]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - DBG_STANDBY"]
pub type DbgStandbyR = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - DBG_STANDBY"]
pub type DbgStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TraceIoenR = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TraceIoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TraceModeR = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TraceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBG_IWDG_STOP` reader - DBG_IWDG_STOP"]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - DBG_IWDG_STOP"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP"]
pub type DbgWwdgStopR = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM1_STOP` reader - DBG_TIM1_STOP"]
pub type DbgTim1StopR = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - DBG_TIM1_STOP"]
pub type DbgTim1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP"]
pub type DbgTim2StopR = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP"]
pub type DbgTim2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - DBG_TIM3_STOP"]
pub type DbgTim3StopR = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - DBG_TIM3_STOP"]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP"]
pub type DbgTim4StopR = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP"]
pub type DbgTim4StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CAN1_STOP` reader - DBG_CAN1_STOP"]
pub type DbgCan1StopR = crate::BitReader;
#[doc = "Field `DBG_CAN1_STOP` writer - DBG_CAN1_STOP"]
pub type DbgCan1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - DBG_I2C1_SMBUS_TIMEOUT"]
pub type DbgI2c1SmbusTimeoutR = crate::BitReader;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - DBG_I2C1_SMBUS_TIMEOUT"]
pub type DbgI2c1SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - DBG_I2C2_SMBUS_TIMEOUT"]
pub type DbgI2c2SmbusTimeoutR = crate::BitReader;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - DBG_I2C2_SMBUS_TIMEOUT"]
pub type DbgI2c2SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8_STOP` reader - DBG_TIM8_STOP"]
pub type DbgTim8StopR = crate::BitReader;
#[doc = "Field `DBG_TIM8_STOP` writer - DBG_TIM8_STOP"]
pub type DbgTim8StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP"]
pub type DbgTim5StopR = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP"]
pub type DbgTim5StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM6_STOP` reader - DBG_TIM6_STOP"]
pub type DbgTim6StopR = crate::BitReader;
#[doc = "Field `DBG_TIM6_STOP` writer - DBG_TIM6_STOP"]
pub type DbgTim6StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - DBG_TIM7_STOP"]
pub type DbgTim7StopR = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - DBG_TIM7_STOP"]
pub type DbgTim7StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CAN2_STOP` reader - DBG_CAN2_STOP"]
pub type DbgCan2StopR = crate::BitReader;
#[doc = "Field `DBG_CAN2_STOP` writer - DBG_CAN2_STOP"]
pub type DbgCan2StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DbgSleepR {
        DbgSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DbgStandbyR {
        DbgStandbyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TraceIoenR {
        TraceIoenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TraceModeR {
        TraceModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - DBG_IWDG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBG_TIM1_STOP"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DbgTim1StopR {
        DbgTim1StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DbgTim2StopR {
        DbgTim2StopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBG_TIM3_STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DbgTim4StopR {
        DbgTim4StopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DBG_CAN1_STOP"]
    #[inline(always)]
    pub fn dbg_can1_stop(&self) -> DbgCan1StopR {
        DbgCan1StopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DbgI2c1SmbusTimeoutR {
        DbgI2c1SmbusTimeoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DbgI2c2SmbusTimeoutR {
        DbgI2c2SmbusTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DBG_TIM8_STOP"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DbgTim8StopR {
        DbgTim8StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DbgTim5StopR {
        DbgTim5StopR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DbgTim6StopR {
        DbgTim6StopR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DbgTim7StopR {
        DbgTim7StopR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DBG_CAN2_STOP"]
    #[inline(always)]
    pub fn dbg_can2_stop(&self) -> DbgCan2StopR {
        DbgCan2StopR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<CrSpec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DbgStopW<CrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DbgStandbyW<CrSpec> {
        DbgStandbyW::new(self, 2)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TraceIoenW<CrSpec> {
        TraceIoenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TraceModeW<CrSpec> {
        TraceModeW::new(self, 6)
    }
    #[doc = "Bit 8 - DBG_IWDG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<CrSpec> {
        DbgIwdgStopW::new(self, 8)
    }
    #[doc = "Bit 9 - DBG_WWDG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<CrSpec> {
        DbgWwdgStopW::new(self, 9)
    }
    #[doc = "Bit 10 - DBG_TIM1_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DbgTim1StopW<CrSpec> {
        DbgTim1StopW::new(self, 10)
    }
    #[doc = "Bit 11 - DBG_TIM2_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DbgTim2StopW<CrSpec> {
        DbgTim2StopW::new(self, 11)
    }
    #[doc = "Bit 12 - DBG_TIM3_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<CrSpec> {
        DbgTim3StopW::new(self, 12)
    }
    #[doc = "Bit 13 - DBG_TIM4_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DbgTim4StopW<CrSpec> {
        DbgTim4StopW::new(self, 13)
    }
    #[doc = "Bit 14 - DBG_CAN1_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_can1_stop(&mut self) -> DbgCan1StopW<CrSpec> {
        DbgCan1StopW::new(self, 14)
    }
    #[doc = "Bit 15 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DbgI2c1SmbusTimeoutW<CrSpec> {
        DbgI2c1SmbusTimeoutW::new(self, 15)
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DbgI2c2SmbusTimeoutW<CrSpec> {
        DbgI2c2SmbusTimeoutW::new(self, 16)
    }
    #[doc = "Bit 17 - DBG_TIM8_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim8_stop(&mut self) -> DbgTim8StopW<CrSpec> {
        DbgTim8StopW::new(self, 17)
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim5_stop(&mut self) -> DbgTim5StopW<CrSpec> {
        DbgTim5StopW::new(self, 18)
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DbgTim6StopW<CrSpec> {
        DbgTim6StopW::new(self, 19)
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DbgTim7StopW<CrSpec> {
        DbgTim7StopW::new(self, 20)
    }
    #[doc = "Bit 21 - DBG_CAN2_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_can2_stop(&mut self) -> DbgCan2StopW<CrSpec> {
        DbgCan2StopW::new(self, 21)
    }
}
#[doc = "DBGMCU_CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
