//! Color module

macro_rules! color {
    ($($constname:ident, $val:expr;)*) => {
        $(
            /// Generated Color: $constname, $val
            pub const $constname: Color = Color::from_rgb_tuple($val);
        )*
    };
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Representation of a color.
pub struct Color(pub u32);

impl Color {
    /// Creates a new Color with the given integer.
    pub const fn new(value: u32) -> Self {
        Color(value)
    }

    /// Creates a new Color from a traditional RGB triplet.
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Color((red as u32) << 16 | (green as u32) << 8 | (blue as u32))
    }

    /// Creates a new Color from RGB floats
    /// Each float is on a scale from 0..1
    pub fn from_f32(red: f32, green: f32, blue: f32) -> Self {
        Color::from_rgb((red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8)
    }

    /// Creates an new Color from a RGB tuple.
    const fn from_rgb_tuple(color: (u8, u8, u8)) -> Self {
        Color((color.0 as u32) << 16 | (color.1 as u32) << 8 | (color.2 as u32))
    }

    /// Gets the red component of the color.
    pub const fn r(self) -> u8 {
        ((self.0 >> 16) & 255) as u8
    }

    /// Gets the green component of the color.
    pub const fn g(self) -> u8 {
        ((self.0 >> 8) & 255) as u8
    }

    /// Gets the blue component of the color.
    pub const fn b(self) -> u8 {
        (self.0 & 255) as u8
    }

    /// Gets the red component of the color.
    pub const fn r_f(self) -> f32 {
        (self.r() / 255) as f32
    }

    /// Gets the green component of the color.
    pub const fn g_f(self) -> f32 {
        (self.g() / 255) as f32
    }

    /// Gets the blue component of the color.
    pub const fn b_f(self) -> f32 {
        (self.b() / 255) as f32
    }

    /// Returns a tuple of the RGB components of the color.
    pub const fn tuple(self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }

    /// Returns a Hexadecimal string of the color.
    pub fn hex(self) -> String {
        format!("{:06X}", self.0)
    }

    /// Returns a grayscale version of the color
    pub fn to_grayscale(&self) -> Color {
        let col = (0.3 * self.r_f()) + (0.59 * self.g_f()) + (0.11 * self.b_f());
        (col, col, col).into()
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((red, green, blue): (u8, u8, u8)) -> Self {
        Color::from_rgb(red, green, blue)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Color::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

impl From<u64> for Color {
    fn from(value: u64) -> Color {
        Color(value as u32)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Color {
        Color(value)
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Color {
        Color(value as u32)
    }
}

color! {
    SNOW, (255, 250, 250);
    GHOST_WHITE, (248, 248, 255);
    WHITE_SMOKE, (245, 245, 245);
    GAINSBORO, (220, 220, 220);
    FLORAL_WHITE, (255, 250, 240);
    OLD_LACE, (253, 245, 230);
    LINEN, (250, 240, 230);
    ANTIQUE_WHITE, (250, 235, 215);
    PAPAYA_WHIP, (255, 239, 213);
    BLANCHED_ALMOND, (255, 235, 205);
    BISQUE, (255, 228, 196);
    PEACH_PUFF, (255, 218, 185);
    NAVAJO_WHITE, (255, 222, 173);
    MOCCASIN, (255, 228, 181);
    CORNSILK, (255, 248, 220);
    IVORY, (255, 255, 240);
    LEMON_CHIFFON, (255, 250, 205);
    SEASHELL, (255, 245, 238);
    HONEYDEW, (240, 255, 240);
    MINT_CREAM, (245, 255, 250);
    AZURE, (240, 255, 255);
    ALICE_BLUE, (240, 248, 255);
    LAVENDER, (230, 230, 250);
    LAVENDER_BLUSH, (255, 240, 245);
    MISTY_ROSE, (255, 228, 225);
    WHITE, (255, 255, 255);
    BLACK, (0, 0, 0);
    DARK_SLATE, (47, 79, 79);
    DIM_GRAY, (105, 105, 105);
    DIM_GREY, (105, 105, 105);
    SLATE_GRAY, (112, 128, 144);
    LIGHT_SLATE, (119, 136, 153);
    GRAY, (190, 190, 190);
    WEB_GRAY, (128, 128, 128);
    LIGHT_GRAY, (211, 211, 211);
    MIDNIGHT_BLUE, (25, 25, 112);
    NAVY, (0, 0, 128);
    CORNFLOWER_BLUE, (100, 149, 237);
    DARKSLATEBLUE, (72, 61, 139);
    SLATE_BLUE, (106, 90, 205);
    MEDIUM_SLATE, (123, 104, 238);
    LIGHT_SLATE_BLUE, (132, 112, 255);
    MEDIUM_BLUE, (0, 0, 205);
    ROYAL_BLUE, (65, 105, 225);
    BLUE, (0, 0, 255);
    DODGER_BLUE, (30, 144, 255);
    DEEP_SKY, (0, 191, 255);
    SKY_BLUE, (135, 206, 235);
    LIGHT_SKY, (135, 206, 250);
    STEEL_BLUE, (70, 130, 180);
    LIGHT_STEEL_BLUE, (176, 196, 222);
    LIGHT_BLUE, (173, 216, 230);
    POWDER_BLUE, (176, 224, 230);
    PALE_TURQUOISE, (175, 238, 238);
    DARK_TURQUOISE, (0, 206, 209);
    MEDIUM_TURQUOISE, (72, 209, 204);
    TURQUOISE, (64, 224, 208);
    CYAN, (0, 255, 255);
    AQUA, (0, 255, 255);
    LIGHT_CYAN, (224, 255, 255);
    CADET_BLUE, (95, 158, 160);
    MEDIUM_AQUAMARINE, (102, 205, 170);
    AQUAMARINE, (127, 255, 212);
    DARK_GREEN, (0, 100, 0);
    DARK_OLIVE, (85, 107, 47);
    DARK_SEA_GREEN, (143, 188, 143);
    SEA_GREEN, (46, 139, 87);
    MEDIUM_SEA, (60, 179, 113);
    LIGHT_SEA, (32, 178, 170);
    PALE_GREEN, (152, 251, 152);
    SPRING_GREEN, (0, 255, 127);
    LAWN_GREEN, (124, 252, 0);
    GREEN, (0, 255, 0);
    LIME, (0, 255, 0);
    WEB_GREEN, (0, 128, 0);
    CHARTREUSE, (127, 255, 0);
    MEDIUM_SPRING, (0, 250, 154);
    GREEN_YELLOW, (173, 255, 47);
    LIME_GREEN, (50, 205, 50);
    YELLOW_GREEN, (154, 205, 50);
    FOREST_GREEN, (34, 139, 34);
    OLIVE_DRAB, (107, 142, 35);
    DARK_KHAKI, (189, 183, 107);
    KHAKI, (240, 230, 140);
    // /// TODO remove duplicates
    PALE_GOLDENROD, (238, 232, 170);
    PALEGOLDENROD, (238, 232, 170);
    LIGHT_GOLDENROD, (250, 250, 210);
    LIGHTGOLDENRODYELLOW, (250, 250, 210);
    LIGHT_YELLOW, (255, 255, 224);
    LIGHTYELLOW, (255, 255, 224);
    YELLOW, (255, 255, 0);
    GOLD, (255, 215, 0);
    LIGHTGOLDENROD, (238, 221, 130);
    GOLDENROD, (218, 165, 32);
    DARK_GOLDENROD, (184, 134, 11);
    DARKGOLDENROD, (184, 134, 11);
    ROSY_BROWN, (188, 143, 143);
    ROSYBROWN, (188, 143, 143);
    INDIAN_RED, (205, 92, 92);
    INDIANRED, (205, 92, 92);
    SADDLE_BROWN, (139, 69, 19);
    SADDLEBROWN, (139, 69, 19);
    SIENNA, (160, 82, 45);
    PERU, (205, 133, 63);
    BURLYWOOD, (222, 184, 135);
    BEIGE, (245, 245, 220);
    WHEAT, (245, 222, 179);
    SANDY_BROWN, (244, 164, 96);
    SANDYBROWN, (244, 164, 96);
    TAN, (210, 180, 140);
    CHOCOLATE, (210, 105, 30);
    FIREBRICK_34, (178, 34, 34);
    BROWN_42, (165, 42, 42);
    DARK_SALMON, (233, 150, 122);
    DARKSALMON, (233, 150, 122);
    SALMON, (250, 128, 114);
    LIGHT_SALMON, (255, 160, 122);
    LIGHTSALMON, (255, 160, 122);
    ORANGE, (255, 165, 0);
    DARK_ORANGE, (255, 140, 0);
    DARKORANGE, (255, 140, 0);
    CORAL, (255, 127, 80);
    LIGHT_CORAL, (240, 128, 128);
    LIGHTCORAL, (240, 128, 128);
    TOMATO, (255, 99, 71);
    ORANGE_RED, (255, 69, 0);
    ORANGERED, (255, 69, 0);
    RED, (255, 0, 0);
    HOT_PINK, (255, 105, 180);
    HOTPINK, (255, 105, 180);
    DEEP_PINK, (255, 20, 147);
    DEEPPINK, (255, 20, 147);
    PINK, (255, 192, 203);
    LIGHT_PINK, (255, 182, 193);
    LIGHTPINK, (255, 182, 193);
    PALE_VIOLET, (219, 112, 147);
    PALEVIOLETRED, (219, 112, 147);
    MAROON, (176, 48, 96);
    X11_MAROON, (176, 48, 96);
    X11MAROON, (176, 48, 96);
    WEB_MAROON, (128, 0, 0);
    WEBMAROON, (128, 0, 0);
    MEDIUM_VIOLET, (199, 21, 133);
    MEDIUMVIOLETRED, (199, 21, 133);
    VIOLET_RED, (208, 32, 144);
    VIOLETRED, (208, 32, 144);
    MAGENTA, (255, 0, 255);
    FUCHSIA, (255, 0, 255);
    VIOLET, (238, 130, 238);
    PLUM, (221, 160, 221);
    ORCHID, (218, 112, 214);
    MEDIUM_ORCHID, (186, 85, 211);
    MEDIUMORCHID, (186, 85, 211);
    DARK_ORCHID, (153, 50, 204);
    DARKORCHID, (153, 50, 204);
    DARK_VIOLET, (148, 0, 211);
    DARKVIOLET, (148, 0, 211);
    BLUE_VIOLET, (138, 43, 226);
    BLUEVIOLET, (138, 43, 226);
    PURPLE, (160, 32, 240);
    X11_PURPLE, (160, 32, 240);
    X11PURPLE, (160, 32, 240);
    WEB_PURPLE, (128, 0, 128);
    WEBPURPLE, (128, 0, 128);
    MEDIUM_PURPLE, (147, 112, 219);
    MEDIUMPURPLE, (147, 112, 219);
    THISTLE, (216, 191, 216);
    SNOW1, (255, 250, 250);
    SNOW2, (238, 233, 233);
    SNOW3, (205, 201, 201);
    SNOW4, (139, 137, 137);
    SEASHELL1, (255, 245, 238);
    SEASHELL2, (238, 229, 222);
    SEASHELL3, (205, 197, 191);
    SEASHELL4, (139, 134, 130);
    ANTIQUEWHITE1, (255, 239, 219);
    ANTIQUEWHITE2, (238, 223, 204);
    ANTIQUEWHITE3, (205, 192, 176);
    ANTIQUEWHITE4, (139, 131, 120);
    BISQUE1, (255, 228, 196);
    BISQUE2, (238, 213, 183);
    BISQUE3, (205, 183, 158);
    BISQUE4, (139, 125, 107);
    PEACHPUFF1, (255, 218, 185);
    PEACHPUFF2, (238, 203, 173);
    PEACHPUFF3, (205, 175, 149);
    PEACHPUFF4, (139, 119, 101);
    NAVAJOWHITE1, (255, 222, 173);
    NAVAJOWHITE2, (238, 207, 161);
    NAVAJOWHITE3, (205, 179, 139);
    NAVAJOWHITE4, (139, 121, 94);
    LEMONCHIFFON1, (255, 250, 205);
    LEMONCHIFFON2, (238, 233, 191);
    LEMONCHIFFON3, (205, 201, 165);
    LEMONCHIFFON4, (139, 137, 112);
    CORNSILK1, (255, 248, 220);
    CORNSILK2, (238, 232, 205);
    CORNSILK3, (205, 200, 177);
    CORNSILK4, (139, 136, 120);
    IVORY1, (255, 255, 240);
    IVORY2, (238, 238, 224);
    IVORY3, (205, 205, 193);
    IVORY4, (139, 139, 131);
    HONEYDEW1, (240, 255, 240);
    HONEYDEW2, (224, 238, 224);
    HONEYDEW3, (193, 205, 193);
    HONEYDEW4, (131, 139, 131);
    LAVENDERBLUSH1, (255, 240, 245);
    LAVENDERBLUSH2, (238, 224, 229);
    LAVENDERBLUSH3, (205, 193, 197);
    LAVENDERBLUSH4, (139, 131, 134);
    MISTYROSE1, (255, 228, 225);
    MISTYROSE2, (238, 213, 210);
    MISTYROSE3, (205, 183, 181);
    MISTYROSE4, (139, 125, 123);
    AZURE1, (240, 255, 255);
    AZURE2, (224, 238, 238);
    AZURE3, (193, 205, 205);
    AZURE4, (131, 139, 139);
    SLATEBLUE1, (131, 111, 255);
    SLATEBLUE2, (122, 103, 238);
    SLATEBLUE3, (105, 89, 205);
    SLATEBLUE4, (71, 60, 139);
    ROYALBLUE1, (72, 118, 255);
    ROYALBLUE2, (67, 110, 238);
    ROYALBLUE3, (58, 95, 205);
    ROYALBLUE4, (39, 64, 139);
    BLUE1, (0, 0, 255);
    BLUE2, (0, 0, 238);
    BLUE3, (0, 0, 205);
    BLUE4, (0, 0, 139);
    DODGERBLUE1, (30, 144, 255);
    DODGERBLUE2, (28, 134, 238);
    DODGERBLUE3, (24, 116, 205);
    DODGERBLUE4, (16, 78, 139);
    STEELBLUE1, (99, 184, 255);
    STEELBLUE2, (92, 172, 238);
    STEELBLUE3, (79, 148, 205);
    STEELBLUE4, (54, 100, 139);
    DEEPSKYBLUE1, (0, 191, 255);
    DEEPSKYBLUE2, (0, 178, 238);
    DEEPSKYBLUE3, (0, 154, 205);
    DEEPSKYBLUE4, (0, 104, 139);
    SKYBLUE1, (135, 206, 255);
    SKYBLUE2, (126, 192, 238);
    SKYBLUE3, (108, 166, 205);
    SKYBLUE4, (74, 112, 139);
    LIGHTSKYBLUE1, (176, 226, 255);
    LIGHTSKYBLUE2, (164, 211, 238);
    LIGHTSKYBLUE3, (141, 182, 205);
    LIGHTSKYBLUE4, (96, 123, 139);
    SLATEGRAY1, (198, 226, 255);
    SLATEGRAY2, (185, 211, 238);
    SLATEGRAY3, (159, 182, 205);
    SLATEGRAY4, (108, 123, 139);
    LIGHTSTEELBLUE1, (202, 225, 255);
    LIGHTSTEELBLUE2, (188, 210, 238);
    LIGHTSTEELBLUE3, (162, 181, 205);
    LIGHTSTEELBLUE4, (110, 123, 139);
    LIGHTBLUE1, (191, 239, 255);
    LIGHTBLUE2, (178, 223, 238);
    LIGHTBLUE3, (154, 192, 205);
    LIGHTBLUE4, (104, 131, 139);
    LIGHTCYAN1, (224, 255, 255);
    LIGHTCYAN2, (209, 238, 238);
    LIGHTCYAN3, (180, 205, 205);
    LIGHTCYAN4, (122, 139, 139);
    PALETURQUOISE1, (187, 255, 255);
    PALETURQUOISE2, (174, 238, 238);
    PALETURQUOISE3, (150, 205, 205);
    PALETURQUOISE4, (102, 139, 139);
    CADETBLUE1, (152, 245, 255);
    CADETBLUE2, (142, 229, 238);
    CADETBLUE3, (122, 197, 205);
    CADETBLUE4, (83, 134, 139);
    TURQUOISE1, (0, 245, 255);
    TURQUOISE2, (0, 229, 238);
    TURQUOISE3, (0, 197, 205);
    TURQUOISE4, (0, 134, 139);
    CYAN1, (0, 255, 255);
    CYAN2, (0, 238, 238);
    CYAN3, (0, 205, 205);
    CYAN4, (0, 139, 139);
    DARKSLATEGRAY1, (151, 255, 255);
    DARKSLATEGRAY2, (141, 238, 238);
    DARKSLATEGRAY3, (121, 205, 205);
    DARKSLATEGRAY4, (82, 139, 139);
    AQUAMARINE1, (127, 255, 212);
    AQUAMARINE2, (118, 238, 198);
    AQUAMARINE3, (102, 205, 170);
    AQUAMARINE4, (69, 139, 116);
    DARKSEAGREEN1, (193, 255, 193);
    DARKSEAGREEN2, (180, 238, 180);
    DARKSEAGREEN3, (155, 205, 155);
    DARKSEAGREEN4, (105, 139, 105);
    SEAGREEN1, (84, 255, 159);
    SEAGREEN2, (78, 238, 148);
    SEAGREEN3, (67, 205, 128);
    SEAGREEN4, (46, 139, 87);
    PALEGREEN1, (154, 255, 154);
    PALEGREEN2, (144, 238, 144);
    PALEGREEN3, (124, 205, 124);
    PALEGREEN4, (84, 139, 84);
    SPRINGGREEN1, (0, 255, 127);
    SPRINGGREEN2, (0, 238, 118);
    SPRINGGREEN3, (0, 205, 102);
    SPRINGGREEN4, (0, 139, 69);
    GREEN1, (0, 255, 0);
    GREEN2, (0, 238, 0);
    GREEN3, (0, 205, 0);
    GREEN4, (0, 139, 0);
    CHARTREUSE1, (127, 255, 0);
    CHARTREUSE2, (118, 238, 0);
    CHARTREUSE3, (102, 205, 0);
    CHARTREUSE4, (69, 139, 0);
    OLIVEDRAB1, (192, 255, 62);
    OLIVEDRAB2, (179, 238, 58);
    OLIVEDRAB3, (154, 205, 50);
    OLIVEDRAB4, (105, 139, 34);
    DARKOLIVEGREEN1, (202, 255, 112);
    DARKOLIVEGREEN2, (188, 238, 104);
    DARKOLIVEGREEN3, (162, 205, 90);
    DARKOLIVEGREEN4, (110, 139, 61);
    KHAKI1, (255, 246, 143);
    KHAKI2, (238, 230, 133);
    KHAKI3, (205, 198, 115);
    KHAKI4, (139, 134, 78);
    LIGHTGOLDENROD1, (255, 236, 139);
    LIGHTGOLDENROD2, (238, 220, 130);
    LIGHTGOLDENROD3, (205, 190, 112);
    LIGHTGOLDENROD4, (139, 129, 76);
    LIGHTYELLOW1, (255, 255, 224);
    LIGHTYELLOW2, (238, 238, 209);
    LIGHTYELLOW3, (205, 205, 180);
    LIGHTYELLOW4, (139, 139, 122);
    YELLOW1, (255, 255, 0);
    YELLOW2, (238, 238, 0);
    YELLOW3, (205, 205, 0);
    YELLOW4, (139, 139, 0);
    GOLD1, (255, 215, 0);
    GOLD2, (238, 201, 0);
    GOLD3, (205, 173, 0);
    GOLD4, (139, 117, 0);
    GOLDENROD1, (255, 193, 37);
    GOLDENROD2, (238, 180, 34);
    GOLDENROD3, (205, 155, 29);
    GOLDENROD4, (139, 105, 20);
    DARKGOLDENROD1, (255, 185, 15);
    DARKGOLDENROD2, (238, 173, 14);
    DARKGOLDENROD3, (205, 149, 12);
    DARKGOLDENROD4, (139, 101, 8);
    ROSYBROWN1, (255, 193, 193);
    ROSYBROWN2, (238, 180, 180);
    ROSYBROWN3, (205, 155, 155);
    ROSYBROWN4, (139, 105, 105);
    INDIANRED1, (255, 106, 106);
    INDIANRED2, (238, 99, 99);
    INDIANRED3, (205, 85, 85);
    INDIANRED4, (139, 58, 58);
    SIENNA1, (255, 130, 71);
    SIENNA2, (238, 121, 66);
    SIENNA3, (205, 104, 57);
    SIENNA4, (139, 71, 38);
    BURLYWOOD1, (255, 211, 155);
    BURLYWOOD2, (238, 197, 145);
    BURLYWOOD3, (205, 170, 125);
    BURLYWOOD4, (139, 115, 85);
    WHEAT1, (255, 231, 186);
    WHEAT2, (238, 216, 174);
    WHEAT3, (205, 186, 150);
    WHEAT4, (139, 126, 102);
    TAN1, (255, 165, 79);
    TAN2, (238, 154, 73);
    TAN3, (205, 133, 63);
    TAN4, (139, 90, 43);
    CHOCOLATE1, (255, 127, 36);
    CHOCOLATE2, (238, 118, 33);
    CHOCOLATE3, (205, 102, 29);
    CHOCOLATE4, (139, 69, 19);
    FIREBRICK1, (255, 48, 48);
    FIREBRICK2, (238, 44, 44);
    FIREBRICK3, (205, 38, 38);
    FIREBRICK4, (139, 26, 26);
    BROWN1, (255, 64, 64);
    BROWN2, (238, 59, 59);
    BROWN3, (205, 51, 51);
    BROWN4, (139, 35, 35);
    SALMON1, (255, 140, 105);
    SALMON2, (238, 130, 98);
    SALMON3, (205, 112, 84);
    SALMON4, (139, 76, 57);
    LIGHTSALMON1, (255, 160, 122);
    LIGHTSALMON2, (238, 149, 114);
    LIGHTSALMON3, (205, 129, 98);
    LIGHTSALMON4, (139, 87, 66);
    ORANGE1, (255, 165, 0);
    ORANGE2, (238, 154, 0);
    ORANGE3, (205, 133, 0);
    ORANGE4, (139, 90, 0);
    DARKORANGE1, (255, 127, 0);
    DARKORANGE2, (238, 118, 0);
    DARKORANGE3, (205, 102, 0);
    DARKORANGE4, (139, 69, 0);
    CORAL1, (255, 114, 86);
    CORAL2, (238, 106, 80);
    CORAL3, (205, 91, 69);
    CORAL4, (139, 62, 47);
    TOMATO1, (255, 99, 71);
    TOMATO2, (238, 92, 66);
    TOMATO3, (205, 79, 57);
    TOMATO4, (139, 54, 38);
    ORANGERED1, (255, 69, 0);
    ORANGERED2, (238, 64, 0);
    ORANGERED3, (205, 55, 0);
    ORANGERED4, (139, 37, 0);
    RED1, (255, 0, 0);
    RED2, (238, 0, 0);
    RED3, (205, 0, 0);
    RED4, (139, 0, 0);
    DEEPPINK1, (255, 20, 147);
    DEEPPINK2, (238, 18, 137);
    DEEPPINK3, (205, 16, 118);
    DEEPPINK4, (139, 10, 80);
    HOTPINK1, (255, 110, 180);
    HOTPINK2, (238, 106, 167);
    HOTPINK3, (205, 96, 144);
    HOTPINK4, (139, 58, 98);
    PINK1, (255, 181, 197);
    PINK2, (238, 169, 184);
    PINK3, (205, 145, 158);
    PINK4, (139, 99, 108);
    LIGHTPINK1, (255, 174, 185);
    LIGHTPINK2, (238, 162, 173);
    LIGHTPINK3, (205, 140, 149);
    LIGHTPINK4, (139, 95, 101);
    PALEVIOLETRED1, (255, 130, 171);
    PALEVIOLETRED2, (238, 121, 159);
    PALEVIOLETRED3, (205, 104, 137);
    PALEVIOLETRED4, (139, 71, 93);
    MAROON1, (255, 52, 179);
    MAROON2, (238, 48, 167);
    MAROON3, (205, 41, 144);
    MAROON4, (139, 28, 98);
    VIOLETRED1, (255, 62, 150);
    VIOLETRED2, (238, 58, 140);
    VIOLETRED3, (205, 50, 120);
    VIOLETRED4, (139, 34, 82);
    MAGENTA1, (255, 0, 255);
    MAGENTA2, (238, 0, 238);
    MAGENTA3, (205, 0, 205);
    MAGENTA4, (139, 0, 139);
    ORCHID1, (255, 131, 250);
    ORCHID2, (238, 122, 233);
    ORCHID3, (205, 105, 201);
    ORCHID4, (139, 71, 137);
    PLUM1, (255, 187, 255);
    PLUM2, (238, 174, 238);
    PLUM3, (205, 150, 205);
    PLUM4, (139, 102, 139);
    MEDIUMORCHID1, (224, 102, 255);
    MEDIUMORCHID2, (209, 95, 238);
    MEDIUMORCHID3, (180, 82, 205);
    MEDIUMORCHID4, (122, 55, 139);
    DARKORCHID1, (191, 62, 255);
    DARKORCHID2, (178, 58, 238);
    DARKORCHID3, (154, 50, 205);
    DARKORCHID4, (104, 34, 139);
    PURPLE1, (155, 48, 255);
    PURPLE2, (145, 44, 238);
    PURPLE3, (125, 38, 205);
    PURPLE4, (85, 26, 139);
    MEDIUMPURPLE1, (171, 130, 255);
    MEDIUMPURPLE2, (159, 121, 238);
    MEDIUMPURPLE3, (137, 104, 205);
    MEDIUMPURPLE4, (93, 71, 139);
    THISTLE1, (255, 225, 255);
    THISTLE2, (238, 210, 238);
    THISTLE3, (205, 181, 205);
    THISTLE4, (139, 123, 139);
    DARK_GREY, (169, 169, 169);
    DARKGREY, (169, 169, 169);
    DARK_GRAY, (169, 169, 169);
    DARKGRAY, (169, 169, 169);
    DARK_BLUE, (0, 0, 139);
    DARKBLUE, (0, 0, 139);
    DARK_CYAN, (0, 139, 139);
    DARKCYAN, (0, 139, 139);
    DARK_MAGENTA, (139, 0, 139);
    DARKMAGENTA, (139, 0, 139);
    DARK_RED, (139, 0, 0);
    DARKRED, (139, 0, 0);
    LIGHT_GREEN, (144, 238, 144);
    LIGHTGREEN, (144, 238, 144);
    CRIMSON, (220, 20, 60);
    INDIGO, (75, 0, 130);
    OLIVE, (128, 128, 0);
    REBECCA_PURPLE, (102, 51, 153);
    REBECCAPURPLE, (102, 51, 153);
    SILVER, (192, 192, 192);
    TEAL, (0, 128, 128);
}
