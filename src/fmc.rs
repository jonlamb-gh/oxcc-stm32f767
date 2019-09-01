#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: BCR2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub btr2: BTR2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bcr3: BCR3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub btr3: BTR3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bcr4: BCR4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub btr4: BTR4,
    _reserved8: [u8; 96usize],
    #[doc = "0x80 - PC Card/NAND Flash control register"]
    pub pcr: PCR,
    #[doc = "0x84 - FIFO status and interrupt register"]
    pub sr: SR,
    #[doc = "0x88 - Common memory space timing register"]
    pub pmem: PMEM,
    #[doc = "0x8c - Attribute memory space timing register"]
    pub patt: PATT,
    _reserved12: [u8; 4usize],
    #[doc = "0x94 - ECC result register"]
    pub eccr: ECCR,
    _reserved13: [u8; 108usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR1,
    _reserved14: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bwtr2: BWTR2,
    _reserved15: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bwtr3: BWTR3,
    _reserved16: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bwtr4: BWTR4,
    _reserved17: [u8; 32usize],
    #[doc = "0x140 - SDRAM Control Register 1"]
    pub sdcr1: SDCR1,
    #[doc = "0x144 - SDRAM Control Register 2"]
    pub sdcr2: SDCR2,
    #[doc = "0x148 - SDRAM Timing register 1"]
    pub sdtr1: SDTR1,
    #[doc = "0x14c - SDRAM Timing register 2"]
    pub sdtr2: SDTR2,
    #[doc = "0x150 - SDRAM Command Mode register"]
    pub sdcmr: SDCMR,
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    pub sdrtr: SDRTR,
    #[doc = "0x158 - SDRAM Status register"]
    pub sdsr: SDSR,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bcr1](bcr1) module"]
pub type BCR1 = crate::Reg<u32, _BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR1;
#[doc = "`read()` method returns [bcr1::R](bcr1::R) reader structure"]
impl crate::Readable for BCR1 {}
#[doc = "`write(|w| ..)` method takes [bcr1::W](bcr1::W) writer structure"]
impl crate::Writable for BCR1 {}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "SRAM/NOR-Flash chip-select timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [btr1](btr1) module"]
pub type BTR1 = crate::Reg<u32, _BTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR1;
#[doc = "`read()` method returns [btr1::R](btr1::R) reader structure"]
impl crate::Readable for BTR1 {}
#[doc = "`write(|w| ..)` method takes [btr1::W](btr1::W) writer structure"]
impl crate::Writable for BTR1 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "SRAM/NOR-Flash chip-select control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bcr2](bcr2) module"]
pub type BCR2 = crate::Reg<u32, _BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR2;
#[doc = "`read()` method returns [bcr2::R](bcr2::R) reader structure"]
impl crate::Readable for BCR2 {}
#[doc = "`write(|w| ..)` method takes [bcr2::W](bcr2::W) writer structure"]
impl crate::Writable for BCR2 {}
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "SRAM/NOR-Flash chip-select timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [btr2](btr2) module"]
pub type BTR2 = crate::Reg<u32, _BTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR2;
#[doc = "`read()` method returns [btr2::R](btr2::R) reader structure"]
impl crate::Readable for BTR2 {}
#[doc = "`write(|w| ..)` method takes [btr2::W](btr2::W) writer structure"]
impl crate::Writable for BTR2 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "SRAM/NOR-Flash chip-select control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bcr3](bcr3) module"]
pub type BCR3 = crate::Reg<u32, _BCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR3;
#[doc = "`read()` method returns [bcr3::R](bcr3::R) reader structure"]
impl crate::Readable for BCR3 {}
#[doc = "`write(|w| ..)` method takes [bcr3::W](bcr3::W) writer structure"]
impl crate::Writable for BCR3 {}
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "SRAM/NOR-Flash chip-select timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [btr3](btr3) module"]
pub type BTR3 = crate::Reg<u32, _BTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR3;
#[doc = "`read()` method returns [btr3::R](btr3::R) reader structure"]
impl crate::Readable for BTR3 {}
#[doc = "`write(|w| ..)` method takes [btr3::W](btr3::W) writer structure"]
impl crate::Writable for BTR3 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "SRAM/NOR-Flash chip-select control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bcr4](bcr4) module"]
pub type BCR4 = crate::Reg<u32, _BCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR4;
#[doc = "`read()` method returns [bcr4::R](bcr4::R) reader structure"]
impl crate::Readable for BCR4 {}
#[doc = "`write(|w| ..)` method takes [bcr4::W](bcr4::W) writer structure"]
impl crate::Writable for BCR4 {}
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "SRAM/NOR-Flash chip-select timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [btr4](btr4) module"]
pub type BTR4 = crate::Reg<u32, _BTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR4;
#[doc = "`read()` method returns [btr4::R](btr4::R) reader structure"]
impl crate::Readable for BTR4 {}
#[doc = "`write(|w| ..)` method takes [btr4::W](btr4::W) writer structure"]
impl crate::Writable for BTR4 {}
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PC Card/NAND Flash control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "PC Card/NAND Flash control register"]
pub mod pcr;
#[doc = "FIFO status and interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "FIFO status and interrupt register"]
pub mod sr;
#[doc = "Common memory space timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmem](pmem) module"]
pub type PMEM = crate::Reg<u32, _PMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM;
#[doc = "`read()` method returns [pmem::R](pmem::R) reader structure"]
impl crate::Readable for PMEM {}
#[doc = "`write(|w| ..)` method takes [pmem::W](pmem::W) writer structure"]
impl crate::Writable for PMEM {}
#[doc = "Common memory space timing register"]
pub mod pmem;
#[doc = "Attribute memory space timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [patt](patt) module"]
pub type PATT = crate::Reg<u32, _PATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT;
#[doc = "`read()` method returns [patt::R](patt::R) reader structure"]
impl crate::Readable for PATT {}
#[doc = "`write(|w| ..)` method takes [patt::W](patt::W) writer structure"]
impl crate::Writable for PATT {}
#[doc = "Attribute memory space timing register"]
pub mod patt;
#[doc = "ECC result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eccr](eccr) module"]
pub type ECCR = crate::Reg<u32, _ECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR;
#[doc = "`read()` method returns [eccr::R](eccr::R) reader structure"]
impl crate::Readable for ECCR {}
#[doc = "ECC result register"]
pub mod eccr;
#[doc = "SRAM/NOR-Flash write timing registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bwtr1](bwtr1) module"]
pub type BWTR1 = crate::Reg<u32, _BWTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR1;
#[doc = "`read()` method returns [bwtr1::R](bwtr1::R) reader structure"]
impl crate::Readable for BWTR1 {}
#[doc = "`write(|w| ..)` method takes [bwtr1::W](bwtr1::W) writer structure"]
impl crate::Writable for BWTR1 {}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "SRAM/NOR-Flash write timing registers 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bwtr2](bwtr2) module"]
pub type BWTR2 = crate::Reg<u32, _BWTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR2;
#[doc = "`read()` method returns [bwtr2::R](bwtr2::R) reader structure"]
impl crate::Readable for BWTR2 {}
#[doc = "`write(|w| ..)` method takes [bwtr2::W](bwtr2::W) writer structure"]
impl crate::Writable for BWTR2 {}
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "SRAM/NOR-Flash write timing registers 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bwtr3](bwtr3) module"]
pub type BWTR3 = crate::Reg<u32, _BWTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR3;
#[doc = "`read()` method returns [bwtr3::R](bwtr3::R) reader structure"]
impl crate::Readable for BWTR3 {}
#[doc = "`write(|w| ..)` method takes [bwtr3::W](bwtr3::W) writer structure"]
impl crate::Writable for BWTR3 {}
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "SRAM/NOR-Flash write timing registers 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bwtr4](bwtr4) module"]
pub type BWTR4 = crate::Reg<u32, _BWTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR4;
#[doc = "`read()` method returns [bwtr4::R](bwtr4::R) reader structure"]
impl crate::Readable for BWTR4 {}
#[doc = "`write(|w| ..)` method takes [bwtr4::W](bwtr4::W) writer structure"]
impl crate::Writable for BWTR4 {}
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
#[doc = "SDRAM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdcr1](sdcr1) module"]
pub type SDCR1 = crate::Reg<u32, _SDCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCR1;
#[doc = "`read()` method returns [sdcr1::R](sdcr1::R) reader structure"]
impl crate::Readable for SDCR1 {}
#[doc = "`write(|w| ..)` method takes [sdcr1::W](sdcr1::W) writer structure"]
impl crate::Writable for SDCR1 {}
#[doc = "SDRAM Control Register 1"]
pub mod sdcr1;
#[doc = "SDRAM Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdcr2](sdcr2) module"]
pub type SDCR2 = crate::Reg<u32, _SDCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCR2;
#[doc = "`read()` method returns [sdcr2::R](sdcr2::R) reader structure"]
impl crate::Readable for SDCR2 {}
#[doc = "`write(|w| ..)` method takes [sdcr2::W](sdcr2::W) writer structure"]
impl crate::Writable for SDCR2 {}
#[doc = "SDRAM Control Register 2"]
pub mod sdcr2;
#[doc = "SDRAM Timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdtr1](sdtr1) module"]
pub type SDTR1 = crate::Reg<u32, _SDTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDTR1;
#[doc = "`read()` method returns [sdtr1::R](sdtr1::R) reader structure"]
impl crate::Readable for SDTR1 {}
#[doc = "`write(|w| ..)` method takes [sdtr1::W](sdtr1::W) writer structure"]
impl crate::Writable for SDTR1 {}
#[doc = "SDRAM Timing register 1"]
pub mod sdtr1;
#[doc = "SDRAM Timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdtr2](sdtr2) module"]
pub type SDTR2 = crate::Reg<u32, _SDTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDTR2;
#[doc = "`read()` method returns [sdtr2::R](sdtr2::R) reader structure"]
impl crate::Readable for SDTR2 {}
#[doc = "`write(|w| ..)` method takes [sdtr2::W](sdtr2::W) writer structure"]
impl crate::Writable for SDTR2 {}
#[doc = "SDRAM Timing register 2"]
pub mod sdtr2;
#[doc = "SDRAM Command Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdcmr](sdcmr) module"]
pub type SDCMR = crate::Reg<u32, _SDCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCMR;
#[doc = "`read()` method returns [sdcmr::R](sdcmr::R) reader structure"]
impl crate::Readable for SDCMR {}
#[doc = "`write(|w| ..)` method takes [sdcmr::W](sdcmr::W) writer structure"]
impl crate::Writable for SDCMR {}
#[doc = "SDRAM Command Mode register"]
pub mod sdcmr;
#[doc = "SDRAM Refresh Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdrtr](sdrtr) module"]
pub type SDRTR = crate::Reg<u32, _SDRTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRTR;
#[doc = "`read()` method returns [sdrtr::R](sdrtr::R) reader structure"]
impl crate::Readable for SDRTR {}
#[doc = "`write(|w| ..)` method takes [sdrtr::W](sdrtr::W) writer structure"]
impl crate::Writable for SDRTR {}
#[doc = "SDRAM Refresh Timer register"]
pub mod sdrtr;
#[doc = "SDRAM Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdsr](sdsr) module"]
pub type SDSR = crate::Reg<u32, _SDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDSR;
#[doc = "`read()` method returns [sdsr::R](sdsr::R) reader structure"]
impl crate::Readable for SDSR {}
#[doc = "SDRAM Status register"]
pub mod sdsr;
