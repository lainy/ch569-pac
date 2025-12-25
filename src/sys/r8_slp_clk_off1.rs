#[doc = "Register `R8_SLP_CLK_OFF1` reader"]
pub type R = crate::R<R8SlpClkOff1Spec>;
#[doc = "Register `R8_SLP_CLK_OFF1` writer"]
pub type W = crate::W<R8SlpClkOff1Spec>;
#[doc = "Field `RB_SLP_CLK_SPI0` reader - sleep SPI0 clock"]
pub type RbSlpClkSpi0R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_SPI0` writer - sleep SPI0 clock"]
pub type RbSlpClkSpi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_SPI1` reader - sleep SPI1 clock"]
pub type RbSlpClkSpi1R = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_SPI1` writer - sleep SPI1 clock"]
pub type RbSlpClkSpi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_EMMC` reader - sleep EMMC clock"]
pub type RbSlpClkEmmcR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_EMMC` writer - sleep EMMC clock"]
pub type RbSlpClkEmmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_HSPI` reader - sleep HSPI clock"]
pub type RbSlpClkHspiR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_HSPI` writer - sleep HSPI clock"]
pub type RbSlpClkHspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_USBHS` reader - sleep USBHS clock"]
pub type RbSlpClkUsbhsR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_USBHS` writer - sleep USBHS clock"]
pub type RbSlpClkUsbhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_USBSS` reader - sleep USBSS clock"]
pub type RbSlpClkUsbssR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_USBSS` writer - sleep USBSS clock"]
pub type RbSlpClkUsbssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_SERD` reader - sleep SERD clock"]
pub type RbSlpClkSerdR = crate::BitReader;
#[doc = "Field `RB_SLP_CLK_SERD` writer - sleep SERD clock"]
pub type RbSlpClkSerdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SLP_CLK_DVP` reader - sleep DVP clock"]
pub type RbSlpClkDvpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sleep SPI0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi0(&self) -> RbSlpClkSpi0R {
        RbSlpClkSpi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep SPI1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi1(&self) -> RbSlpClkSpi1R {
        RbSlpClkSpi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sleep EMMC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_emmc(&self) -> RbSlpClkEmmcR {
        RbSlpClkEmmcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sleep HSPI clock"]
    #[inline(always)]
    pub fn rb_slp_clk_hspi(&self) -> RbSlpClkHspiR {
        RbSlpClkHspiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sleep USBHS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbhs(&self) -> RbSlpClkUsbhsR {
        RbSlpClkUsbhsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - sleep USBSS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbss(&self) -> RbSlpClkUsbssR {
        RbSlpClkUsbssR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - sleep SERD clock"]
    #[inline(always)]
    pub fn rb_slp_clk_serd(&self) -> RbSlpClkSerdR {
        RbSlpClkSerdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - sleep DVP clock"]
    #[inline(always)]
    pub fn rb_slp_clk_dvp(&self) -> RbSlpClkDvpR {
        RbSlpClkDvpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sleep SPI0 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi0(&mut self) -> RbSlpClkSpi0W<'_, R8SlpClkOff1Spec> {
        RbSlpClkSpi0W::new(self, 0)
    }
    #[doc = "Bit 1 - sleep SPI1 clock"]
    #[inline(always)]
    pub fn rb_slp_clk_spi1(&mut self) -> RbSlpClkSpi1W<'_, R8SlpClkOff1Spec> {
        RbSlpClkSpi1W::new(self, 1)
    }
    #[doc = "Bit 2 - sleep EMMC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_emmc(&mut self) -> RbSlpClkEmmcW<'_, R8SlpClkOff1Spec> {
        RbSlpClkEmmcW::new(self, 2)
    }
    #[doc = "Bit 3 - sleep HSPI clock"]
    #[inline(always)]
    pub fn rb_slp_clk_hspi(&mut self) -> RbSlpClkHspiW<'_, R8SlpClkOff1Spec> {
        RbSlpClkHspiW::new(self, 3)
    }
    #[doc = "Bit 4 - sleep USBHS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbhs(&mut self) -> RbSlpClkUsbhsW<'_, R8SlpClkOff1Spec> {
        RbSlpClkUsbhsW::new(self, 4)
    }
    #[doc = "Bit 5 - sleep USBSS clock"]
    #[inline(always)]
    pub fn rb_slp_clk_usbss(&mut self) -> RbSlpClkUsbssW<'_, R8SlpClkOff1Spec> {
        RbSlpClkUsbssW::new(self, 5)
    }
    #[doc = "Bit 6 - sleep SERD clock"]
    #[inline(always)]
    pub fn rb_slp_clk_serd(&mut self) -> RbSlpClkSerdW<'_, R8SlpClkOff1Spec> {
        RbSlpClkSerdW::new(self, 6)
    }
}
#[doc = "sleep clock off control byte 1\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_slp_clk_off1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_slp_clk_off1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SlpClkOff1Spec;
impl crate::RegisterSpec for R8SlpClkOff1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_slp_clk_off1::R`](R) reader structure"]
impl crate::Readable for R8SlpClkOff1Spec {}
#[doc = "`write(|w| ..)` method takes [`r8_slp_clk_off1::W`](W) writer structure"]
impl crate::Writable for R8SlpClkOff1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SLP_CLK_OFF1 to value 0"]
impl crate::Resettable for R8SlpClkOff1Spec {}
