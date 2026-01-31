use bytes::{Buf, BufMut, Bytes, BytesMut};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::{fmt::Debug, io};
use tdf::{
    DecodeResult, TdfDeserialize, TdfDeserializer, TdfSerialize, types::bytes::serialize_bytes,
};
use tokio_util::codec::{Decoder, Encoder};

#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone)]
#[repr(u8)]
pub enum MessageType {
    Message,
    Reply,
    Notification,
    ErrorReply,
    Ping,
    PingReply,
}

#[derive(Debug, Clone, Copy)]
pub struct Fire2Frame {
    pub component: u16,
    pub command: u16,
    pub msg_num: u32,
    pub msg_type: MessageType,
}

impl Fire2Frame {
    fn read_msg_num(src: &mut BytesMut) -> u32 {
        let b0 = src.get_u8() as u32;
        let b1 = src.get_u8() as u32;
        let b2 = src.get_u8() as u32;
        (b0 << 16) | (b1 << 8) | b2
    }

    fn write_msg_num(dst: &mut BytesMut, msg_num: u32) {
        let msg_num = msg_num & 0x00FFFFFF;
        dst.put_u8((msg_num >> 16) as u8);
        dst.put_u8((msg_num >> 8) as u8);
        dst.put_u8(msg_num as u8);
    }

    pub const fn notification(component: u16, command: u16) -> Self {
        Self {
            component,
            command,
            msg_num: 0,
            msg_type: MessageType::Notification,
        }
    }

    pub const fn reply(&self) -> Self {
        Self {
            msg_type: MessageType::Reply,
            ..*self
        }
    }
}

#[derive(Clone)]
pub struct Packet {
    pub frame: Fire2Frame,
    pub metadata: Bytes,
    pub contents: Bytes,
}

impl Packet {
    const HEADER_SIZE: usize = 16;

    pub const fn new(frame: Fire2Frame, metadata: Bytes, contents: Bytes) -> Self {
        Self {
            frame,
            metadata,
            contents,
        }
    }

    pub const fn empty(frame: Fire2Frame) -> Self {
        Self::new(frame, Bytes::new(), Bytes::new())
    }

    pub fn reply<T: TdfSerialize>(request: &Packet, body: T) -> Self {
        Self::new(request.frame.reply(), Bytes::new(), serialize_bytes(&body))
    }

    pub fn reply_empty(request: &Packet) -> Self {
        Self::empty(request.frame.reply())
    }

    pub fn notification<T: TdfSerialize>(component: u16, command: u16, body: T) -> Self {
        Self::new(
            Fire2Frame::notification(component, command),
            Bytes::new(),
            serialize_bytes(&body),
        )
    }

    pub fn deserialize<'de, T: TdfDeserialize<'de>>(&'de self) -> DecodeResult<T> {
        T::deserialize(&mut TdfDeserializer::new(&self.contents))
    }

    pub fn read(src: &mut BytesMut) -> Option<Self> {
        if src.len() < Self::HEADER_SIZE {
            return None;
        }

        let payload_size = u32::from_be_bytes([src[0], src[1], src[2], src[3]]) as usize;
        let metadata_size = u16::from_be_bytes([src[4], src[5]]) as usize;
        let total_size = Self::HEADER_SIZE + metadata_size + payload_size;

        if src.len() < total_size {
            return None;
        }

        src.advance(4);
        src.advance(2);

        let component = src.get_u16();
        let command = src.get_u16();

        let msg_num = Fire2Frame::read_msg_num(src);

        // In the original Blaze implementation this byte is split
        // into `message type` and `user index`, but in Pamplona
        // the user index stays always a zero
        let ty = MessageType::from_u8(src.get_u8() >> 5)?;

        // Skip options and reserved byte, never used and always zero
        let _options = src.get_u8();
        let _reserved = src.get_u8();

        let metadata = src.split_to(metadata_size).freeze();
        let contents = src.split_to(payload_size).freeze();

        Some(Self {
            frame: Fire2Frame {
                component,
                command,
                msg_num,
                msg_type: ty,
            },
            metadata,
            contents,
        })
    }

    pub fn write(&self, dst: &mut BytesMut) {
        dst.put_u32(self.contents.len() as u32);
        dst.put_u16(self.metadata.len() as u16);

        dst.put_u16(self.frame.component);
        dst.put_u16(self.frame.command);

        Fire2Frame::write_msg_num(dst, self.frame.msg_num);
        dst.put_u8((self.frame.msg_type as u8) << 5);

        dst.put_u8(0);
        dst.put_u8(0);

        dst.extend_from_slice(&self.metadata);
        dst.extend_from_slice(&self.contents);
    }
}

pub struct PacketCodec;

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Packet::read(src))
    }
}

impl Encoder<Packet> for PacketCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write(dst);
        Ok(())
    }
}
