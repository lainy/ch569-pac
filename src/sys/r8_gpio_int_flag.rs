#[doc = "Register `R8_GPIO_INT_FLAG` reader"]
pub type R = crate::R<R8GpioIntFlagSpec>;
#[doc = "Register `R8_GPIO_INT_FLAG` writer"]
pub type W = crate::W<R8GpioIntFlagSpec>;
#[doc = "Field `RB_GPIO_PA2_IF` reader - PA2 pin interrupt flag"]
pub type RbGpioPa2IfR = crate::BitReader;
#[doc = "Field `RB_GPIO_PA2_IF` writer - PA2 pin interrupt flag"]
pub type RbGpioPa2IfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PA2 pin interrupt flag"]
    #[inline(always)]
    pub fn rb_gpio_pa2_if(&self) -> RbGpioPa2IfR {
        RbGpioPa2IfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PA2 pin interrupt flag"]
    #[inline(always)]
    pub fn rb_gpio_pa2_if(&mut self) -> RbGpioPa2IfW<'_, R8GpioIntFlagSpec> {
        RbGpioPa2IfW::new(self, 0)
    }
}
#[doc = "GPIO interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8GpioIntFlagSpec;
impl crate::RegisterSpec for R8GpioIntFlagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_gpio_int_flag::R`](R) reader structure"]
impl crate::Readable for R8GpioIntFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_gpio_int_flag::W`](W) writer structure"]
impl crate::Writable for R8GpioIntFlagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_GPIO_INT_FLAG to value 0"]
impl crate::Resettable for R8GpioIntFlagSpec {}
