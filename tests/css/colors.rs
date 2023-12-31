use domrs::CssColor;

#[test]
fn display_should_work() {
  assert_eq!("aliceblue", CssColor::AliceBlue.to_string());
  assert_eq!("antiquewhite", CssColor::AntiqueWhite.to_string());
  assert_eq!("aqua", CssColor::Aqua.to_string());
  assert_eq!("aquamarine", CssColor::Aquamarine.to_string());
  assert_eq!("azure", CssColor::Azure.to_string());
  assert_eq!("beige", CssColor::Beige.to_string());
  assert_eq!("bisque", CssColor::Bisque.to_string());
  assert_eq!("black", CssColor::Black.to_string());
  assert_eq!("blanchedalmond", CssColor::BlanchedAlmond.to_string());
  assert_eq!("blue", CssColor::Blue.to_string());
  assert_eq!("blueviolet", CssColor::BlueViolet.to_string());
  assert_eq!("brown", CssColor::Brown.to_string());
  assert_eq!("burlywood", CssColor::BurlyWood.to_string());
  assert_eq!("cadetblue", CssColor::CadetBlue.to_string());
  assert_eq!("chartreuse", CssColor::Chartreuse.to_string());
  assert_eq!("chocolate", CssColor::Chocolate.to_string());
  assert_eq!("coral", CssColor::Coral.to_string());
  assert_eq!("cornflowerblue", CssColor::CornflowerBlue.to_string());
  assert_eq!("cornsilk", CssColor::Cornsilk.to_string());
  assert_eq!("crimson", CssColor::Crimson.to_string());
  assert_eq!("cyan", CssColor::Cyan.to_string());
  assert_eq!("darkblue", CssColor::DarkBlue.to_string());
  assert_eq!("darkcyan", CssColor::DarkCyan.to_string());
  assert_eq!("darkgoldenrod", CssColor::DarkGoldenRod.to_string());
  assert_eq!("darkgray", CssColor::DarkGray.to_string());
  assert_eq!("darkgreen", CssColor::DarkGreen.to_string());
  assert_eq!("darkgrey", CssColor::DarkGrey.to_string());
  assert_eq!("darkkhaki", CssColor::DarkKhaki.to_string());
  assert_eq!("darkmagenta", CssColor::DarkMagenta.to_string());
  assert_eq!("darkolivegreen", CssColor::DarkOliveGreen.to_string());
  assert_eq!("darkorange", CssColor::DarkOrange.to_string());
  assert_eq!("darkorchid", CssColor::DarkOrchid.to_string());
  assert_eq!("darkred", CssColor::DarkRed.to_string());
  assert_eq!("darksalmon", CssColor::DarkSalmon.to_string());
  assert_eq!("darkseagreen", CssColor::DarkSeaGreen.to_string());
  assert_eq!("darkslateblue", CssColor::DarkSlateBlue.to_string());
  assert_eq!("darkslategray", CssColor::DarkSlateGray.to_string());
  assert_eq!("darkslategrey", CssColor::DarkSlateGrey.to_string());
  assert_eq!("darkturquoise", CssColor::DarkTurquoise.to_string());
  assert_eq!("darkviolet", CssColor::DarkViolet.to_string());
  assert_eq!("deeppink", CssColor::DeepPink.to_string());
  assert_eq!("deepskyblue", CssColor::DeepSkyBlue.to_string());
  assert_eq!("dimgray", CssColor::DimGray.to_string());
  assert_eq!("dimgrey", CssColor::DimGrey.to_string());
  assert_eq!("dodgerblue", CssColor::DodgerBlue.to_string());
  assert_eq!("firebrick", CssColor::FireBrick.to_string());
  assert_eq!("floralwhite", CssColor::FloralWhite.to_string());
  assert_eq!("forestgreen", CssColor::ForestGreen.to_string());
  assert_eq!("fuchsia", CssColor::Fuchsia.to_string());
  assert_eq!("gainsboro", CssColor::Gainsboro.to_string());
  assert_eq!("ghostwhite", CssColor::GhostWhite.to_string());
  assert_eq!("gold", CssColor::Gold.to_string());
  assert_eq!("goldenrod", CssColor::GoldenRod.to_string());
  assert_eq!("gray", CssColor::Gray.to_string());
  assert_eq!("green", CssColor::Green.to_string());
  assert_eq!("greenyellow", CssColor::GreenYellow.to_string());
  assert_eq!("grey", CssColor::Grey.to_string());
  assert_eq!("honeydew", CssColor::HoneyDew.to_string());
  assert_eq!("hotpink", CssColor::HotPink.to_string());
  assert_eq!("indianred", CssColor::IndianRed.to_string());
  assert_eq!("indigo", CssColor::Indigo.to_string());
  assert_eq!("ivory", CssColor::Ivory.to_string());
  assert_eq!("khaki", CssColor::Khaki.to_string());
  assert_eq!("lavender", CssColor::Lavender.to_string());
  assert_eq!("lavenderblush", CssColor::LavenderBlush.to_string());
  assert_eq!("lawngreen", CssColor::LawnGreen.to_string());
  assert_eq!("lemonchiffon", CssColor::LemonChiffon.to_string());
  assert_eq!("lightblue", CssColor::LightBlue.to_string());
  assert_eq!("lightcoral", CssColor::LightCoral.to_string());
  assert_eq!("lightcyan", CssColor::LightCyan.to_string());
  assert_eq!("lightgoldenrodyellow", CssColor::LightGoldenRodYellow.to_string());
  assert_eq!("lightgray", CssColor::LightGray.to_string());
  assert_eq!("lightgreen", CssColor::LightGreen.to_string());
  assert_eq!("lightgrey", CssColor::LightGrey.to_string());
  assert_eq!("lightpink", CssColor::LightPink.to_string());
  assert_eq!("lightsalmon", CssColor::LightSalmon.to_string());
  assert_eq!("lightseagreen", CssColor::LightSeaGreen.to_string());
  assert_eq!("lightskyblue", CssColor::LightSkyBlue.to_string());
  assert_eq!("lightslategray", CssColor::LightSlateGray.to_string());
  assert_eq!("lightslategrey", CssColor::LightSlateGrey.to_string());
  assert_eq!("lightsteelblue", CssColor::LightSteelBlue.to_string());
  assert_eq!("lightyellow", CssColor::LightYellow.to_string());
  assert_eq!("lime", CssColor::Lime.to_string());
  assert_eq!("limegreen", CssColor::LimeGreen.to_string());
  assert_eq!("linen", CssColor::Linen.to_string());
  assert_eq!("magenta", CssColor::Magenta.to_string());
  assert_eq!("maroon", CssColor::Maroon.to_string());
  assert_eq!("mediumaquamarine", CssColor::MediumAquaMarine.to_string());
  assert_eq!("mediumblue", CssColor::MediumBlue.to_string());
  assert_eq!("mediumorchid", CssColor::MediumOrchid.to_string());
  assert_eq!("mediumpurple", CssColor::MediumPurple.to_string());
  assert_eq!("mediumseagreen", CssColor::MediumSeaGreen.to_string());
  assert_eq!("mediumslateblue", CssColor::MediumSlateBlue.to_string());
  assert_eq!("mediumspringgreen", CssColor::MediumSpringGreen.to_string());
  assert_eq!("mediumturquoise", CssColor::MediumTurquoise.to_string());
  assert_eq!("mediumvioletred", CssColor::MediumVioletRed.to_string());
  assert_eq!("midnightblue", CssColor::MidnightBlue.to_string());
  assert_eq!("mintcream", CssColor::MintCream.to_string());
  assert_eq!("mistyrose", CssColor::MistyRose.to_string());
  assert_eq!("moccasin", CssColor::Moccasin.to_string());
  assert_eq!("navajowhite", CssColor::NavajoWhite.to_string());
  assert_eq!("navy", CssColor::Navy.to_string());
  assert_eq!("oldlace", CssColor::OldLace.to_string());
  assert_eq!("olive", CssColor::Olive.to_string());
  assert_eq!("olivedrab", CssColor::OliveDrab.to_string());
  assert_eq!("orange", CssColor::Orange.to_string());
  assert_eq!("orangered", CssColor::OrangeRed.to_string());
  assert_eq!("orchid", CssColor::Orchid.to_string());
  assert_eq!("palegoldenrod", CssColor::PaleGoldenRod.to_string());
  assert_eq!("palegreen", CssColor::PaleGreen.to_string());
  assert_eq!("paleturquoise", CssColor::PaleTurquoise.to_string());
  assert_eq!("palevioletred", CssColor::PaleVioletRed.to_string());
  assert_eq!("papayawhip", CssColor::PapayaWhip.to_string());
  assert_eq!("peachpuff", CssColor::PeachPuff.to_string());
  assert_eq!("peru", CssColor::Peru.to_string());
  assert_eq!("pink", CssColor::Pink.to_string());
  assert_eq!("plum", CssColor::Plum.to_string());
  assert_eq!("powderblue", CssColor::PowderBlue.to_string());
  assert_eq!("purple", CssColor::Purple.to_string());
  assert_eq!("rebeccapurple", CssColor::RebeccaPurple.to_string());
  assert_eq!("red", CssColor::Red.to_string());
  assert_eq!("rosybrown", CssColor::RosyBrown.to_string());
  assert_eq!("royalblue", CssColor::RoyalBlue.to_string());
  assert_eq!("saddlebrown", CssColor::SaddleBrown.to_string());
  assert_eq!("salmon", CssColor::Salmon.to_string());
  assert_eq!("sandybrown", CssColor::SandyBrown.to_string());
  assert_eq!("seagreen", CssColor::SeaGreen.to_string());
  assert_eq!("seashell", CssColor::SeaShell.to_string());
  assert_eq!("sienna", CssColor::Sienna.to_string());
  assert_eq!("silver", CssColor::Silver.to_string());
  assert_eq!("skyblue", CssColor::SkyBlue.to_string());
  assert_eq!("slateblue", CssColor::SlateBlue.to_string());
  assert_eq!("slategray", CssColor::SlateGray.to_string());
  assert_eq!("slategrey", CssColor::SlateGrey.to_string());
  assert_eq!("snow", CssColor::Snow.to_string());
  assert_eq!("springgreen", CssColor::SpringGreen.to_string());
  assert_eq!("steelblue", CssColor::SteelBlue.to_string());
  assert_eq!("tan", CssColor::Tan.to_string());
  assert_eq!("teal", CssColor::Teal.to_string());
  assert_eq!("thistle", CssColor::Thistle.to_string());
  assert_eq!("tomato", CssColor::Tomato.to_string());
  assert_eq!("transparent", CssColor::Transparent.to_string());
  assert_eq!("turquoise", CssColor::Turquoise.to_string());
  assert_eq!("violet", CssColor::Violet.to_string());
  assert_eq!("wheat", CssColor::Wheat.to_string());
  assert_eq!("white", CssColor::White.to_string());
  assert_eq!("whitesmoke", CssColor::WhiteSmoke.to_string());
  assert_eq!("yellow", CssColor::Yellow.to_string());
  assert_eq!("yellowgreen", CssColor::YellowGreen.to_string());
  assert_eq!("#0000EF", CssColor::Hex(0xef).to_string());
  assert_eq!("#EFEFEF", CssColor::Hex(0xefefef).to_string());
  assert_eq!("#EFEFEF", CssColor::Hex(0xEFEFEF).to_string());
  assert_eq!("#FFFFFF", CssColor::Hex(0xffffffff).to_string());
  assert_eq!("#FFFFFF", CssColor::Hex(0xFFFFFFFF).to_string());
}

#[test]
fn clone_should_work() {
  assert_eq!(CssColor::White.clone().to_string(), CssColor::White.to_string());
}
