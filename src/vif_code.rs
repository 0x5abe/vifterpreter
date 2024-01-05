use bilge::prelude::*;
use binrw::{args, BinRead, BinResult, Endian};
use std::io::{Read, Seek};
use strum::EnumDiscriminants;

#[derive(Debug, EnumDiscriminants)]
#[repr(u8)]
#[allow(non_camel_case_types)]
#[strum_discriminants(
    attributes(allow(non_camel_case_types), bitsize(8)),
    derive(TryFromBits),
    name(VifCodeCmd)
)]
pub enum VifCode {
    Nop = 0x0,
    Stcycl {
        cl: u8,
        wl: u8,
    } = 0x1,
    Stmod {
        mode: u2,
    } = 0x5,
    Flush = 0x11,
    Mscnt = 0x17,
    Strow {
        r0: u32,
        r1: u32,
        r2: u32,
        r3: u32,
    } = 0x30,
    Mpg {
        address: u16,
        num: usize,
        data: Vec<u8>,
    } = 0x4a,
    Unpack_S_32 {
        address: u10,
        zero_extension: bool,
        add_tops: bool,
        num: usize,
        data: Vec<u32>,
    } = 0x60,
    Unpack_V2_16 {
        address: u10,
        zero_extension: bool,
        add_tops: bool,
        num: usize,
        data: Vec<u16>,
    } = 0x65,
    Unpack_V4_32 {
        address: u10,
        zero_extension: bool,
        add_tops: bool,
        num: usize,
        data: Vec<u32>,
    } = 0x6c,
    Unpack_V4_16 {
        address: u10,
        zero_extension: bool,
        add_tops: bool,
        num: usize,
        data: Vec<u16>,
    } = 0x6d,
    Unpack_V4_8 {
        address: u10,
        zero_extension: bool,
        add_tops: bool,
        num: usize,
        data: Vec<u8>,
    } = 0x6e,
}

impl BinRead for VifCode {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        //println!("Stream pos: {}", reader.stream_position()?);
        let generic_vif_code = GenericVifCode::read_options(reader, endian, ())?;

        let vif_code: VifCode = match generic_vif_code.cmd() {
            VifCodeCmd::Nop => {
                //println!("Nop");

                VifCode::Nop
            }
            VifCodeCmd::Stcycl => {
                //println!("Stcycl");

                let cl = generic_vif_code.immediate() as u8;
                let wl = (generic_vif_code.immediate() >> 8) as u8;

                VifCode::Stcycl { cl, wl }
            }
            VifCodeCmd::Stmod => {
                //println!("Stmod");

                let mode = u2::new(generic_vif_code.immediate() as u8 & 0x3);

                VifCode::Stmod { mode }
            }
            VifCodeCmd::Flush => {
                //println!("Flush");
                VifCode::Flush
            }
            VifCodeCmd::Mscnt => {
                //println!("Mscnt");
                VifCode::Mscnt
            }
            VifCodeCmd::Strow => {
                //println!("Strow");

                let r0 = u32::read_options(reader, endian, ())?;
                let r1 = u32::read_options(reader, endian, ())?;
                let r2 = u32::read_options(reader, endian, ())?;
                let r3 = u32::read_options(reader, endian, ())?;

                VifCode::Strow { r0, r1, r2, r3 }
            }
            VifCodeCmd::Mpg => {
                //println!("Mpg");

                let num = if generic_vif_code.num() != 0 {
                    generic_vif_code.num() as usize * 8
                } else {
                    256 * 8
                };
                let address = generic_vif_code.immediate() * 8;
                let data = Vec::<u8>::read_options(reader, endian, args! {count: num})?;

                VifCode::Mpg { address, num, data }
            }
            VifCodeCmd::Unpack_S_32 => {
                //println!("Unpack S 32");

                let num = if generic_vif_code.num() != 0 {
                    generic_vif_code.num() as usize
                } else {
                    256
                };
                let address = u10::new(generic_vif_code.immediate() & 0x3ff);
                let zero_extension = generic_vif_code.immediate() & 0x4000 != 0;
                let add_tops = generic_vif_code.immediate() & 0x8000 != 0;
                let data = Vec::<u32>::read_options(reader, endian, args! {count: num})?;

                VifCode::Unpack_S_32 {
                    address,
                    zero_extension,
                    add_tops,
                    num,
                    data,
                }
            }
            VifCodeCmd::Unpack_V2_16 => {
                //println!("Unpack V2 16");

                let num = if generic_vif_code.num() != 0 {
                    generic_vif_code.num() as usize
                } else {
                    256
                };
                let address = u10::new(generic_vif_code.immediate() & 0x3ff);
                let zero_extension = generic_vif_code.immediate() & 0x4000 != 0;
                let add_tops = generic_vif_code.immediate() & 0x8000 != 0;
                let data = Vec::<u16>::read_options(reader, endian, args! {count: num * 2})?;

                VifCode::Unpack_V2_16 {
                    address,
                    zero_extension,
                    add_tops,
                    num,
                    data,
                }
            }
            VifCodeCmd::Unpack_V4_32 => {
                //println!("Unpack V4 32");

                let num = if generic_vif_code.num() != 0 {
                    generic_vif_code.num() as usize
                } else {
                    256
                };
                let address = u10::new(generic_vif_code.immediate() & 0x3ff);
                let zero_extension = generic_vif_code.immediate() & 0x4000 != 0;
                let add_tops = generic_vif_code.immediate() & 0x8000 != 0;
                let data = Vec::<u32>::read_options(reader, endian, args! {count: num * 4})?;

                VifCode::Unpack_V4_32 {
                    address,
                    zero_extension,
                    add_tops,
                    num,
                    data,
                }
            }
            VifCodeCmd::Unpack_V4_16 => {
                //println!("Unpack V4 16");

                let num = if generic_vif_code.num() != 0 {
                    generic_vif_code.num() as usize
                } else {
                    256
                };
                let address = u10::new(generic_vif_code.immediate() & 0x3ff);
                let zero_extension = generic_vif_code.immediate() & 0x4000 != 0;
                let add_tops = generic_vif_code.immediate() & 0x8000 != 0;
                let data = Vec::<u16>::read_options(reader, endian, args! {count: num * 4})?;

                VifCode::Unpack_V4_16 {
                    address,
                    zero_extension,
                    add_tops,
                    num,
                    data,
                }
            }
            VifCodeCmd::Unpack_V4_8 => {
                //println!("Unpack V4 8");

                let num = if generic_vif_code.num() != 0 {
                    generic_vif_code.num() as usize
                } else {
                    256
                };
                let address = u10::new(generic_vif_code.immediate() & 0x3ff);
                let zero_extension = generic_vif_code.immediate() & 0x4000 != 0;
                let add_tops = generic_vif_code.immediate() & 0x8000 != 0;
                let data = Vec::<u8>::read_options(reader, endian, args! {count: num * 4})?;

                VifCode::Unpack_V4_8 {
                    address,
                    zero_extension,
                    add_tops,
                    num,
                    data,
                }
            }
        };

        Ok(vif_code)
    }
}

#[bitsize(32)]
#[derive(BinRead)]
struct GenericVifCode {
    immediate: u16,
    num: u8,
    cmd: VifCodeCmd,
}
