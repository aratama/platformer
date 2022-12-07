// misaki_gothic_2nd
use crate::wasm4::*;
use crate::image::Image;

const MISAKI_GOTHIC_2ND_WIDTH: u32 = 688;
const MISAKI_GOTHIC_2ND_HEIGHT: u32 = 16;
const MISAKI_GOTHIC_2ND_FLAGS: u32 = BLIT_1BPP;
const MISAKI_GOTHIC_2ND: [u8; 1376] = [ 0xff,0xdf,0xff,0xff,0xff,0x83,0xff,0x83,0xff,0xdf,0xdf,0xd5,0xef,0xeb,0xfb,0xfb,0x77,0x75,0xff,0xf5,0xf7,0xf5,0xbf,0xab,0xf7,0xf5,0xbb,0xb5,0x83,0x85,0xdf,0xd5,0xdf,0xd5,0xff,0xff,0xf5,0xff,0xff,0xdf,0xd5,0xdf,0x7f,0xb7,0xdf,0xff,0x77,0x75,0x73,0x17,0x15,0x1b,0xcf,0xcf,0xcb,0xff,0xf5,0xfb,0x41,0x45,0x4b,0xef,0x8f,0xdf,0xb7,0xef,0xff,0xb7,0xff,0xef,0xff,0xef,0x8f,0xa7,0x83,0xdf,0x83,0xff,0xdf,0x8f,0x83,0xef,0xef,0xff,0xff,0xff,0xdf,0x83,0xff,0x7b,0x87,0xff,0x87,0xff,0xdf,0x09,0x0b,0x0f,0x83,0x83,0xe7,0xe7,0x41,0x41,0x83,0x83,0x01,0x01,0xbf,0xbf,0x01,0x01,0xbb,0xbb,0xef,0xef,0x0f,0x0f,0x01,0x01,0xff,0xc3,0xc3,0x01,0x01,0xdf,0xdf,0x09,0x63,0xa3,0x13,0x83,0x41,0x41,0x45,0xdb,0xdb,0xd5,0xf7,0xf5,0xf5,0xdf,0xdf,0xd5,0x77,0x77,0x75,0x01,0xef,0x09,0xa3,0x03,0xaf,0xa3,0xdf,0x43,0xef,0xe3,0xf7,0x9b,0xef,0x13,0xef,0xbf,0x13,0xef,0xef,0x03,0xef,0xff,0xff,0xff,0x87,0xdf,0x77,0x7d,0xff,0x83,0xff,0x03,0x0b,0xdf,0xd5,0xd5,0xf7,0xf7,0x9f,0x95,0x77,0x77,0xff,0xff,0xf7,0xf7,0xbf,0xbf,0xd7,0xd7,0x01,0x01,0xdf,0xdf,0xd1,0xd1,0xdf,0xdf,0xc7,0x3d,0x3d,0xef,0xef,0xd3,0xd3,0xdb,0x7f,0x95,0xcd,0x6d,0x77,0x77,0x73,0xb9,0xb9,0xb9,0xef,0xef,0xeb,0xaf,0xaf,0xab,0x41,0x41,0x43,0xef,0xdb,0xdf,0x95,0xdf,0xa7,0x15,0x47,0x35,0xe7,0xef,0xbf,0xbb,0xc3,0xcb,0xdf,0x27,0xcd,0xc3,0xc7,0xdf,0xdf,0xff,0xff,0xff,0xdf,0x83,0x7b,0x7d,0x87,0x7d,0x07,0xef,0xdf,0xc3,0xb5,0xb5,0x01,0x01,0x7f,0x7f,0x77,0x77,0xff,0xff,0x8b,0x8b,0xbf,0xbf,0xb7,0xb7,0xbb,0xbb,0x01,0x01,0xbf,0xbf,0xa3,0xa3,0x3b,0xfd,0xfd,0xdf,0xd5,0xcf,0xcf,0xbb,0x7f,0x55,0xdd,0x6d,0x77,0x77,0x77,0xbb,0xbb,0xbb,0xef,0xef,0xef,0xb7,0xb7,0xb7,0x77,0x77,0x77,0x01,0x81,0x9f,0x55,0x03,0x1b,0xdd,0x2b,0x75,0xef,0xef,0x43,0xbb,0xbd,0xdb,0x83,0x9b,0xdd,0xad,0xbb,0x89,0xcf,0xff,0xff,0xff,0x87,0x55,0x7b,0x7d,0x7b,0xfd,0xdf,0xc7,0x87,0x9d,0xb7,0xb7,0xdb,0xdb,0x9f,0x9f,0x77,0x77,0xbf,0xbf,0x73,0x73,0xbd,0xbd,0xc7,0xc7,0xb3,0xb3,0xef,0xef,0xb7,0xb7,0x9d,0x9d,0xfb,0xfd,0xfd,0xdf,0xdf,0xbf,0xbf,0x63,0x6f,0x49,0x99,0x5d,0x47,0x47,0x47,0xbb,0xbb,0xbb,0xb3,0xb3,0xb3,0x7b,0x7b,0x7b,0x47,0x47,0x47,0xaf,0x5b,0x5d,0x4d,0xdd,0xd7,0xdb,0x6b,0x55,0x8f,0x87,0x3d,0xfb,0x4d,0x9b,0x7d,0xbb,0x9d,0x69,0xe7,0x67,0xb5,0xff,0xff,0xff,0x53,0x4d,0x5f,0x5f,0xf7,0xf3,0x8f,0xb7,0x5b,0x5d,0x77,0x77,0xbf,0xbf,0xe7,0xe7,0x77,0x77,0x7f,0x7f,0xbf,0xbf,0xbb,0xbb,0xf7,0xf7,0xbf,0xbf,0xdf,0xdf,0x6f,0x6f,0xfb,0xfb,0xf7,0xfb,0xfb,0xdf,0xdf,0x7f,0x7f,0x59,0x5f,0x55,0x55,0x3d,0x33,0x33,0x33,0xbb,0xbb,0xbb,0xb5,0xb5,0xb5,0xfd,0xfd,0xfd,0x33,0x33,0x33,0x67,0x5b,0x1d,0x5d,0xdd,0xdf,0xef,0x47,0x63,0x67,0x6b,0xfb,0xf7,0xb5,0x5b,0xfd,0x3b,0x5d,0x55,0x93,0xd7,0xb5,0xff,0xff,0xff,0x8b,0x9b,0xbf,0xbf,0xcf,0xcf,0x63,0x71,0x97,0x93,0x4f,0x4f,0xc3,0xc3,0xfb,0xfb,0xef,0xef,0x81,0x81,0xc7,0xc7,0xc7,0xc7,0xcf,0xcf,0xc3,0xc3,0xe3,0xe3,0x71,0x71,0xc7,0xc7,0xcf,0xc7,0xc7,0xe3,0xe3,0x81,0x81,0xe3,0x61,0xb9,0xd9,0xe3,0x45,0x45,0x45,0xc7,0xc7,0xc7,0x4d,0x4d,0x4d,0xff,0xff,0xff,0x45,0x45,0x45,0x89,0xb7,0xc3,0xb3,0xe3,0xef,0xef,0xdf,0xdf,0x9b,0x9d,0xc7,0xcf,0xc3,0xd9,0xc3,0xa7,0xd3,0xb9,0x6d,0xc1,0x7b,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x01,0xff,0xfd,0xff,0xef,0xff,0xff,0xff,0xf7,0xdf,0xd5,0xef,0xeb,0xdf,0xd5,0xbf,0xb5,0xff,0xf5,0xbb,0xb5,0x9f,0x95,0xff,0xf5,0xdf,0xd5,0x7d,0x75,0xdf,0xd5,0xf3,0xf5,0xff,0xff,0xf5,0x83,0x85,0xdf,0xd5,0xef,0xff,0x83,0xef,0xfb,0xd7,0xd5,0xdb,0xbf,0xb5,0xbb,0xff,0xf5,0xfb,0xff,0xf5,0xfb,0xef,0xeb,0xeb,0xff,0x8f,0xef,0xfb,0x83,0xff,0xbf,0xff,0xff,0xff,0xff,0x83,0xbb,0xf7,0xbf,0xff,0xff,0xff,0xf7,0xff,0x01,0x3f,0xd5,0xff,0xff,0xff,0xfd,0xfb,0xfb,0xef,0x01,0xff,0x01,0xf7,0x01,0x01,0x01,0x03,0x03,0xc1,0xc1,0x81,0x81,0x01,0x01,0x01,0x01,0xef,0xef,0x83,0x83,0xd1,0xd1,0xbd,0xbf,0xc1,0xc1,0x8f,0x8f,0xff,0x5d,0x5f,0xff,0xff,0xdf,0xdf,0xef,0x83,0xfb,0x03,0xfb,0xdb,0xdf,0xd5,0xbf,0xbf,0xb5,0x01,0x01,0x05,0xdf,0xdf,0xd5,0x01,0x01,0x05,0x01,0xf3,0xef,0x9b,0xdf,0xbf,0xa1,0xff,0x87,0xff,0x01,0xff,0xbb,0xd7,0xbf,0x01,0xff,0x01,0x01,0x01,0xfd,0xdf,0xdf,0xdf,0xbf,0x03,0xeb,0xf7,0xf7,0x03,0x7d,0x83,0xef,0x03,0xf7,0xdd,0xdd,0xef,0xef,0xbd,0xbd,0xb7,0xb7,0xfd,0xfd,0xbb,0xbb,0x3f,0x3f,0xfb,0xfb,0x0d,0x0d,0xbb,0xbb,0xbd,0xbd,0xef,0xef,0x5b,0xad,0xad,0x01,0x01,0xdf,0xdf,0x01,0xff,0xdb,0xfb,0xf7,0xdb,0xdb,0xdb,0xb1,0xb1,0xb1,0xfd,0xfd,0xf9,0xaf,0xaf,0xab,0xef,0xef,0xeb,0xfd,0x9f,0xdf,0xeb,0xdf,0xa3,0x1d,0xff,0xf7,0x83,0xfd,0x01,0xbb,0xd7,0xbf,0x7d,0x03,0x7d,0xb7,0xed,0x81,0xff,0x01,0x03,0x83,0xdb,0xe7,0xcf,0xc7,0x7b,0x7d,0xef,0xef,0xf7,0xe7,0xdd,0xdd,0xef,0xef,0x7d,0x7d,0x77,0x77,0xfd,0xfd,0xbb,0xbb,0xdd,0xdd,0xfb,0xfb,0xdd,0xdd,0xfb,0xfb,0x45,0x45,0x01,0x01,0xab,0xad,0xad,0xef,0xef,0xc7,0xc7,0xef,0xff,0xe7,0xf7,0xf7,0xbb,0xbb,0xbb,0x8f,0x8f,0x8f,0xfd,0xfd,0xfb,0x77,0x77,0x77,0xab,0xab,0xaf,0xfb,0xe3,0xdf,0xf7,0x01,0x1b,0xdb,0x87,0xf7,0xfb,0x81,0xfd,0xbb,0xd7,0xbd,0x7d,0x7b,0x7d,0xb7,0xeb,0xfd,0xfd,0x7d,0xdb,0x6f,0xc7,0xef,0x2f,0x37,0xfb,0xfb,0xef,0xef,0xc7,0xd7,0xdd,0xdd,0x01,0x01,0xfb,0xfb,0xf7,0xf7,0xfd,0xfd,0xfb,0xfb,0xfb,0xfb,0xf7,0xf7,0xdb,0xdb,0xf7,0xf7,0xfb,0xfb,0xef,0xef,0xab,0xfb,0xfb,0xef,0xef,0xd9,0xd9,0xef,0xff,0xf3,0xcb,0xef,0xbd,0xbd,0xbd,0xbf,0xbf,0xbf,0xfb,0xfb,0xfb,0xfb,0xfb,0xfb,0xad,0xad,0xab,0xd7,0xff,0xb7,0xeb,0xdf,0xd7,0xdf,0xf7,0xf7,0xc3,0xfd,0xfd,0xfb,0xd5,0xbb,0x7d,0xfb,0xfb,0xb7,0xef,0xfb,0xfb,0x7d,0xdb,0xef,0xdf,0xef,0xef,0xf7,0xf7,0xf7,0xef,0xef,0x37,0x37,0xbd,0xbd,0xf7,0xf7,0xe7,0xe7,0xef,0xef,0xfd,0xfd,0xf7,0xf7,0xe7,0xe7,0xcb,0xcb,0xdf,0xdf,0xef,0xef,0xe7,0xe7,0xef,0xef,0xf7,0xf7,0xf7,0xef,0xef,0xdf,0xdf,0xef,0xff,0xcd,0x2d,0xdf,0xbd,0xbd,0xbd,0xbf,0xbf,0xbf,0xe7,0xe7,0xe7,0xfd,0xfd,0xfd,0x6d,0x6d,0x6d,0xef,0x0f,0x7b,0xdd,0xdf,0xdf,0xef,0xf7,0xf7,0xfb,0xfd,0xf3,0xf7,0xb5,0xa7,0x7d,0xf7,0xf7,0x01,0xef,0xe7,0xe7,0xf3,0xbb,0xef,0xbf,0x9f,0xef,0xf7,0xcf,0xcf,0x83,0x01,0xe7,0xe7,0x73,0x73,0xf7,0xf7,0x9f,0x9f,0xdf,0xdf,0x01,0x01,0xcf,0xcf,0x9f,0x9f,0x3d,0x3d,0xc1,0xc1,0xdf,0xdf,0x9f,0x9f,0x9f,0x9f,0xcf,0xcf,0xcf,0x9f,0x9f,0xdf,0xdf,0x9f,0x01,0x3f,0xef,0x3f,0x7d,0x7d,0x7d,0xc1,0xc1,0xc1,0x9f,0x9f,0x9f,0xff,0xff,0xff,0xcf,0xcf,0xcf,0xf7,0xf1,0x05,0x3f,0xe1,0xef,0xef,0x03,0x01,0x83,0x01,0xcf,0xcf,0x73,0x9f,0x01,0xcf,0xcf,0xf7,0x01,0x9f,0x9f,0xcf,0x67,0xdf,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff ];

pub const MISAKI_GOTHIC_2ND_IMAGE: Image = Image {
    width: MISAKI_GOTHIC_2ND_WIDTH,
    height: MISAKI_GOTHIC_2ND_HEIGHT,
    flags: MISAKI_GOTHIC_2ND_FLAGS,
    data: &MISAKI_GOTHIC_2ND,
};

