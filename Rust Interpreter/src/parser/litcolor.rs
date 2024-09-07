use super::*;
#[derive(Debug)]

pub struct LiteralColorState{
    wsf: Vec<u8>
}

enum LitColorFoundResult{
    Found,
    FoundOnLast,
    CouldFind,
    Failed
}

impl ParseState for LiteralColorState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        let value = get_color_word(&self.wsf,word.str);
        match value {
            LitColorFoundResult::Found => {
                env.exprs.vec[env.index] = Expr::LitCol {
                    str_start: word.pos + env.global_index,
                    str_length: word.len(),
                    value: word.str.to_owned()
                };
                MatchResult::Matched(rest.pos, false)
            },
            LitColorFoundResult::CouldFind => {
                env.exprs.vec[env.index] = Expr::LitCol {
                    str_start: word.pos + env.global_index,
                    str_length: usize::MAX,
                    value: Vec::new()
                };
                self.wsf.append(&mut word.str.to_owned().to_ascii_lowercase());
                MatchResult::Continue
            },
            LitColorFoundResult::Failed => {
                MatchResult::Failed
            },
            LitColorFoundResult::FoundOnLast => {
                unreachable!();
            }
        }
    }

    fn step_match(
            &mut self,
            env: &mut Environment,
            _child: Option<usize>,
            word: &Slice,
            rest: &Slice,
        ) -> MatchResult {
            let value = get_color_word(&self.wsf,word.str);
            match value {
                //Finishes a color name
                LitColorFoundResult::Found => {
                    if let Expr::LitCol { str_start, str_length, value } 
                    = &mut env.exprs.vec[env.index] {
                        if let Some(len) = (word.pos + word.len() + env.global_index).checked_sub(*str_start) {
                            *str_length = len;
                            self.wsf.append(&mut word.str.to_owned().to_ascii_lowercase());
                            *value = self.wsf.to_owned();
                        }else{
                            panic!("Expression is ending before it started!")
                        }
                    }else{
                        unreachable!()
                    }
                    MatchResult::Matched(rest.pos, false)
                },
                //Last word could have had more color words after it but didn't.
                LitColorFoundResult::FoundOnLast => {
                    if let Expr::LitCol { str_start, str_length, value } 
                    = &mut env.exprs.vec[env.index] {
                        if let Some(len) = (word.pos + env.global_index).checked_sub(*str_start) {
                            *str_length = len;
                            *value = self.wsf.to_owned();
                        }else{
                            panic!("Expression is ending before it started!")
                        }
                    }else{
                        unreachable!()
                    }
                    MatchResult::Matched(word.pos, false)
                },
                //Beginning of a color name, keep searching
                LitColorFoundResult::CouldFind => {
                    self.wsf.append(&mut word.str.to_owned().to_ascii_lowercase());
                    MatchResult::Continue
                },
                //Cannot be a color
                LitColorFoundResult::Failed => {
                    MatchResult::Failed
                }
            }
    }

    fn get_name(&self) -> &'static str {
        "ColorLit"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl LiteralColorState {
    pub fn new() -> Self {
        Self {
            wsf: Vec::new()
        }
    }
}

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
            | b"sea" | b"sky" | b"slate" | b"spring" | b"steel" | b"white" | b"yellow"
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
            _ => LitColorFoundResult::Failed      
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
        b"lime" => match cleaned_word {
            b"green" => LitColorFoundResult::Found,
            _ => LitColorFoundResult::FoundOnLast
        }
        b"medium" => match cleaned_word {
            b"blue" | b"orchid" | b"purple" | b"seagreen" | b"slateblue" | b"springgreen"
            | b"turquoise" | b"violetred" 
                => LitColorFoundResult::Found,
            b"sea" | b"slate" | b"spring" | b"violet"
                => LitColorFoundResult::CouldFind,
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
        }
        _ => LitColorFoundResult::Failed
    }
}