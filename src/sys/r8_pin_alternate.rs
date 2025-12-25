#[doc = "Register `R8_PIN_ALTERNATE` reader"]
pub type R = crate::R<R8PinAlternateSpec>;
#[doc = "Register `R8_PIN_ALTERNATE` writer"]
pub type W = crate::W<R8PinAlternateSpec>;
#[doc = "Field `RB_PIN_MII` reader - ETH mii interface selection"]
pub type RbPinMiiR = crate::BitReader;
#[doc = "Field `RB_PIN_MII` writer - ETH mii interface selection"]
pub type RbPinMiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_TMR1` reader - TMR1 alternate pin enable"]
pub type RbPinTmr1R = crate::BitReader;
#[doc = "Field `RB_PIN_TMR1` writer - TMR1 alternate pin enable"]
pub type RbPinTmr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_TMR2` reader - TMR2 alternate pin enable"]
pub type RbPinTmr2R = crate::BitReader;
#[doc = "Field `RB_PIN_TMR2` writer - TMR2 alternate pin enable"]
pub type RbPinTmr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_UART0` reader - RXD0/TXD0 alternate pin enable"]
pub type RbPinUart0R = crate::BitReader;
#[doc = "Field `RB_PIN_UART0` writer - RXD0/TXD0 alternate pin enable"]
pub type RbPinUart0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETH mii interface selection"]
    #[inline(always)]
    pub fn rb_pin_mii(&self) -> RbPinMiiR {
        RbPinMiiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&self) -> RbPinTmr1R {
        RbPinTmr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&self) -> RbPinTmr2R {
        RbPinTmr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&self) -> RbPinUart0R {
        RbPinUart0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETH mii interface selection"]
    #[inline(always)]
    pub fn rb_pin_mii(&mut self) -> RbPinMiiW<'_, R8PinAlternateSpec> {
        RbPinMiiW::new(self, 0)
    }
    #[doc = "Bit 1 - TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&mut self) -> RbPinTmr1W<'_, R8PinAlternateSpec> {
        RbPinTmr1W::new(self, 1)
    }
    #[doc = "Bit 2 - TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&mut self) -> RbPinTmr2W<'_, R8PinAlternateSpec> {
        RbPinTmr2W::new(self, 2)
    }
    #[doc = "Bit 4 - RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&mut self) -> RbPinUart0W<'_, R8PinAlternateSpec> {
        RbPinUart0W::new(self, 4)
    }
}
#[doc = "alternate pin control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pin_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pin_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PinAlternateSpec;
impl crate::RegisterSpec for R8PinAlternateSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pin_alternate::R`](R) reader structure"]
impl crate::Readable for R8PinAlternateSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pin_alternate::W`](W) writer structure"]
impl crate::Writable for R8PinAlternateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PIN_ALTERNATE to value 0"]
impl crate::Resettable for R8PinAlternateSpec {}
