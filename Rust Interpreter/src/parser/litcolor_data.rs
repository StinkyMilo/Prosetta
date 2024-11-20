pub enum LitColorFoundResult {
    ///found a color
    Found,
    ///found color on last word
    FoundOnLast,
    ///may find complete color on next word
    CouldFind,
    ///failed to find color
    Failed,
}

#[rustfmt::skip]
pub fn get_color_word(word_so_far: &[u8], word: &[u8]) -> LitColorFoundResult {
    let cleaned_word = &*word.to_ascii_lowercase();
    match &*word_so_far {
        b"" => match cleaned_word {
            b"black" | b"silver" | b"gray" | b"maroon" | b"red" | b"purple" | b"fuchsia"
            | b"navy" | b"teal"
            | b"aquamarine" | b"azure" | b"beige" | b"bisque" | b"brown" | b"chartreuse" | b"chocolate"
            | b"coral" | b"crimson" | b"cornsilk" | b"cyan" | b"firebrick" | b"gainsboro"
            | b"gold" | b"goldenrod" | b"grey" | b"honeydew" | b"indigo" | b"ivory" | b"khaki" 
            | b"lawngreen" | b"linen" | b"magenta" | b"moccasin" | b"orchid"
            | b"aliceblue" | b"antiquewhite" | b"blanchedalmond" | b"blueviolet" | b"burlywood"
            | b"cadetblue" | b"cornflowerblue" | b"darkblue" | b"darkcyan" | b"darkgoldenrod" 
            | b"darkgray" | b"darkgreen" | b"darkgrey" | b"darkkhaki" | b"darkmagenta"
            | b"darkolivegreen" | b"darkorange" | b"darkorchid" | b"darkred" | b"darksalmon"
            | b"darkseagreen" | b"darkslateblue" | b"darkslategray" | b"darkslategrey" | b"darkturquoise"
            | b"darkviolet" | b"deeppink" | b"deepskyblue" | b"dimgray" | b"dimgrey" | b"dodgerblue"
            | b"floralwhite" | b"forestgreen" | b"ghostwhite" | b"greenyellow"
            | b"hotpink" | b"indianred" | b"lavenderblush" | b"lemonchiffon"
            | b"lightblue" | b"lightcoral" | b"lightcyan" | b"lightgoldenrodyellow" | b"lightgray"
            | b"lightgreen" | b"lightgrey" | b"lightpink" | b"lightsalmon" | b"lightseagreen"
            | b"lightskyblue" | b"lightslategray" | b"lightslategrey" | b"lightsteelblue" | b"lightyellow"
            | b"limegreen" | b"mediumaquamarine" | b"mediumblue" | b"mediumorchid" | b"mediumpurple"
            | b"mediumseagreen" | b"mediumslateblue" | b"mediumspringgreen" | b"mediumturquoise"
            | b"mediumvioletred" | b"midnightblue" | b"mintcream" | b"mistyrose" | b"navajowhite"
            | b"oldlace" | b"olivedrab" | b"orangered" | b"palegoldenrod" | b"palegreen" | b"paleturquoise"
            | b"palevioletred" | b"papayawhip" | b"peachpuff" | b"peru" | b"pink" | b"plum" | b"powderblue"
            | b"rebeccapurple" | b"rosybrown" | b"royalblue" | b"saddlebrown" | b"salmon"
            | b"sandybrown" | b"seagreen" | b"seashell" | b"sienna" | b"skyblue" | b"slateblue"
            | b"slategray" | b"slategrey" | b"snow" | b"springgreen" | b"steelblue" | b"tan"
            | b"thistle" | b"tomato" | b"transparent" | b"turquoise" | b"violet" | b"wheat"
            | b"whitesmoke" | b"yellowgreen"
                => LitColorFoundResult::Found,
            b"alice" | b"antique" | b"blanched" | b"blue" | b"burly" | b"cadet" | b"cornflower"
            | b"corn" | b"dark" | b"deep" | b"dim" | b"dodger" | b"fire" | b"floral" | b"forest"
            | b"ghost" | b"golden" | b"green" | b"honey" | b"hot" | b"indian" | b"lavender"
            | b"lawn" | b"lemon" | b"light" | b"lime" | b"medium" | b"midnight" | b"mint"
            | b"misty" | b"navajo" | b"old" | b"olive" | b"orange" | b"pale" | b"papaya"
            | b"peach" | b"powder" | b"rebecca" | b"rosy" | b"royal" | b"saddle" | b"sandy"
            | b"sea" | b"sky" | b"slate" | b"spring" | b"steel" | b"white" | b"yellow" | b"aqua"
            | b"darkgolden" | b"darkolive" | b"darksea" | b"darkslate" | b"deepsky"
            | b"lightgolden" | b"lightgoldenrod" | b"lightsea" | b"lightsky" | b"lightslate"
            | b"mediumaqua" | b"mediumsea" | b"mediumslate" | b"mediumspring"
            | b"mediumviolet" | b"palegolden" | b"paleviolet" | b"lightsteel"
                => LitColorFoundResult::CouldFind,
            _ => LitColorFoundResult::Failed
        },
        b"alice" =>  match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"antique" =>  match cleaned_word {
            b"white" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"aqua" => match cleaned_word {
            b"marine" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"blanched" =>  match cleaned_word {
            b"almond" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"blue" =>  match cleaned_word {
            b"violet" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"burly" =>  match cleaned_word {
            b"wood" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"cadet" =>  match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"cornflower" =>  match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"corn" => match cleaned_word {
            b"flower" => LitColorFoundResult::CouldFind,
            b"silk" => LitColorFoundResult::Found,
            b"flowerblue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"dark" => match cleaned_word {
            b"blue" | b"cyan" | b"goldenrod" | b"gray" | b"green" | b"grey" | b"khaki"
            | b"magenta" | b"olivegreen" | b"orange" | b"orchid" | b"red" | b"salmon"
            | b"seagreen" | b"slateblue" | b"slategray" | b"slategrey" | b"turquoise"
            | b"violet"
                => LitColorFoundResult::Found,
            b"golden" | b"olive" | b"sea" | b"slate"
                => LitColorFoundResult::CouldFind,
            _ => LitColorFoundResult::Failed
        },
        b"deep" => match cleaned_word {
            b"pink" | b"skyblue" => LitColorFoundResult::Found,
            b"sky" => LitColorFoundResult::CouldFind,
            _ => LitColorFoundResult::Failed
        },
        b"dim" => match cleaned_word {
            b"gray" | b"grey" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"dodger" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"fire" => match cleaned_word {
            b"brick" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"floral" => match cleaned_word {
            b"white" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"forest" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"ghost" => match cleaned_word { 
            b"white" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"golden" => match cleaned_word {
            b"rod" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"green" =>  match cleaned_word {
            b"yellow" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"honey" => match cleaned_word {
            b"dew" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"hot" => match cleaned_word {
            b"pink" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"indian" => match cleaned_word {
            b"red" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lavender" => match cleaned_word {
            b"blush" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"lawn" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lemon" => match cleaned_word {
            b"chiffon" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"light" => match cleaned_word {
            b"blue" | b"coral" | b"cyan" | b"goldenrodyellow" | b"gray" | b"green" | b"grey"
            | b"pink" | b"salmon" | b"seagreen" | b"skyblue" | b"slategray" | b"slategrey"
            | b"steelblue" | b"yellow" 
                => LitColorFoundResult::Found,
            b"goldenrod" | b"golden" | b"sea" | b"sky" | b"slate" | b"steel"
                => LitColorFoundResult::CouldFind,
            _ => LitColorFoundResult::Failed
        },
        b"steel" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lime" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"medium" => match cleaned_word {
            b"blue" | b"orchid" | b"purple" | b"seagreen" | b"slateblue" | b"springgreen"
            | b"turquoise" | b"violetred"  | b"aquamarine"
                => LitColorFoundResult::Found,
            b"sea" | b"slate" | b"spring" | b"violet" | b"aqua"
                => LitColorFoundResult::CouldFind,
            _ => LitColorFoundResult::Failed
        },
        b"spring" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"midnight" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"mint" => match cleaned_word {
            b"cream" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"misty" => match cleaned_word {
            b"rose" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"navajo" => match cleaned_word {
            b"white" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"old" => match cleaned_word {
            b"lace" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"olive" => match cleaned_word {
            b"drab" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"orange" => match cleaned_word {
            b"red" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"pale" => match cleaned_word {
            b"goldenrod" | b"green" | b"turquoise" | b"violetred"
                => LitColorFoundResult::Found,
            b"golden" | b"violet"
                => LitColorFoundResult::CouldFind,
            _ => LitColorFoundResult::Failed
        },
        b"papaya" => match cleaned_word {
            b"whip" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"peach" => match cleaned_word {
            b"puff" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"powder" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"rebecca" => match cleaned_word {
            b"purple" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"darkgolden" => match cleaned_word {
            b"rod" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"darkolive" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"darksea" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"darkslate" => match cleaned_word {
            b"blue" | b"gray" | b"grey" 
                => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        }
        b"deepsky" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lightgolden" => match cleaned_word {
            b"rodyellow" => LitColorFoundResult::Found,
            b"rod" => LitColorFoundResult::CouldFind,
            _ => LitColorFoundResult::Failed
        },
        b"lightgoldenrod" => match cleaned_word {
            b"yellow" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lightsea" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lightsky" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lightslate" => match cleaned_word {
            b"gray" | b"grey" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"lightsteel" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"mediumsea" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"mediumslate" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"mediumspring" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"mediumviolet" => match cleaned_word {
            b"red" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"mediumaqua" => match cleaned_word {
            b"marine" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        }
        b"palegolden" => match cleaned_word {
            b"rod" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"paleviolet" => match cleaned_word {
            b"red" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"rosy" => match cleaned_word {
            b"brown" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"royal" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"saddle" => match cleaned_word {
            b"brown" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"sandy" => match cleaned_word {
            b"brown" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"sea" => match cleaned_word {
            b"green" | b"shell"
                => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"sky" => match cleaned_word {
            b"blue" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"slate" => match cleaned_word {
            b"blue" | b"gray" | b"grey"
                => LitColorFoundResult::Found,
            _ => LitColorFoundResult::Failed
        },
        b"white" => match cleaned_word {
            b"smoke" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        },
        b"yellow" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        }
        _ => unreachable!()
    }
}
