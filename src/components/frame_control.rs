use crate::frame_types::*;

#[inline]
/// Mini helper to check, whether a bit is set or not.
fn flag_is_set(data: u8, bit: u8) -> bool {
    if bit == 0 {
        let mask = 1;
        (data & mask) > 0
    } else {
        let mask = 1 << bit;
        (data & mask) > 0
    }
}

/// The very first two bytes of every frame contain the FrameControl header.
/// https://en.wikipedia.org/wiki/802.11_Frame_Types
///
/// The bytes are structured as follows.
/// The bytes will be interpreted from left to right.
///
/// First byte
/// The first two bits specify the protocol version
/// Until now, this has always been 0 and is expected to be 0
/// bit 0-1: Version
/// bit 2-3: FrameType
/// bit 4-7: FrameSubType
///
/// 8 Flags (second byte)
/// bit_0 `to_ds`: Set if the frame is to be sent by the AP to the distribution system.
/// bit_1 `from_ds`: Set if the frame is from the distribution system.
/// bit_2 `more_frag`: Set if this frame is a fragment of a bigger frame and there are more fragments to follow.
/// bit_3 `retry`: Set if this frame is a retransmission, maybe through the loss of an ACK.
/// bit_4 `power_mgmt`: indicates what power mode ('save' or 'active') the station is to be in once the frame has been sent.
/// bit_5 `more_data`: set by the AP to indicate that more frames are destined to a particular station that may be in power save mode.
///                     These frames will be buffered at the AP ready for the station should it decide to become 'active'.
/// bit_6 `wep`: Set if WEP is being used to encrypt the body of the frame
/// bit_7 `order`: Set if the frame is being sent according to the 'Strictly Ordered Class'
#[derive(Clone, Debug)]
pub struct FrameControl {
    pub protocol_version: u8,
    pub frame_type: FrameType,
    pub frame_subtype: FrameSubType,
    pub flags: u8,
}

impl FrameControl {
    pub fn to_ds(&self) -> bool {
        flag_is_set(self.flags, 0)
    }

    pub fn from_ds(&self) -> bool {
        flag_is_set(self.flags, 1)
    }

    pub fn more_frag(&self) -> bool {
        flag_is_set(self.flags, 2)
    }

    pub fn retry(&self) -> bool {
        flag_is_set(self.flags, 3)
    }

    pub fn pwr_mgmt(&self) -> bool {
        flag_is_set(self.flags, 4)
    }

    pub fn more_data(&self) -> bool {
        flag_is_set(self.flags, 5)
    }

    pub fn wep(&self) -> bool {
        flag_is_set(self.flags, 6)
    }

    pub fn order(&self) -> bool {
        flag_is_set(self.flags, 7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::parse_frame_control;

    fn flag_for_bit(bit: u8, frame_control: &FrameControl) -> bool {
        match bit {
            0 => frame_control.to_ds(),
            1 => frame_control.from_ds(),
            2 => frame_control.more_frag(),
            3 => frame_control.retry(),
            4 => frame_control.pwr_mgmt(),
            5 => frame_control.more_data(),
            6 => frame_control.wep(),
            7 => frame_control.order(),
            _ => panic!("Unhandled bit {}", bit),
        }
    }

    #[test]
    /// Set each flag once and ensure that only that bit is set.
    /// For this, we shift a byte with value `1` up to seven times to the left.
    fn test_flags() {
        for bit in 0..7 {
            let second_byte = 0b0000_0001 << bit;
            let bytes = [0b0000_0000, second_byte];
            let frame_control = parse_frame_control(&bytes).unwrap().1;

            // All bits except the currently selected bit should be false.
            for check_bit in 0..7 {
                if bit == check_bit {
                    assert!(flag_for_bit(check_bit, &frame_control));
                } else {
                    assert!(!flag_for_bit(check_bit, &frame_control));
                }
            }
        }
    }

    #[test]
    /// Create a Management-Beacon FrameControl header
    /// FrameType should be `00` and SubType `1000`
    /// Remember
    fn test_beacon() {
        let bytes = [0b1000_0000, 0b0000_0000];
        let frame_control = parse_frame_control(&bytes).unwrap().1;

        assert!(matches!(frame_control.frame_type, FrameType::Management));
        assert!(matches!(frame_control.frame_subtype, FrameSubType::Beacon));
    }
}
