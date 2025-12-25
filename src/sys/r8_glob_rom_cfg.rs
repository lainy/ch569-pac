#[doc = "Register `R8_GLOB_ROM_CFG` reader"]
pub type R = crate::R<R8GlobRomCfgSpec>;
#[doc = "Register `R8_GLOB_ROM_CFG` writer"]
pub type W = crate::W<R8GlobRomCfgSpec>;
#[doc = "Field `RB_ROM_EXT_RE` reader - enable flash ROM being read by external programmer"]
pub type RbRomExtReR = crate::BitReader;
#[doc = "Field `RB_CODE_RAM_WE` reader - enable code RAM being write"]
pub type RbCodeRamWeR = crate::BitReader;
#[doc = "Field `RB_CODE_RAM_WE` writer - enable code RAM being write"]
pub type RbCodeRamWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ROM_DATA_WE` reader - enable flash ROM data area being erase/write"]
pub type RbRomDataWeR = crate::BitReader;
#[doc = "Field `RB_ROM_DATA_WE` writer - enable flash ROM data area being erase/write"]
pub type RbRomDataWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ROM_CODE_WE` reader - enable flash ROM code_data area being erase/write"]
pub type RbRomCodeWeR = crate::BitReader;
#[doc = "Field `RB_ROM_CODE_WE` writer - enable flash ROM code_data area being erase/write"]
pub type RbRomCodeWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ROM_CODE_OFS` reader - Config the start offset address of user code in Flash"]
pub type RbRomCodeOfsR = crate::BitReader;
#[doc = "Field `RB_ROM_CODE_OFS` writer - Config the start offset address of user code in Flash"]
pub type RbRomCodeOfsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable flash ROM being read by external programmer"]
    #[inline(always)]
    pub fn rb_rom_ext_re(&self) -> RbRomExtReR {
        RbRomExtReR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable code RAM being write"]
    #[inline(always)]
    pub fn rb_code_ram_we(&self) -> RbCodeRamWeR {
        RbCodeRamWeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable flash ROM data area being erase/write"]
    #[inline(always)]
    pub fn rb_rom_data_we(&self) -> RbRomDataWeR {
        RbRomDataWeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable flash ROM code_data area being erase/write"]
    #[inline(always)]
    pub fn rb_rom_code_we(&self) -> RbRomCodeWeR {
        RbRomCodeWeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Config the start offset address of user code in Flash"]
    #[inline(always)]
    pub fn rb_rom_code_ofs(&self) -> RbRomCodeOfsR {
        RbRomCodeOfsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - enable code RAM being write"]
    #[inline(always)]
    pub fn rb_code_ram_we(&mut self) -> RbCodeRamWeW<'_, R8GlobRomCfgSpec> {
        RbCodeRamWeW::new(self, 1)
    }
    #[doc = "Bit 2 - enable flash ROM data area being erase/write"]
    #[inline(always)]
    pub fn rb_rom_data_we(&mut self) -> RbRomDataWeW<'_, R8GlobRomCfgSpec> {
        RbRomDataWeW::new(self, 2)
    }
    #[doc = "Bit 3 - enable flash ROM code_data area being erase/write"]
    #[inline(always)]
    pub fn rb_rom_code_we(&mut self) -> RbRomCodeWeW<'_, R8GlobRomCfgSpec> {
        RbRomCodeWeW::new(self, 3)
    }
    #[doc = "Bit 4 - Config the start offset address of user code in Flash"]
    #[inline(always)]
    pub fn rb_rom_code_ofs(&mut self) -> RbRomCodeOfsW<'_, R8GlobRomCfgSpec> {
        RbRomCodeOfsW::new(self, 4)
    }
}
#[doc = "flash ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_glob_rom_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_glob_rom_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8GlobRomCfgSpec;
impl crate::RegisterSpec for R8GlobRomCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_glob_rom_cfg::R`](R) reader structure"]
impl crate::Readable for R8GlobRomCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_glob_rom_cfg::W`](W) writer structure"]
impl crate::Writable for R8GlobRomCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_GLOB_ROM_CFG to value 0x80"]
impl crate::Resettable for R8GlobRomCfgSpec {
    const RESET_VALUE: u8 = 0x80;
}
