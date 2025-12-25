#[doc = "Register `R32_HSPI_UDF0` reader"]
pub type R = crate::R<R32HspiUdf0Spec>;
#[doc = "Register `R32_HSPI_UDF0` writer"]
pub type W = crate::W<R32HspiUdf0Spec>;
#[doc = "Field `RB_HSPI_UDF0` reader - parallel if user defined field 0 register"]
pub type RbHspiUdf0R = crate::FieldReader<u32>;
#[doc = "Field `RB_HSPI_UDF0` writer - parallel if user defined field 0 register"]
pub type RbHspiUdf0W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - parallel if user defined field 0 register"]
    #[inline(always)]
    pub fn rb_hspi_udf0(&self) -> RbHspiUdf0R {
        RbHspiUdf0R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - parallel if user defined field 0 register"]
    #[inline(always)]
    pub fn rb_hspi_udf0(&mut self) -> RbHspiUdf0W<'_, R32HspiUdf0Spec> {
        RbHspiUdf0W::new(self, 0)
    }
}
#[doc = "parallel if user defined field 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_hspi_udf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_hspi_udf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32HspiUdf0Spec;
impl crate::RegisterSpec for R32HspiUdf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_hspi_udf0::R`](R) reader structure"]
impl crate::Readable for R32HspiUdf0Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_hspi_udf0::W`](W) writer structure"]
impl crate::Writable for R32HspiUdf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_HSPI_UDF0 to value 0"]
impl crate::Resettable for R32HspiUdf0Spec {}
