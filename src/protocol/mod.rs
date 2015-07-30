trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

// Although the "Guide to the Kafka Protocol" does not consider 'offset' and
// 'message size' to be part of a message, this distinction has no effect on
// their representation on the wire.
struct MessageSet {
    messages: Vec<Message>
}

struct Message {
    offset: i64,

    // On the wire, message_size should be the number of bytes in the the
    // "MessageAndOffset" that follows.
    //
    // message_size: i32,

    crc: i32,
    magic_byte: i8,
    attributes: MessageAttributes,
    key: Vec<u8>,

    // This could recursively represent more message sets
    value: Vec<u8>
}

struct MessageAttributes {
    compression_codec: CompressionCodec
}

enum CompressionCodec {
    NoCompression,
    GZip,
    Snappy
}

pub mod request;
pub mod response;
