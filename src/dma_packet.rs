use crate::dma_tag::{DmaTag, DmaTagId};
use binrw::{args, BinRead, BinWrite, BinResult, Endian};
use std::io::{Read, Seek};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, BinWrite, Deserialize)]
pub struct DmaPacket {
    pub dma_tags: Vec<DmaTag>,
}

impl BinRead for DmaPacket {
    type Args<'a> = (u32,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let (size,) = args;
        let mut dma_tags = Vec::new();

        let stream_start = reader.stream_position()?;

        while reader.stream_position()? < stream_start + size as u64 {
            dma_tags.push(DmaTag::read_options(reader, endian, ())?);

            match dma_tags[dma_tags.len() - 1].effective_dma_tag.id() {
                DmaTagId::Refe => {
                    let stream_pos = reader.stream_position()? as usize;
                    Vec::<u8>::read_options(
                        reader,
                        endian,
                        args! {count: stream_start as usize + size as usize - stream_pos},
                    )?;
                }
                DmaTagId::Ret => {
                    let stream_pos = reader.stream_position()? as usize;
                    Vec::<u8>::read_options(
                        reader,
                        endian,
                        args! {count: stream_start as usize + size as usize - stream_pos},
                    )?;
                }
                DmaTagId::End => {
                    let stream_pos = reader.stream_position()? as usize;
                    Vec::<u8>::read_options(
                        reader,
                        endian,
                        args! {count: stream_start as usize + size as usize - stream_pos},
                    )?;
                }
                _ => (),
            }
        }

        Ok(DmaPacket { dma_tags })
    }
}
