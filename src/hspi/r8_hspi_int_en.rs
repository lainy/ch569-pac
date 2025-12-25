#[doc = "Register `R8_HSPI_INT_EN` reader"]
pub type R = crate::R<R8HspiIntEnSpec>;
#[doc = "Register `R8_HSPI_INT_EN` writer"]
pub type W = crate::W<R8HspiIntEnSpec>;
#[doc = "Field `RB_HSPI_IE_T_DONE` reader - parallel if transmit done interrupt enable"]
pub type RbHspiIeTDoneR = crate::BitReader;
#[doc = "Field `RB_HSPI_IE_T_DONE` writer - parallel if transmit done interrupt enable"]
pub type RbHspiIeTDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_IE_R_DONE` reader - parallel if receive done interrupt enable"]
pub type RbHspiIeRDoneR = crate::BitReader;
#[doc = "Field `RB_HSPI_IE_R_DONE` writer - parallel if receive done interrupt enable"]
pub type RbHspiIeRDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_IE_FIFO_OV` reader - parallel if fifo overflow interrupt enable"]
pub type RbHspiIeFifoOvR = crate::BitReader;
#[doc = "Field `RB_HSPI_IE_FIFO_OV` writer - parallel if fifo overflow interrupt enable"]
pub type RbHspiIeFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_IE_B_DONE` reader - parallel if tx burst done interrupt enable"]
pub type RbHspiIeBDoneR = crate::BitReader;
#[doc = "Field `RB_HSPI_IE_B_DONE` writer - parallel if tx burst done interrupt enable"]
pub type RbHspiIeBDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - parallel if transmit done interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_t_done(&self) -> RbHspiIeTDoneR {
        RbHspiIeTDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if receive done interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_r_done(&self) -> RbHspiIeRDoneR {
        RbHspiIeRDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - parallel if fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_fifo_ov(&self) -> RbHspiIeFifoOvR {
        RbHspiIeFifoOvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - parallel if tx burst done interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_b_done(&self) -> RbHspiIeBDoneR {
        RbHspiIeBDoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if transmit done interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_t_done(&mut self) -> RbHspiIeTDoneW<'_, R8HspiIntEnSpec> {
        RbHspiIeTDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - parallel if receive done interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_r_done(&mut self) -> RbHspiIeRDoneW<'_, R8HspiIntEnSpec> {
        RbHspiIeRDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - parallel if fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_fifo_ov(&mut self) -> RbHspiIeFifoOvW<'_, R8HspiIntEnSpec> {
        RbHspiIeFifoOvW::new(self, 2)
    }
    #[doc = "Bit 3 - parallel if tx burst done interrupt enable"]
    #[inline(always)]
    pub fn rb_hspi_ie_b_done(&mut self) -> RbHspiIeBDoneW<'_, R8HspiIntEnSpec> {
        RbHspiIeBDoneW::new(self, 3)
    }
}
#[doc = "parallel if interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiIntEnSpec;
impl crate::RegisterSpec for R8HspiIntEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_int_en::R`](R) reader structure"]
impl crate::Readable for R8HspiIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_int_en::W`](W) writer structure"]
impl crate::Writable for R8HspiIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_INT_EN to value 0"]
impl crate::Resettable for R8HspiIntEnSpec {}
