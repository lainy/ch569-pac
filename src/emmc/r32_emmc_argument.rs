#[doc = "Register `R32_EMMC_ARGUMENT` reader"]
pub type R = crate::R<R32EmmcArgumentSpec>;
#[doc = "Register `R32_EMMC_ARGUMENT` writer"]
pub type W = crate::W<R32EmmcArgumentSpec>;
#[doc = "Field `EMMC_ARGUMENT` reader - 32 bit command parameter register"]
pub type EmmcArgumentR = crate::FieldReader<u32>;
#[doc = "Field `EMMC_ARGUMENT` writer - 32 bit command parameter register"]
pub type EmmcArgumentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit command parameter register"]
    #[inline(always)]
    pub fn emmc_argument(&self) -> EmmcArgumentR {
        EmmcArgumentR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bit command parameter register"]
    #[inline(always)]
    pub fn emmc_argument(&mut self) -> EmmcArgumentW<'_, R32EmmcArgumentSpec> {
        EmmcArgumentW::new(self, 0)
    }
}
#[doc = "SD 32bits command argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_emmc_argument::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_emmc_argument::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32EmmcArgumentSpec;
impl crate::RegisterSpec for R32EmmcArgumentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_emmc_argument::R`](R) reader structure"]
impl crate::Readable for R32EmmcArgumentSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_emmc_argument::W`](W) writer structure"]
impl crate::Writable for R32EmmcArgumentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_EMMC_ARGUMENT to value 0"]
impl crate::Resettable for R32EmmcArgumentSpec {}
