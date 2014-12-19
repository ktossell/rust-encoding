// AUTOGENERATED FROM index-koi8-r.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index index-koi8-r.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: c5497cd9071cb352c0e56b219154e539badf63de40b71578f09e2e11fe7d50ae
// Date: 2014-09-15

static FORWARD_TABLE: &'static [u16] = &[
    9472, 9474, 9484, 9488, 9492, 9496, 9500, 9508, 9516, 9524, 9532, 9600,
    9604, 9608, 9612, 9616, 9617, 9618, 9619, 8992, 9632, 8729, 8730, 8776,
    8804, 8805, 160, 8993, 176, 178, 183, 247, 9552, 9553, 9554, 1105, 9555,
    9556, 9557, 9558, 9559, 9560, 9561, 9562, 9563, 9564, 9565, 9566, 9567,
    9568, 9569, 1025, 9570, 9571, 9572, 9573, 9574, 9575, 9576, 9577, 9578,
    9579, 9580, 169, 1102, 1072, 1073, 1094, 1076, 1077, 1092, 1075, 1093,
    1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087, 1103, 1088, 1089, 1090,
    1091, 1078, 1074, 1100, 1099, 1079, 1096, 1101, 1097, 1095, 1098, 1070,
    1040, 1041, 1062, 1044, 1045, 1060, 1043, 1061, 1048, 1049, 1050, 1051,
    1052, 1053, 1054, 1055, 1071, 1056, 1057, 1058, 1059, 1046, 1042, 1068,
    1067, 1047, 1064, 1069, 1065, 1063, 1066,
];

/// Returns the index code point for pointer `code` in this index.
#[inline]
#[stable]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

static BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 154, 0, 0, 0, 0, 0, 0, 0, 0, 191, 0, 0, 0, 0, 0, 0,
    156, 0, 157, 0, 0, 0, 0, 158, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 226, 247, 231,
    228, 229, 246, 250, 233, 234, 235, 236, 237, 238, 239, 240, 242, 243, 244,
    245, 230, 232, 227, 254, 251, 253, 255, 249, 248, 252, 224, 241, 193, 194,
    215, 199, 196, 197, 214, 218, 201, 202, 203, 204, 205, 206, 207, 208, 210,
    211, 212, 213, 198, 200, 195, 222, 219, 221, 223, 217, 216, 220, 192, 209,
    0, 163, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 150, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 151, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 152, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 155, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    128, 0, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 0, 0, 0, 131, 0, 0, 0, 132, 0,
    0, 0, 133, 0, 0, 0, 134, 0, 0, 0, 0, 0, 0, 0, 135, 0, 0, 0, 0, 0, 0, 0,
    136, 0, 0, 0, 0, 0, 0, 0, 137, 0, 0, 0, 0, 0, 0, 0, 138, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 161, 162, 164, 165, 166, 167,
    168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 180, 181, 182, 183,
    184, 185, 186, 187, 188, 189, 190, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 139, 0, 0, 0, 140, 0, 0, 0, 141, 0, 0, 0, 142, 0, 0, 0,
    143, 144, 145, 146, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 148, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];

static BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 0, 0, 0, 32, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 96, 128, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    192, 0, 224, 256, 0, 0, 0, 0, 0, 288, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 320, 352, 384, 416, 448, 480,
];

/// Returns the index pointer for code point `code` in this index.
#[inline]
#[stable]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 5) as uint;
    let offset = if offset < 302 {BACKWARD_TABLE_UPPER[offset] as uint} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 31) as uint)]
}

#[cfg(test)]
single_byte_tests!();
