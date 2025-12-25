#[doc = "Register `R8_UART1_MCR` reader"]
pub type R = crate::R<R8Uart1McrSpec>;
#[doc = "Register `R8_UART1_MCR` writer"]
pub type W = crate::W<R8Uart1McrSpec>;
#[doc = "Field `RB_MCR_OUT2` reader - UART1 control OUT2"]
pub type RbMcrOut2R = crate::BitReader;
#[doc = "Field `RB_MCR_OUT2` writer - UART1 control OUT2"]
pub type RbMcrOut2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_MCR_AU_FLOW_EN` reader - UART0 enable autoflow control"]
pub type RbMcrAuFlowEnR = crate::BitReader;
#[doc = "Field `RB_MCR_AU_FLOW_EN` writer - UART0 enable autoflow control"]
pub type RbMcrAuFlowEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - UART1 control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&self) -> RbMcrOut2R {
        RbMcrOut2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn rb_mcr_au_flow_en(&self) -> RbMcrAuFlowEnR {
        RbMcrAuFlowEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - UART1 control OUT2"]
    #[inline(always)]
    pub fn rb_mcr_out2(&mut self) -> RbMcrOut2W<'_, R8Uart1McrSpec> {
        RbMcrOut2W::new(self, 3)
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn rb_mcr_au_flow_en(&mut self) -> RbMcrAuFlowEnW<'_, R8Uart1McrSpec> {
        RbMcrAuFlowEnW::new(self, 5)
    }
}
#[doc = "UART1 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1McrSpec;
impl crate::RegisterSpec for R8Uart1McrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart1_mcr::R`](R) reader structure"]
impl crate::Readable for R8Uart1McrSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart1_mcr::W`](W) writer structure"]
impl crate::Writable for R8Uart1McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART1_MCR to value 0"]
impl crate::Resettable for R8Uart1McrSpec {}
