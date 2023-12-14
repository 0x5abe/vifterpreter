use crate::dma_tag::{DmaTag, DmaTagId};
use binrw::{args, parser, BinRead, BinResult};

#[derive(BinRead, Debug)]
pub struct DmaPacket {
    pub qword_count: u32,
    #[br(parse_with=parse_dma_tags, args(qword_count))]
    pub dma_tags: Vec<DmaTag>,
}

#[parser(reader, endian)]
fn parse_dma_tags(qword_count: u32) -> BinResult<Vec<DmaTag>> {
    let mut dma_tags = Vec::new();

    let stream_start = reader.stream_position()?;

    while reader.stream_position()? < stream_start + qword_count as u64 * 16 {
        dma_tags.push(DmaTag::read_options(reader, endian, ())?);

        match dma_tags[dma_tags.len() - 1].effective_dma_tag.id() {
            DmaTagId::Refe => {
                let stream_pos = reader.stream_position()? as usize;
                Vec::<u8>::read_options(
                    reader,
                    endian,
                    args! {count: stream_start as usize + qword_count as usize * 16 - stream_pos},
                )?;
            }
            DmaTagId::Ret => {
                let stream_pos = reader.stream_position()? as usize;
                Vec::<u8>::read_options(
                    reader,
                    endian,
                    args! {count: stream_start as usize + qword_count as usize * 16 - stream_pos},
                )?;
            }
            DmaTagId::End => {
                let stream_pos = reader.stream_position()? as usize;
                Vec::<u8>::read_options(
                    reader,
                    endian,
                    args! {count: stream_start as usize + qword_count as usize * 16 - stream_pos},
                )?;
            }
            _ => (),
        }
    }

    Ok(dma_tags)
}
