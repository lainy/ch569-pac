#[doc = "Register `ITHRESDR` reader"]
pub type R = crate::R<IthresdrSpec>;
#[doc = "Register `ITHRESDR` writer"]
pub type W = crate::W<IthresdrSpec>;
#[doc = "Field `THRESHOLD` reader - THRESHOLD"]
pub type ThresholdR = crate::FieldReader;
#[doc = "Field `THRESHOLD` writer - THRESHOLD"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, IthresdrSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "Interrupt Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ithresdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ithresdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IthresdrSpec;
impl crate::RegisterSpec for IthresdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ithresdr::R`](R) reader structure"]
impl crate::Readable for IthresdrSpec {}
#[doc = "`write(|w| ..)` method takes [`ithresdr::W`](W) writer structure"]
impl crate::Writable for IthresdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITHRESDR to value 0"]
impl crate::Resettable for IthresdrSpec {}
