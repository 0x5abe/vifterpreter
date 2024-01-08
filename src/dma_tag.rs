use crate::vif_code::VifCode;
use bilge::prelude::*;
use binrw::{BinRead, BinResult, BinWrite};
use serde::{Deserialize, Serialize};

#[bitsize(2)]
#[derive(FromBits, Debug, Serialize, Deserialize)]
pub enum PriorityControl {
    NoEffect = 0,
    Reserved = 1,
    Disabled = 2,
    Enabled = 3,
}

#[bitsize(3)]
#[derive(FromBits, Debug, Serialize, Deserialize)]
pub enum DmaTagId {
    Refe = 0,
    Cnt = 1,
    Next = 2,
    Ref = 3,
    Refs = 4,
    Call = 5,
    Ret = 6,
    End = 7,
}

#[bitsize(64)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits)]
pub struct EffectiveDmaTag {
    pub qword_count: u16,
    padding: u10,
    priority_control: PriorityControl,
    pub id: DmaTagId,
    irq: u1,
    address: u32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct DmaTag {
    pub effective_dma_tag: EffectiveDmaTag,
    #[br(parse_with=parse_vif_codes, args(&effective_dma_tag))]
    #[bw(write_with=write_vif_codes, args(&effective_dma_tag))]
    pub vif_codes: Vec<VifCode>,
}

#[binrw::parser(reader, endian)]
fn parse_vif_codes(effective_dma_tag: &EffectiveDmaTag) -> BinResult<Vec<VifCode>> {
    let mut vif_codes = Vec::new();

    let stream_start = reader.stream_position()?;

    while reader.stream_position()? < stream_start + 8 + effective_dma_tag.qword_count() as u64 * 16
    {
        vif_codes.push(VifCode::read_options(reader, endian, ())?);
    }

    Ok(vif_codes)
}

#[binrw::writer(writer, endian)]
#[allow(unused_variables)]
fn write_vif_codes(data: &Vec<VifCode>, effective_dma_tag: &EffectiveDmaTag) -> BinResult<()> {
    todo!()
}
