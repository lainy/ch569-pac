#[doc = "Register `HSPI_RX_SC` reader"]
pub type R = crate::R<HspiRxScSpec>;
#[doc = "Register `HSPI_RX_SC` writer"]
pub type W = crate::W<HspiRxScSpec>;
#[doc = "Field `RB_HSPI_RX_NUM` reader - parallel if rx sequence num"]
pub type RbHspiRxNumR = crate::FieldReader;
#[doc = "Field `RB_HSPI_RX_NUM` writer - parallel if rx sequence num"]
pub type RbHspiRxNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RB_HSPI_RX_TOG` reader - parallel if rx addr toggle flag"]
pub type RbHspiRxTogR = crate::BitReader;
#[doc = "Field `RB_HSPI_RX_TOG` writer - parallel if rx addr toggle flag"]
pub type RbHspiRxTogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - parallel if rx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_rx_num(&self) -> RbHspiRxNumR {
        RbHspiRxNumR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - parallel if rx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog(&self) -> RbHspiRxTogR {
        RbHspiRxTogR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - parallel if rx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_rx_num(&mut self) -> RbHspiRxNumW<'_, HspiRxScSpec> {
        RbHspiRxNumW::new(self, 0)
    }
    #[doc = "Bit 4 - parallel if rx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog(&mut self) -> RbHspiRxTogW<'_, HspiRxScSpec> {
        RbHspiRxTogW::new(self, 4)
    }
}
#[doc = "parallel RX sequence ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`hspi_rx_sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_rx_sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HspiRxScSpec;
impl crate::RegisterSpec for HspiRxScSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hspi_rx_sc::R`](R) reader structure"]
impl crate::Readable for HspiRxScSpec {}
#[doc = "`write(|w| ..)` method takes [`hspi_rx_sc::W`](W) writer structure"]
impl crate::Writable for HspiRxScSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSPI_RX_SC to value 0"]
impl crate::Resettable for HspiRxScSpec {}
