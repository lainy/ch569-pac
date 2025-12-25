#[doc = "Register `R8_UART1_LSR` reader"]
pub type R = crate::R<R8Uart1LsrSpec>;
#[doc = "Field `RB_LSR_DATA_RDY` reader - UART receiver fifo data ready status"]
pub type RbLsrDataRdyR = crate::BitReader;
#[doc = "Field `RB_LSR_OVER_ERR` reader - UART receiver overrun error"]
pub type RbLsrOverErrR = crate::BitReader;
#[doc = "Field `RB_LSR_PAR_ERR` reader - UART receiver frame error"]
pub type RbLsrParErrR = crate::BitReader;
#[doc = "Field `RB_LSR_FRAME_ERR` reader - UART receiver frame error"]
pub type RbLsrFrameErrR = crate::BitReader;
#[doc = "Field `RB_LSR_BREAK_ERR` reader - UART receiver break error"]
pub type RbLsrBreakErrR = crate::BitReader;
#[doc = "Field `RB_LSR_TX_FIFO_EMP` reader - UART transmitter fifo empty status"]
pub type RbLsrTxFifoEmpR = crate::BitReader;
#[doc = "Field `RB_LSR_TX_ALL_EMP` reader - UART transmitter all empty status"]
pub type RbLsrTxAllEmpR = crate::BitReader;
#[doc = "Field `RB_LSR_ERR_RX_FIFO` reader - indicate error in UART receiver fifo"]
pub type RbLsrErrRxFifoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART receiver fifo data ready status"]
    #[inline(always)]
    pub fn rb_lsr_data_rdy(&self) -> RbLsrDataRdyR {
        RbLsrDataRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART receiver overrun error"]
    #[inline(always)]
    pub fn rb_lsr_over_err(&self) -> RbLsrOverErrR {
        RbLsrOverErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART receiver frame error"]
    #[inline(always)]
    pub fn rb_lsr_par_err(&self) -> RbLsrParErrR {
        RbLsrParErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART receiver frame error"]
    #[inline(always)]
    pub fn rb_lsr_frame_err(&self) -> RbLsrFrameErrR {
        RbLsrFrameErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART receiver break error"]
    #[inline(always)]
    pub fn rb_lsr_break_err(&self) -> RbLsrBreakErrR {
        RbLsrBreakErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmitter fifo empty status"]
    #[inline(always)]
    pub fn rb_lsr_tx_fifo_emp(&self) -> RbLsrTxFifoEmpR {
        RbLsrTxFifoEmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART transmitter all empty status"]
    #[inline(always)]
    pub fn rb_lsr_tx_all_emp(&self) -> RbLsrTxAllEmpR {
        RbLsrTxAllEmpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - indicate error in UART receiver fifo"]
    #[inline(always)]
    pub fn rb_lsr_err_rx_fifo(&self) -> RbLsrErrRxFifoR {
        RbLsrErrRxFifoR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART1 line status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1LsrSpec;
impl crate::RegisterSpec for R8Uart1LsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart1_lsr::R`](R) reader structure"]
impl crate::Readable for R8Uart1LsrSpec {}
#[doc = "`reset()` method sets R8_UART1_LSR to value 0xc0"]
impl crate::Resettable for R8Uart1LsrSpec {
    const RESET_VALUE: u8 = 0xc0;
}
