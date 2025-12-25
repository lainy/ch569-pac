#[doc = "Register `R8_HSPI_INT_FLAG` reader"]
pub type R = crate::R<R8HspiIntFlagSpec>;
#[doc = "Register `R8_HSPI_INT_FLAG` writer"]
pub type W = crate::W<R8HspiIntFlagSpec>;
#[doc = "Field `RB_HSPI_IF_T_DONE` reader - interrupt flag for parallel if transmit done"]
pub type RbHspiIfTDoneR = crate::BitReader;
#[doc = "Field `RB_HSPI_IF_T_DONE` writer - interrupt flag for parallel if transmit done"]
pub type RbHspiIfTDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_IF_R_DONE` reader - interrupt flag for parallel if receive done"]
pub type RbHspiIfRDoneR = crate::BitReader;
#[doc = "Field `RB_HSPI_IF_R_DONE` writer - interrupt flag for parallel if receive done"]
pub type RbHspiIfRDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_IF_FIFO_OV` reader - interrupt flag for parallel if FIFO overflow"]
pub type RbHspiIfFifoOvR = crate::BitReader;
#[doc = "Field `RB_HSPI_IF_FIFO_OV` writer - interrupt flag for parallel if FIFO overflow"]
pub type RbHspiIfFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_HSPI_IF_B_DONE` reader - interrupt flag for parallel if tx burst done"]
pub type RbHspiIfBDoneR = crate::BitReader;
#[doc = "Field `RB_HSPI_IF_B_DONE` writer - interrupt flag for parallel if tx burst done"]
pub type RbHspiIfBDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt flag for parallel if transmit done"]
    #[inline(always)]
    pub fn rb_hspi_if_t_done(&self) -> RbHspiIfTDoneR {
        RbHspiIfTDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt flag for parallel if receive done"]
    #[inline(always)]
    pub fn rb_hspi_if_r_done(&self) -> RbHspiIfRDoneR {
        RbHspiIfRDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt flag for parallel if FIFO overflow"]
    #[inline(always)]
    pub fn rb_hspi_if_fifo_ov(&self) -> RbHspiIfFifoOvR {
        RbHspiIfFifoOvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt flag for parallel if tx burst done"]
    #[inline(always)]
    pub fn rb_hspi_if_b_done(&self) -> RbHspiIfBDoneR {
        RbHspiIfBDoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag for parallel if transmit done"]
    #[inline(always)]
    pub fn rb_hspi_if_t_done(&mut self) -> RbHspiIfTDoneW<'_, R8HspiIntFlagSpec> {
        RbHspiIfTDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - interrupt flag for parallel if receive done"]
    #[inline(always)]
    pub fn rb_hspi_if_r_done(&mut self) -> RbHspiIfRDoneW<'_, R8HspiIntFlagSpec> {
        RbHspiIfRDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt flag for parallel if FIFO overflow"]
    #[inline(always)]
    pub fn rb_hspi_if_fifo_ov(&mut self) -> RbHspiIfFifoOvW<'_, R8HspiIntFlagSpec> {
        RbHspiIfFifoOvW::new(self, 2)
    }
    #[doc = "Bit 3 - interrupt flag for parallel if tx burst done"]
    #[inline(always)]
    pub fn rb_hspi_if_b_done(&mut self) -> RbHspiIfBDoneW<'_, R8HspiIntFlagSpec> {
        RbHspiIfBDoneW::new(self, 3)
    }
}
#[doc = "parallel if interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_hspi_int_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_hspi_int_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8HspiIntFlagSpec;
impl crate::RegisterSpec for R8HspiIntFlagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_hspi_int_flag::R`](R) reader structure"]
impl crate::Readable for R8HspiIntFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_hspi_int_flag::W`](W) writer structure"]
impl crate::Writable for R8HspiIntFlagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_HSPI_INT_FLAG to value 0"]
impl crate::Resettable for R8HspiIntFlagSpec {}
