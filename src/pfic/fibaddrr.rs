#[doc = "Register `FIBADDRR` reader"]
pub type R = crate::R<FibaddrrSpec>;
#[doc = "Register `FIBADDRR` writer"]
pub type W = crate::W<FibaddrrSpec>;
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub type BaseaddrR = crate::FieldReader;
#[doc = "Field `BASEADDR` writer - BASEADDR"]
pub type BaseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BaseaddrR {
        BaseaddrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BaseaddrW<'_, FibaddrrSpec> {
        BaseaddrW::new(self, 28)
    }
}
#[doc = "Interrupt Fast Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fibaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fibaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FibaddrrSpec;
impl crate::RegisterSpec for FibaddrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fibaddrr::R`](R) reader structure"]
impl crate::Readable for FibaddrrSpec {}
#[doc = "`write(|w| ..)` method takes [`fibaddrr::W`](W) writer structure"]
impl crate::Writable for FibaddrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIBADDRR to value 0"]
impl crate::Resettable for FibaddrrSpec {}
