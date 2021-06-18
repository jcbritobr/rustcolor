/*!
This module implements all terminal color rendering operations,
color conversions, color name definitions and traits for rust color library.

### 3bit and 4bit

The original specification only had 8 colors. The **SGR** parameters 30-37 selects
the foreground color, while 40-37 selects the background. Few terminals implements brighter
color, providing 8 additional foreground and background colors.

### Examples

* to get black letters on white background - **ESC[30;47m**.
* to get brighter colors with black letters on white background - **ESC[90;107m**. (+60)
* to reset all attributes - **ESC[0m**.

### 8bit

As [256-color](https://en.wikipedia.org/wiki/8-bit_color) lookup tables became common on graphic cards, escape sequences were added to select from a pre-defined set of 256 colors

### Examples

* ESC[ 38;5;⟨n⟩m Select foreground color
* ESC[ 48;5;⟨n⟩m Select background color \
* ESC[ 38;5;⟨n1⟩;48;5;⟨n2⟩m both foreground and background \
0 - 7:  standard colors (as in ESC [ 30–37 m) \
8 - 15:  high intensity colors (as in ESC [ 90–97 m) \
16 - 231:  6 × 6 × 6 cube (216 colors): 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5) \
232-255:  grayscale from black to white in 24 steps
*/

use std::usize;

pub const FG_BLACK: usize = 30;
pub const FG_RED: usize = 31;
pub const FG_GREEN: usize = 32;
pub const FG_YELLOW: usize = 33;
pub const FG_BLUE: usize = 34;
pub const FG_MAGENTA: usize = 35;
pub const FG_CYAN: usize = 36;
pub const FG_WHITE: usize = 37;
pub const DEFAULT: usize = 39;

pub const BG_BLACK: usize = 40;
pub const BG_RED: usize = 41;
pub const BG_GREEN: usize = 42;
pub const BG_YELLOW: usize = 43;
pub const BG_BLUE: usize = 44;
pub const BG_MAGENTA: usize = 45;
pub const BG_CYAN: usize = 46;
pub const BG_WHITE: usize = 47;
pub const BG_DEFAULT: usize = 49;

pub const FG_DARK_GRAY: usize = 90;
pub const FG_LIGHT_RED: usize = 91;
pub const FG_LIGHT_GREEN: usize = 92;
pub const FG_LIGHT_YELLOW: usize = 93;
pub const FG_LIGHT_BLUE: usize = 94;
pub const FG_LIGHT_MAGENTA: usize = 95;
pub const FG_LIGHT_CYAN: usize = 96;
pub const FG_LIGHT_WHITE: usize = 97;

pub const BG_DARK_GRAY: usize = 100;
pub const BG_LIGHT_RED: usize = 101;
pub const BG_LIGHT_GREEN: usize = 102;
pub const BG_LIGH_YELLOW: usize = 103;
pub const BG_LIGHT_BLUE: usize = 104;
pub const BG_LIGHT_MAGENTA: usize = 105;
pub const BG_LIGHT_CYAN: usize = 106;
pub const BG_LIGHT_WHITE: usize = 107;

/// darken the color if it has 4bit
pub fn darken(color: usize) -> usize {
    color - 60
}

/// lighten the color if it has 4bit
pub fn lighten(color: usize) -> usize {
    color + 60
}

pub const C8_000: usize = 0;
pub const C8_001: usize = 1;
pub const C8_002: usize = 2;
pub const C8_003: usize = 3;
pub const C8_004: usize = 4;
pub const C8_005: usize = 5;
pub const C8_006: usize = 6;
pub const C8_007: usize = 7;
pub const C8_008: usize = 8;
pub const C8_009: usize = 9;

pub const C8_010: usize = 10;
pub const C8_011: usize = 11;
pub const C8_012: usize = 12;
pub const C8_013: usize = 13;
pub const C8_014: usize = 14;
pub const C8_015: usize = 15;
pub const C8_016: usize = 16;
pub const C8_017: usize = 17;
pub const C8_018: usize = 18;
pub const C8_019: usize = 19;

pub const C8_020: usize = 20;
pub const C8_021: usize = 21;
pub const C8_022: usize = 22;
pub const C8_023: usize = 23;
pub const C8_024: usize = 24;
pub const C8_025: usize = 25;
pub const C8_026: usize = 26;
pub const C8_027: usize = 27;
pub const C8_028: usize = 28;
pub const C8_029: usize = 29;

pub const C8_030: usize = 30;
pub const C8_031: usize = 31;
pub const C8_032: usize = 32;
pub const C8_033: usize = 33;
pub const C8_034: usize = 34;
pub const C8_035: usize = 35;
pub const C8_036: usize = 36;
pub const C8_037: usize = 37;
pub const C8_038: usize = 38;
pub const C8_039: usize = 39;

pub const C8_040: usize = 40;
pub const C8_041: usize = 41;
pub const C8_042: usize = 42;
pub const C8_043: usize = 43;
pub const C8_044: usize = 44;
pub const C8_045: usize = 45;
pub const C8_046: usize = 46;
pub const C8_047: usize = 47;
pub const C8_048: usize = 48;
pub const C8_049: usize = 49;

pub const C8_050: usize = 50;
pub const C8_051: usize = 51;
pub const C8_052: usize = 52;
pub const C8_053: usize = 53;
pub const C8_054: usize = 54;
pub const C8_055: usize = 55;
pub const C8_056: usize = 56;
pub const C8_057: usize = 57;
pub const C8_058: usize = 58;
pub const C8_059: usize = 59;

pub const C8_060: usize = 60;
pub const C8_061: usize = 61;
pub const C8_062: usize = 62;
pub const C8_063: usize = 63;
pub const C8_064: usize = 64;
pub const C8_065: usize = 65;
pub const C8_066: usize = 66;
pub const C8_067: usize = 67;
pub const C8_068: usize = 68;
pub const C8_069: usize = 69;

pub const C8_070: usize = 70;
pub const C8_071: usize = 71;
pub const C8_072: usize = 72;
pub const C8_073: usize = 73;
pub const C8_074: usize = 74;
pub const C8_075: usize = 75;
pub const C8_076: usize = 76;
pub const C8_077: usize = 77;
pub const C8_078: usize = 78;
pub const C8_079: usize = 79;

pub const C8_080: usize = 80;
pub const C8_081: usize = 81;
pub const C8_082: usize = 82;
pub const C8_083: usize = 83;
pub const C8_084: usize = 84;
pub const C8_085: usize = 85;
pub const C8_086: usize = 86;
pub const C8_087: usize = 87;
pub const C8_088: usize = 88;
pub const C8_089: usize = 89;

pub const C8_090: usize = 90;
pub const C8_091: usize = 91;
pub const C8_092: usize = 92;
pub const C8_093: usize = 93;
pub const C8_094: usize = 94;
pub const C8_095: usize = 95;
pub const C8_096: usize = 96;
pub const C8_097: usize = 97;
pub const C8_098: usize = 98;
pub const C8_099: usize = 99;

pub const C8_100: usize = 100;
pub const C8_101: usize = 101;
pub const C8_102: usize = 102;
pub const C8_103: usize = 103;
pub const C8_104: usize = 104;
pub const C8_105: usize = 105;
pub const C8_106: usize = 106;
pub const C8_107: usize = 107;
pub const C8_108: usize = 108;
pub const C8_109: usize = 109;

pub const C8_110: usize = 110;
pub const C8_111: usize = 111;
pub const C8_112: usize = 112;
pub const C8_113: usize = 113;
pub const C8_114: usize = 114;
pub const C8_115: usize = 115;
pub const C8_116: usize = 116;
pub const C8_117: usize = 117;
pub const C8_118: usize = 118;
pub const C8_119: usize = 119;

pub const C8_120: usize = 120;
pub const C8_121: usize = 121;
pub const C8_122: usize = 122;
pub const C8_123: usize = 123;
pub const C8_124: usize = 124;
pub const C8_125: usize = 125;
pub const C8_126: usize = 126;
pub const C8_127: usize = 127;
pub const C8_128: usize = 128;
pub const C8_129: usize = 129;

pub const C8_130: usize = 130;
pub const C8_131: usize = 131;
pub const C8_132: usize = 132;
pub const C8_133: usize = 133;
pub const C8_134: usize = 134;
pub const C8_135: usize = 135;
pub const C8_136: usize = 136;
pub const C8_137: usize = 137;
pub const C8_138: usize = 138;
pub const C8_139: usize = 139;

pub const C8_140: usize = 140;
pub const C8_141: usize = 141;
pub const C8_142: usize = 142;
pub const C8_143: usize = 143;
pub const C8_144: usize = 144;
pub const C8_145: usize = 145;
pub const C8_146: usize = 146;
pub const C8_147: usize = 147;
pub const C8_148: usize = 148;
pub const C8_149: usize = 149;

pub const C8_150: usize = 150;
pub const C8_151: usize = 151;
pub const C8_152: usize = 152;
pub const C8_153: usize = 153;
pub const C8_154: usize = 154;
pub const C8_155: usize = 155;
pub const C8_156: usize = 156;
pub const C8_157: usize = 157;
pub const C8_158: usize = 158;
pub const C8_159: usize = 159;

pub const C8_160: usize = 160;
pub const C8_161: usize = 161;
pub const C8_162: usize = 162;
pub const C8_163: usize = 163;
pub const C8_164: usize = 164;
pub const C8_165: usize = 165;
pub const C8_166: usize = 166;
pub const C8_167: usize = 167;
pub const C8_168: usize = 168;
pub const C8_169: usize = 169;

pub const C8_170: usize = 170;
pub const C8_171: usize = 171;
pub const C8_172: usize = 172;
pub const C8_173: usize = 173;
pub const C8_174: usize = 174;
pub const C8_175: usize = 175;
pub const C8_176: usize = 176;
pub const C8_177: usize = 177;
pub const C8_178: usize = 178;
pub const C8_179: usize = 179;

pub const C8_180: usize = 180;
pub const C8_181: usize = 181;
pub const C8_182: usize = 182;
pub const C8_183: usize = 183;
pub const C8_184: usize = 184;
pub const C8_185: usize = 185;
pub const C8_186: usize = 186;
pub const C8_187: usize = 187;
pub const C8_188: usize = 188;
pub const C8_189: usize = 189;

pub const C8_190: usize = 190;
pub const C8_191: usize = 191;
pub const C8_192: usize = 192;
pub const C8_193: usize = 193;
pub const C8_194: usize = 194;
pub const C8_195: usize = 195;
pub const C8_196: usize = 196;
pub const C8_197: usize = 197;
pub const C8_198: usize = 198;
pub const C8_199: usize = 199;

pub const C8_200: usize = 200;
pub const C8_201: usize = 201;
pub const C8_202: usize = 202;
pub const C8_203: usize = 203;
pub const C8_204: usize = 204;
pub const C8_205: usize = 205;
pub const C8_206: usize = 206;
pub const C8_207: usize = 207;
pub const C8_208: usize = 208;
pub const C8_209: usize = 209;

pub const C8_210: usize = 210;
pub const C8_211: usize = 211;
pub const C8_212: usize = 212;
pub const C8_213: usize = 213;
pub const C8_214: usize = 214;
pub const C8_215: usize = 215;
pub const C8_216: usize = 216;
pub const C8_217: usize = 217;
pub const C8_218: usize = 218;
pub const C8_219: usize = 219;

pub const C8_220: usize = 220;
pub const C8_221: usize = 221;
pub const C8_222: usize = 222;
pub const C8_223: usize = 223;
pub const C8_224: usize = 224;
pub const C8_225: usize = 225;
pub const C8_226: usize = 226;
pub const C8_227: usize = 227;
pub const C8_228: usize = 228;
pub const C8_229: usize = 229;

pub const C8_230: usize = 230;
pub const C8_231: usize = 231;
pub const C8_232: usize = 232;
pub const C8_233: usize = 233;
pub const C8_234: usize = 234;
pub const C8_235: usize = 235;
pub const C8_236: usize = 236;
pub const C8_237: usize = 237;
pub const C8_238: usize = 238;
pub const C8_239: usize = 239;

pub const C8_240: usize = 240;
pub const C8_241: usize = 241;
pub const C8_242: usize = 242;
pub const C8_243: usize = 243;
pub const C8_244: usize = 244;
pub const C8_245: usize = 245;
pub const C8_246: usize = 246;
pub const C8_247: usize = 247;
pub const C8_248: usize = 248;
pub const C8_249: usize = 249;

pub const C8_250: usize = 250;
pub const C8_251: usize = 251;
pub const C8_252: usize = 252;
pub const C8_253: usize = 253;
pub const C8_254: usize = 254;
pub const C8_255: usize = 255;