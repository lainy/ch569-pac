#[doc = "Register `R8_UART1_FCR` reader"]
pub type R = crate::R<R8Uart1FcrSpec>;
#[doc = "Register `R8_UART1_FCR` writer"]
pub type W = crate::W<R8Uart1FcrSpec>;
#[doc = "Field `RB_FCR_FIFO_EN` reader - UART FIFO enable"]
pub type RbFcrFifoEnR = crate::BitReader;
#[doc = "Field `RB_FCR_FIFO_EN` writer - UART FIFO enable"]
pub type RbFcrFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FCR_RX_FIFO_CLR` reader - clear UART receiver FIFO, high action, auto clear"]
pub type RbFcrRxFifoClrR = crate::BitReader;
#[doc = "Field `RB_FCR_RX_FIFO_CLR` writer - clear UART receiver FIFO, high action, auto clear"]
pub type RbFcrRxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FCR_TX_FIFO_CLR` reader - clear UART transmitter FIFO, high action, auto clear"]
pub type RbFcrTxFifoClrR = crate::BitReader;
#[doc = "Field `RB_FCR_TX_FIFO_CLR` writer - clear UART transmitter FIFO, high action, auto clear"]
pub type RbFcrTxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_FCR_FIFO_TRIG` reader - UART receiver FIFO trigger level"]
pub type RbFcrFifoTrigR = crate::FieldReader;
#[doc = "Field `RB_FCR_FIFO_TRIG` writer - UART receiver FIFO trigger level"]
pub type RbFcrFifoTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - UART FIFO enable"]
    #[inline(always)]
    pub fn rb_fcr_fifo_en(&self) -> RbFcrFifoEnR {
        RbFcrFifoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clear UART receiver FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_rx_fifo_clr(&self) -> RbFcrRxFifoClrR {
        RbFcrRxFifoClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clear UART transmitter FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_tx_fifo_clr(&self) -> RbFcrTxFifoClrR {
        RbFcrTxFifoClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 6:7 - UART receiver FIFO trigger level"]
    #[inline(always)]
    pub fn rb_fcr_fifo_trig(&self) -> RbFcrFifoTrigR {
        RbFcrFifoTrigR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - UART FIFO enable"]
    #[inline(always)]
    pub fn rb_fcr_fifo_en(&mut self) -> RbFcrFifoEnW<'_, R8Uart1FcrSpec> {
        RbFcrFifoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clear UART receiver FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_rx_fifo_clr(&mut self) -> RbFcrRxFifoClrW<'_, R8Uart1FcrSpec> {
        RbFcrRxFifoClrW::new(self, 1)
    }
    #[doc = "Bit 2 - clear UART transmitter FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rb_fcr_tx_fifo_clr(&mut self) -> RbFcrTxFifoClrW<'_, R8Uart1FcrSpec> {
        RbFcrTxFifoClrW::new(self, 2)
    }
    #[doc = "Bits 6:7 - UART receiver FIFO trigger level"]
    #[inline(always)]
    pub fn rb_fcr_fifo_trig(&mut self) -> RbFcrFifoTrigW<'_, R8Uart1FcrSpec> {
        RbFcrFifoTrigW::new(self, 6)
    }
}
#[doc = "UART1 FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1FcrSpec;
impl crate::RegisterSpec for R8Uart1FcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart1_fcr::R`](R) reader structure"]
impl crate::Readable for R8Uart1FcrSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart1_fcr::W`](W) writer structure"]
impl crate::Writable for R8Uart1FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART1_FCR to value 0"]
impl crate::Resettable for R8Uart1FcrSpec {}
