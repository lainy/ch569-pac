#[doc = "Register `R8_UART2_IER` reader"]
pub type R = crate::R<R8Uart2IerSpec>;
#[doc = "Register `R8_UART2_IER` writer"]
pub type W = crate::W<R8Uart2IerSpec>;
#[doc = "Field `RB_IER_RECV_RDY` reader - UART interrupt enable for receiver data ready"]
pub type RbIerRecvRdyR = crate::BitReader;
#[doc = "Field `RB_IER_RECV_RDY` writer - UART interrupt enable for receiver data ready"]
pub type RbIerRecvRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_IER_THR_EMPTY` reader - UART interrupt enable for THR empty"]
pub type RbIerThrEmptyR = crate::BitReader;
#[doc = "Field `RB_IER_THR_EMPTY` writer - UART interrupt enable for THR empty"]
pub type RbIerThrEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_IER_LINE_STAT` reader - UART interrupt enable for receiver line status"]
pub type RbIerLineStatR = crate::BitReader;
#[doc = "Field `RB_IER_LINE_STAT` writer - UART interrupt enable for receiver line status"]
pub type RbIerLineStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_IER_TXD_EN` reader - UART TXD pin enable"]
pub type RbIerTxdEnR = crate::BitReader;
#[doc = "Field `RB_IER_TXD_EN` writer - UART TXD pin enable"]
pub type RbIerTxdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_IER_RESET` reader - UART software reset control, high action, auto clear"]
pub type RbIerResetR = crate::BitReader;
#[doc = "Field `RB_IER_RESET` writer - UART software reset control, high action, auto clear"]
pub type RbIerResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART interrupt enable for receiver data ready"]
    #[inline(always)]
    pub fn rb_ier_recv_rdy(&self) -> RbIerRecvRdyR {
        RbIerRecvRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART interrupt enable for THR empty"]
    #[inline(always)]
    pub fn rb_ier_thr_empty(&self) -> RbIerThrEmptyR {
        RbIerThrEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART interrupt enable for receiver line status"]
    #[inline(always)]
    pub fn rb_ier_line_stat(&self) -> RbIerLineStatR {
        RbIerLineStatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - UART TXD pin enable"]
    #[inline(always)]
    pub fn rb_ier_txd_en(&self) -> RbIerTxdEnR {
        RbIerTxdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART software reset control, high action, auto clear"]
    #[inline(always)]
    pub fn rb_ier_reset(&self) -> RbIerResetR {
        RbIerResetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART interrupt enable for receiver data ready"]
    #[inline(always)]
    pub fn rb_ier_recv_rdy(&mut self) -> RbIerRecvRdyW<'_, R8Uart2IerSpec> {
        RbIerRecvRdyW::new(self, 0)
    }
    #[doc = "Bit 1 - UART interrupt enable for THR empty"]
    #[inline(always)]
    pub fn rb_ier_thr_empty(&mut self) -> RbIerThrEmptyW<'_, R8Uart2IerSpec> {
        RbIerThrEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - UART interrupt enable for receiver line status"]
    #[inline(always)]
    pub fn rb_ier_line_stat(&mut self) -> RbIerLineStatW<'_, R8Uart2IerSpec> {
        RbIerLineStatW::new(self, 2)
    }
    #[doc = "Bit 6 - UART TXD pin enable"]
    #[inline(always)]
    pub fn rb_ier_txd_en(&mut self) -> RbIerTxdEnW<'_, R8Uart2IerSpec> {
        RbIerTxdEnW::new(self, 6)
    }
    #[doc = "Bit 7 - UART software reset control, high action, auto clear"]
    #[inline(always)]
    pub fn rb_ier_reset(&mut self) -> RbIerResetW<'_, R8Uart2IerSpec> {
        RbIerResetW::new(self, 7)
    }
}
#[doc = "UART2 interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart2IerSpec;
impl crate::RegisterSpec for R8Uart2IerSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart2_ier::R`](R) reader structure"]
impl crate::Readable for R8Uart2IerSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart2_ier::W`](W) writer structure"]
impl crate::Writable for R8Uart2IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART2_IER to value 0"]
impl crate::Resettable for R8Uart2IerSpec {}
