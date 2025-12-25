#[doc = "Register `R8_EMMC_TIMEOUT` reader"]
pub type R = crate::R<R8EmmcTimeoutSpec>;
#[doc = "Register `R8_EMMC_TIMEOUT` writer"]
pub type W = crate::W<R8EmmcTimeoutSpec>;
#[doc = "Field `RB_EMMC_TOCNT_MASK` reader - response data timeout configuration"]
pub type RbEmmcTocntMaskR = crate::FieldReader;
#[doc = "Field `RB_EMMC_TOCNT_MASK` writer - response data timeout configuration"]
pub type RbEmmcTocntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - response data timeout configuration"]
    #[inline(always)]
    pub fn rb_emmc_tocnt_mask(&self) -> RbEmmcTocntMaskR {
        RbEmmcTocntMaskR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - response data timeout configuration"]
    #[inline(always)]
    pub fn rb_emmc_tocnt_mask(&mut self) -> RbEmmcTocntMaskW<'_, R8EmmcTimeoutSpec> {
        RbEmmcTocntMaskW::new(self, 0)
    }
}
#[doc = "SD 8bits data timeout value\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_emmc_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_emmc_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8EmmcTimeoutSpec;
impl crate::RegisterSpec for R8EmmcTimeoutSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_emmc_timeout::R`](R) reader structure"]
impl crate::Readable for R8EmmcTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_emmc_timeout::W`](W) writer structure"]
impl crate::Writable for R8EmmcTimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_EMMC_TIMEOUT to value 0x0c"]
impl crate::Resettable for R8EmmcTimeoutSpec {
    const RESET_VALUE: u8 = 0x0c;
}
