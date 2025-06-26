use defmt::{error, info};
use heapless::String;

use crate::{EdidBuffer, EDID_BUFFER_LEN};

// https://glenwing.github.io/docs/VESA-EEDID-A1.pdf

/// Parses EDID data and prints it to console.
pub fn read_edid(buf: EdidBuffer) {
    let mut correct_edid: EdidBuffer = [0; 256]; // data starting at header

    // Skip to start of header
    for i in 0..EDID_BUFFER_LEN {
        if buf[i] == 0
            && buf[i + 1] == 255
            && buf[i + 2] == 255
            && buf[i + 3] == 255
            && buf[i + 4] == 255
            && buf[i + 5] == 255
            && buf[i + 6] == 255
            && buf[i + 7] == 0
        {
            // signature found, copy into correct_edid
            for (place, data) in correct_edid.iter_mut().zip(buf[i..EDID_BUFFER_LEN].iter()) {
                *place = *data
            }
        }
    }

    if correct_edid[1] == 0 {
        error!("EDID signature not found");
        return;
    } else {
        info!("{:?}", correct_edid);
        info!("Valid signature found: {:?}", correct_edid[0..8]);
        info!(
            "Manufacturer ID: {:?} ({:x})",
            read_mfn(((correct_edid[8] as u16) << 8) + (correct_edid[9] as u16)).as_str(),
            ((correct_edid[8] as u16) << 8) + (correct_edid[9] as u16)
        );
        info!(
            "Product code: {:?} ({:x})",
            ((correct_edid[11] as u16) << 8) + (correct_edid[10] as u16),
            ((correct_edid[11] as u16) << 8) + (correct_edid[10] as u16)
        );
        info!(
            "Serial number: {:?} ({:x})",
            ((correct_edid[15] as u32) << 24)
                + ((correct_edid[14] as u32) << 16)
                + ((correct_edid[13] as u32) << 8)
                + (correct_edid[12] as u32),
            ((correct_edid[15] as u32) << 24)
                + ((correct_edid[14] as u32) << 16)
                + ((correct_edid[13] as u32) << 8)
                + (correct_edid[12] as u32)
        );
        info!(
            "Manufacture week: {:?} ({:x})",
            correct_edid[16], correct_edid[16]
        );
        info!(
            "Manufacture year: {:?} ({:x})",
            1990 + correct_edid[17] as u16,
            correct_edid[17]
        );
        info!(
            "EDID version: {:?}.{:?} ({:x})",
            correct_edid[18],
            correct_edid[19],
            ((correct_edid[18] as u16) << 8) + (correct_edid[19] as u16)
        );
        match ((correct_edid[18] as u16) << 8) + (correct_edid[19] as u16) {
            0x103 => read_103(&correct_edid),
            0x104 => read_104(&correct_edid),
            _ => error!("Unimplemented EDID version")
        }
    }
}

/// Reads the 3 5-bit letters making the manufacturer name
fn read_mfn(mf: u16) -> String<3> {
    let mut mfn = String::new();
    let _ = mfn.push((b'a' + ((mf >> 10) & 0b11111) as u8 - 1) as char);
    let _ = mfn.push((b'a' + ((mf >> 5) & 0b11111) as u8 - 1) as char);
    let _ = mfn.push((b'a' + (mf & 0b11111) as u8 - 1) as char);
    mfn
}

/// Reads past the header for EDID v1.3
fn read_103(buf: &EdidBuffer) {
    read_display_params_103(&buf);

    if buf[0x23] & 0b100000 == 0b100000 {
        info!("640x480p60 supported!");
    } else {
        info!("640x480p60 not supported");
    }
}

/// Reads past the header for EDID v1.4
fn read_104(buf: &EdidBuffer) {
    read_display_params_104(&buf);
}

fn read_display_params_103(buf: &EdidBuffer) {
    if buf[20] & 0b10000000 == 0b10000000 {
        // digital
        info!("Digital input");
        
        // bits 6-1 are reserved and should be 0
        
        if buf[20] & 0b1 == 0b1 {
            info!("Compatible with VESA DFP");
        } else {
            info!("Not compatible with VESA DFP");
        }

        // more digital/analog specific 
        match (buf[24] & 0b11000) >> 3 {
            0b00 => info!("RGB 4:4:4"),
            0b01 => info!("RGB 4:4:4 + YCrCb 4:4:4"),
            0b10 => info!("RGB 4:4:4 + YCrCb 4:2:2"),
            0b11 => info!("RGB 4:4:4 + YCrCb 4:4:4 + YCrCb 4:2:2 "),
            _ => info!("Invalid display type"),
        }
    } else {
        // analog (never)
        match buf[20] & 0b1100000 >> 5 {
            0b00 => info!("Video white and sync levels: +0.7/-0.3 V"),
            0b01 => info!("Video white and sync levels: +0.714/-0.286 V"),
            0b10 => info!("Video white and sync levels: +1.0/-0.4 V"),
            0b11 => info!("Video white and sync levels: +0.7/0 V (EVC)"),
            _ => info!("Invalid video white and sync levels"),
        }

        if buf[20] & 0b10000 == 0b10000 {
            info!("Blank-to-black setup expected")
        } else {
            info!("Blank-to-black setup not expected")
        }

        if buf[20] & 0b1000 == 0b1000 {
            info!("Separate sync supported")
        } else {
            info!("Separate sync not supported")
        }

        if buf[20] & 0b100 == 0b100 {
            info!("Composite sync on HSync supported")
        } else {
            info!("Composite sync on HSync not supported")
        }

        if buf[20] & 0b10 == 0b10 {
            info!("Sync on green supported")
        } else {
            info!("Sync on green not supported")
        }

        if buf[20] & 0b1 == 0b1 {
            info!("VSync pulse must be serrated when composite or sync-on-green is used")
        }

        match (buf[24] & 0b11000) >> 3 {
            0b00 => info!("monochrome or grayscale"),
            0b01 => info!("RGB color"),
            0b10 => info!("non-RGB color"),
            0b11 => info!("undefined "),
            _ => info!("Invalid display type"),
        }
    }

    let hori_size = buf[21];
    let vert_size = buf[22];

    if hori_size == 0 && vert_size == 0 {
        info!("Screen size/aspect ratio are undefined (projector?");
    } else if hori_size == 0 {
        info!(
            "Portrait aspect ratio: {:?}",
            100.0 / (vert_size as u16 + 99) as f32
        );
    } else if vert_size == 0 {
        info!(
            "Landscape aspect ratio: {:?}",
            (vert_size as u16 + 99) as f32 / 100.0
        );
    } else {
        info!("Display size: {:?}x{:?}cm", hori_size, vert_size);
    }

    let gamma = buf[23];

    if gamma == 255 {
        info!("Gamma information in DI-EXT");
    } else {
        info!("Default gamma: {:?}", (gamma as u16 + 100) as f32 / 100.0);
    }

    if buf[24] & 0b10000000 == 0b10000000 {
        info!("DPMS standby supported");
    } else {
        info!("DPMS standby not supported");
    }

    if buf[24] & 0b1000000 == 0b1000000 {
        info!("DPMS suspend supported");
    } else {
        info!("DPMS suspend not supported");
    }

    if buf[24] & 0b100000 == 0b100000 {
        info!("DPMS active-off supported");
    } else {
        info!("DPMS active-off not supported");
    }

    if buf[24] & 0b100 == 0b100 {
        info!("Using standard sRGB colour space");
    } else {
        info!("Not using standard sRGB colour space");
    }

    if buf[24] & 0b10 == 0b10 {
        if buf[19] == 3 {
            info!("Preferred timing mode in DTD1 includes native pixel format and refresh rate");
        } else {
            info!("Preferred timing mode is specified in DTD1");
        }
    } else {
        if buf[19] == 3 {
            info!("Preferred timing mode in DTD1 does not native pixel format and refresh rate");
        } else {
            info!("Preferred timing mode is not specified in DTD1");
        }
    }

    if buf[24] & 0b1 == 0b1 {
        info!("Continuous timings with GTF or CVT");
    } else {
        info!("No continuous timings with GTF or CVT");
    }
}

fn read_display_params_104(buf: &EdidBuffer) {
    if buf[20] & 0b10000000 == 0b10000000 {
        // digital
        info!("Digital input");
        match (buf[20] & 0b1110000) >> 4 {
            0b000 => info!("Undefined bit depth"),
            0b001 => info!("Using 6-bit colour"),
            0b011 => info!("Using 10-bit colour"),
            0b100 => info!("Using 12-bit colour"),
            0b101 => info!("Using 14-bit colour"),
            0b110 => info!("Using 16-bit colour"),
            0b111 => info!("reserved"),
            _ => info!("Invalid bit depth"),
        }

        match buf[20] & 0b1111 {
            0b0000 => info!("Undefined video interface"),
            0b0001 => info!("Communicating over DVI"),
            0b0010 => info!("Communicating over HDMIa"),
            0b0011 => info!("Communicating over HDMIb"),
            0b0100 => info!("Communicating over MDDI"),
            0b0101 => info!("Communicating over DisplayPort"),
            _ => info!("Invalid video interface {:?}", buf[20] & 0b1111),
        }

        match (buf[24] & 0b11000) >> 3 {
            0b00 => info!("RGB 4:4:4"),
            0b01 => info!("RGB 4:4:4 + YCrCb 4:4:4"),
            0b10 => info!("RGB 4:4:4 + YCrCb 4:2:2"),
            0b11 => info!("RGB 4:4:4 + YCrCb 4:4:4 + YCrCb 4:2:2 "),
            _ => info!("Invalid display type"),
        }
    } else {
        // analog (never)
        match buf[20] & 0b1100000 >> 5 {
            0b00 => info!("Video white and sync levels: +0.7/-0.3 V"),
            0b01 => info!("Video white and sync levels: +0.714/-0.286 V"),
            0b10 => info!("Video white and sync levels: +1.0/-0.4 V"),
            0b11 => info!("Video white and sync levels: +0.7/0 V (EVC)"),
            _ => info!("Invalid video white and sync levels"),
        }

        if buf[20] & 0b10000 == 0b10000 {
            info!("Blank-to-black setup expected")
        } else {
            info!("Blank-to-black setup not expected")
        }

        if buf[20] & 0b1000 == 0b1000 {
            info!("Separate sync supported")
        } else {
            info!("Separate sync not supported")
        }

        if buf[20] & 0b100 == 0b100 {
            info!("Composite sync on HSync supported")
        } else {
            info!("Composite sync on HSync not supported")
        }

        if buf[20] & 0b10 == 0b10 {
            info!("Sync on green supported")
        } else {
            info!("Sync on green not supported")
        }

        if buf[20] & 0b1 == 0b1 {
            info!("VSync pulse must be serrated when composite or sync-on-green is used")
        }

        match (buf[24] & 0b11000) >> 3 {
            0b00 => info!("monochrome or grayscale"),
            0b01 => info!("RGB color"),
            0b10 => info!("non-RGB color"),
            0b11 => info!("undefined "),
            _ => info!("Invalid display type"),
        }
    }

    let hori_size = buf[21];
    let vert_size = buf[22];

    if hori_size == 0 && vert_size == 0 {
        info!("Screen size/aspect ratio are undefined (projector?");
    } else if hori_size == 0 {
        info!(
            "Portrait aspect ratio: {:?}",
            100.0 / (vert_size as u16 + 99) as f32
        );
    } else if vert_size == 0 {
        info!(
            "Landscape aspect ratio: {:?}",
            (vert_size as u16 + 99) as f32 / 100.0
        );
    } else {
        info!("Display size: {:?}x{:?}cm", hori_size, vert_size);
    }

    let gamma = buf[23];

    if gamma == 255 {
        info!("Gamma information in DI-EXT");
    } else {
        info!("Default gamma: {:?}", (gamma as u16 + 100) as f32 / 100.0);
    }

    if buf[24] & 0b10000000 == 0b10000000 {
        info!("DPMS standby supported");
    } else {
        info!("DPMS standby not supported");
    }

    if buf[24] & 0b1000000 == 0b1000000 {
        info!("DPMS suspend supported");
    } else {
        info!("DPMS suspend not supported");
    }

    if buf[24] & 0b100000 == 0b100000 {
        info!("DPMS active-off supported");
    } else {
        info!("DPMS active-off not supported");
    }

    if buf[24] & 0b100 == 0b100 {
        info!("Using standard sRGB colour space");
    } else {
        info!("Not using standard sRGB colour space");
    }

    if buf[24] & 0b10 == 0b10 {
        if buf[19] == 3 {
            info!("Preferred timing mode in DTD1 includes native pixel format and refresh rate");
        } else {
            info!("Preferred timing mode is specified in DTD1");
        }
    } else {
        if buf[19] == 3 {
            info!("Preferred timing mode in DTD1 does not native pixel format and refresh rate");
        } else {
            info!("Preferred timing mode is not specified in DTD1");
        }
    }

    if buf[24] & 0b1 == 0b1 {
        info!("Continuous timings with GTF or CVT");
    } else {
        info!("No continuous timings with GTF or CVT");
    }
}
