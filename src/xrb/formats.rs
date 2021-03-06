/**
 * Every request contains an 8-bit major opcode and a 16-bit length field expressed in units of four bytes.
 * Every request consists of four bytes of a header (containing the major opcode, the length field, and a data byte) followed by zero or more additional bytes of data.
 * The length field defines the total length of the request, including the header.
 * The length field in a request must equal the minimum length required to contain the request.
 * If the specified length is smaller or larger than the required length, an error is generated.
 * Unused bytes in a request are not required to be zero.
 * Major opcodes 128 through 255 are reserved for extensions.
 * Extensions are intended to contain multiple requests, so extension requests typically have an additional minor opcode encoded in the second data byte in the request header.
 * However, the placement and interpretation of this minor opcode and of all other fields in extension requests are not defined by the core protocol.
 * Every request on a given connection is implicitly assigned a sequence number, starting with one, that is used in replies, errors, and events.
 */
pub(crate) struct RawRequest {
    major_opcode: u8,
    length: u16,
    data: Vec<u8>,
}
/* Every reply contains a 32-bit length field expressed in units of four bytes.
 * Every reply consists of 32 bytes followed by zero or more additional bytes of data, as specified in the length field.
 * Unused bytes within a reply are not guaranteed to be zero.
 * Every reply also contains the least significant 16 bits of the sequence number of the corresponding request.
 */
pub struct RawReply {
    length: u32,
    data: Vec<u8>,
    sequence_number: u16,
}

/* Error reports are 32 bytes long.
 * Every error includes an 8-bit error code.
 * Error codes 128 through 255 are reserved for extensions.
 * Every error also includes the major and minor opcodes of the failed request and the least significant 16 bits of the sequence number of the request.
 * For the following errors (see section 4), the failing resource ID is also returned: Colormap, Cursor, Drawable, Font, GContext, IDChoice, Pixmap and Window.
 * For Atom errors, the failing atom is returned.
 * For Value errors, the failing value is returned.
 * Other core errors return no additional data.
 * Unused bytes within an error are not guaranteed to be zero.
 */
pub struct RawError {
    error_code: u8,
    major_opcode: u8,
    minor_opcode: u8,
    sequence_number: u16,
}

/* Events are 32 bytes long.
 * Unused bytes within an event are not guaranteed to be zero.
 * Every event contains an 8-bit type code.
 * The most significant bit in this code is set if the event was generated from a SendEvent request.
 * Event codes 64 through 127 are reserved for extensions, although the core protocol does not define a mechanism for selecting interest in such events.
 * Every core event (with the exception of KeymapNotify) also contains the least significant 16 bits of the sequence number of the last request issued by the client that was (or is currently being) processed by the server.
 */
pub struct RawEvent {
    type_code: u8,
    sequence_number: u16,
}

