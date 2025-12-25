#[doc = "Register `R8_GPIO_INT_ENABLE` reader"]
pub type R = crate::R<R8GpioIntEnableSpec>;
#[doc = "Register `R8_GPIO_INT_ENABLE` writer"]
pub type W = crate::W<R8GpioIntEnableSpec>;
#[doc = "Field `RB_GPIO_PA2_IE` reader - PA2 pin interrupt enable"]
pub type RbGpioPa2IeR = crate::BitReader;
#[doc = "Field `RB_GPIO_PA2_IE` writer - PA2 pin interrupt enable"]
pub type RbGpioPa2IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PA2 pin interrupt enable"]
    #[inline(always)]
    pub fn rb_gpio_pa2_ie(&self) -> RbGpioPa2IeR {
        RbGpioPa2IeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PA2 pin interrupt enable"]
    #[inline(always)]
    pub fn rb_gpio_pa2_ie(&mut self) -> RbGpioPa2IeW<'_, R8GpioIntEnableSpec> {
        RbGpioPa2IeW::new(self, 0)
    }
}
#[doc = "GPIO interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8GpioIntEnableSpec;
impl crate::RegisterSpec for R8GpioIntEnableSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_gpio_int_enable::R`](R) reader structure"]
impl crate::Readable for R8GpioIntEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_gpio_int_enable::W`](W) writer structure"]
impl crate::Writable for R8GpioIntEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_GPIO_INT_ENABLE to value 0"]
impl crate::Resettable for R8GpioIntEnableSpec {}
