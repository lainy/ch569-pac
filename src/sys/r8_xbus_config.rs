#[doc = "Register `R8_XBUS_CONFIG` reader"]
pub type R = crate::R<R8XbusConfigSpec>;
#[doc = "Register `R8_XBUS_CONFIG` writer"]
pub type W = crate::W<R8XbusConfigSpec>;
#[doc = "Field `RB_XBUS_ENABLE` reader - external bus enable"]
pub type RbXbusEnableR = crate::BitReader;
#[doc = "Field `RB_XBUS_ENABLE` writer - external bus enable"]
pub type RbXbusEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_XBUS_ADDR_OE` reader - external bus address output enable"]
pub type RbXbusAddrOeR = crate::FieldReader;
#[doc = "Field `RB_XBUS_ADDR_OE` writer - external bus address output enable"]
pub type RbXbusAddrOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_XBUS_WIDTH` reader - external bus access pulse width"]
pub type RbXbusWidthR = crate::FieldReader;
#[doc = "Field `RB_XBUS_WIDTH` writer - external bus access pulse width"]
pub type RbXbusWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_XBUS_HOLD` reader - external bus hold time"]
pub type RbXbusHoldR = crate::BitReader;
#[doc = "Field `RB_XBUS_HOLD` writer - external bus hold time"]
pub type RbXbusHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_XBUS_SETUP` reader - external bus setup time"]
pub type RbXbusSetupR = crate::BitReader;
#[doc = "Field `RB_XBUS_SETUP` writer - external bus setup time"]
pub type RbXbusSetupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - external bus enable"]
    #[inline(always)]
    pub fn rb_xbus_enable(&self) -> RbXbusEnableR {
        RbXbusEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - external bus address output enable"]
    #[inline(always)]
    pub fn rb_xbus_addr_oe(&self) -> RbXbusAddrOeR {
        RbXbusAddrOeR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - external bus access pulse width"]
    #[inline(always)]
    pub fn rb_xbus_width(&self) -> RbXbusWidthR {
        RbXbusWidthR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - external bus hold time"]
    #[inline(always)]
    pub fn rb_xbus_hold(&self) -> RbXbusHoldR {
        RbXbusHoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - external bus setup time"]
    #[inline(always)]
    pub fn rb_xbus_setup(&self) -> RbXbusSetupR {
        RbXbusSetupR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - external bus enable"]
    #[inline(always)]
    pub fn rb_xbus_enable(&mut self) -> RbXbusEnableW<'_, R8XbusConfigSpec> {
        RbXbusEnableW::new(self, 0)
    }
    #[doc = "Bits 2:3 - external bus address output enable"]
    #[inline(always)]
    pub fn rb_xbus_addr_oe(&mut self) -> RbXbusAddrOeW<'_, R8XbusConfigSpec> {
        RbXbusAddrOeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - external bus access pulse width"]
    #[inline(always)]
    pub fn rb_xbus_width(&mut self) -> RbXbusWidthW<'_, R8XbusConfigSpec> {
        RbXbusWidthW::new(self, 4)
    }
    #[doc = "Bit 6 - external bus hold time"]
    #[inline(always)]
    pub fn rb_xbus_hold(&mut self) -> RbXbusHoldW<'_, R8XbusConfigSpec> {
        RbXbusHoldW::new(self, 6)
    }
    #[doc = "Bit 7 - external bus setup time"]
    #[inline(always)]
    pub fn rb_xbus_setup(&mut self) -> RbXbusSetupW<'_, R8XbusConfigSpec> {
        RbXbusSetupW::new(self, 7)
    }
}
#[doc = "external bus configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_xbus_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_xbus_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8XbusConfigSpec;
impl crate::RegisterSpec for R8XbusConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_xbus_config::R`](R) reader structure"]
impl crate::Readable for R8XbusConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_xbus_config::W`](W) writer structure"]
impl crate::Writable for R8XbusConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_XBUS_CONFIG to value 0"]
impl crate::Resettable for R8XbusConfigSpec {}
