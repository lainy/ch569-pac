#[doc = "Register `R8_DVP_CR0` reader"]
pub type R = crate::R<R8DvpCr0Spec>;
#[doc = "Register `R8_DVP_CR0` writer"]
pub type W = crate::W<R8DvpCr0Spec>;
#[doc = "Field `RB_DVP_ENABLE` reader - DVP enable"]
pub type RbDvpEnableR = crate::BitReader;
#[doc = "Field `RB_DVP_ENABLE` writer - DVP enable"]
pub type RbDvpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_V_POLAR` reader - DVP VSYNC polarity control"]
pub type RbDvpVPolarR = crate::BitReader;
#[doc = "Field `RB_DVP_V_POLAR` writer - DVP VSYNC polarity control"]
pub type RbDvpVPolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_H_POLAR` reader - DVP HSYNC polarity control"]
pub type RbDvpHPolarR = crate::BitReader;
#[doc = "Field `RB_DVP_H_POLAR` writer - DVP HSYNC polarity control"]
pub type RbDvpHPolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_P_POLAR` reader - DVP PCLK polarity control"]
pub type RbDvpPPolarR = crate::BitReader;
#[doc = "Field `RB_DVP_P_POLAR` writer - DVP PCLK polarity control"]
pub type RbDvpPPolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` reader - DVP data bit width confguration"]
pub type RbDvpMskDatModR = crate::FieldReader;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` writer - DVP data bit width confguration"]
pub type RbDvpMskDatModW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_DVP_JPEG` reader - DVP JPEG mode"]
pub type RbDvpJpegR = crate::BitReader;
#[doc = "Field `RB_DVP_JPEG` writer - DVP JPEG mode"]
pub type RbDvpJpegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_DVP_RAW_CM` reader - DVP row count mode"]
pub type RbDvpRawCmR = crate::BitReader;
#[doc = "Field `RB_DVP_RAW_CM` writer - DVP row count mode"]
pub type RbDvpRawCmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    pub fn rb_dvp_enable(&self) -> RbDvpEnableR {
        RbDvpEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_v_polar(&self) -> RbDvpVPolarR {
        RbDvpVPolarR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_h_polar(&self) -> RbDvpHPolarR {
        RbDvpHPolarR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    pub fn rb_dvp_p_polar(&self) -> RbDvpPPolarR {
        RbDvpPPolarR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DVP data bit width confguration"]
    #[inline(always)]
    pub fn rb_dvp_msk_dat_mod(&self) -> RbDvpMskDatModR {
        RbDvpMskDatModR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    pub fn rb_dvp_jpeg(&self) -> RbDvpJpegR {
        RbDvpJpegR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DVP row count mode"]
    #[inline(always)]
    pub fn rb_dvp_raw_cm(&self) -> RbDvpRawCmR {
        RbDvpRawCmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    pub fn rb_dvp_enable(&mut self) -> RbDvpEnableW<'_, R8DvpCr0Spec> {
        RbDvpEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_v_polar(&mut self) -> RbDvpVPolarW<'_, R8DvpCr0Spec> {
        RbDvpVPolarW::new(self, 1)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_h_polar(&mut self) -> RbDvpHPolarW<'_, R8DvpCr0Spec> {
        RbDvpHPolarW::new(self, 2)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    pub fn rb_dvp_p_polar(&mut self) -> RbDvpPPolarW<'_, R8DvpCr0Spec> {
        RbDvpPPolarW::new(self, 3)
    }
    #[doc = "Bits 4:5 - DVP data bit width confguration"]
    #[inline(always)]
    pub fn rb_dvp_msk_dat_mod(&mut self) -> RbDvpMskDatModW<'_, R8DvpCr0Spec> {
        RbDvpMskDatModW::new(self, 4)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    pub fn rb_dvp_jpeg(&mut self) -> RbDvpJpegW<'_, R8DvpCr0Spec> {
        RbDvpJpegW::new(self, 6)
    }
    #[doc = "Bit 7 - DVP row count mode"]
    #[inline(always)]
    pub fn rb_dvp_raw_cm(&mut self) -> RbDvpRawCmW<'_, R8DvpCr0Spec> {
        RbDvpRawCmW::new(self, 7)
    }
}
#[doc = "DVP control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_dvp_cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_dvp_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8DvpCr0Spec;
impl crate::RegisterSpec for R8DvpCr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_dvp_cr0::R`](R) reader structure"]
impl crate::Readable for R8DvpCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`r8_dvp_cr0::W`](W) writer structure"]
impl crate::Writable for R8DvpCr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_DVP_CR0 to value 0"]
impl crate::Resettable for R8DvpCr0Spec {}
