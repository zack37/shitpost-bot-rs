use serde_json::json;
use serenity::model::{
    channel::ReactionType,
    guild::{Emoji, Role},
    id::EmojiId,
};
use std::collections::HashMap;

lazy_static! {
    static ref EMOJI_MAP: HashMap<&'static str, Emoji> = {
        let mut m = HashMap::with_capacity(18);
        m.insert(
            "zack",
            create_emoji(EmojiId(401543766084943892), "zack", false),
        );
        m.insert(
            "maga",
            create_emoji(EmojiId(423325774674788352), "maga", false),
        );
        m.insert(
            "trumpgasm",
            create_emoji(EmojiId(416058928502276106), "trumpgasm", false),
        );
        m.insert(
            "bepsi",
            create_emoji(EmojiId(410166385918869504), "bepsi", false),
        );
        m.insert(
            "gkappa",
            create_emoji(EmojiId(423160844650676244), "gkappa", false),
        );
        m.insert(
            "letsgo",
            create_emoji(EmojiId(436328704059113475), "letsgo", false),
        );
        m.insert(
            "gzack",
            create_emoji(EmojiId(610223670123560970), "gzack", false),
        );
        m.insert(
            "party_parrot",
            create_emoji(EmojiId(397874122232954901), "party_parrot", true),
        );
        m.insert(
            "ultrafastparrot",
            create_emoji(EmojiId(397874139848769557), "ultrafastparrot", true),
        );
        m.insert(
            "parrotwave7",
            create_emoji(EmojiId(397874137529319425), "parrotwave7", true),
        );
        m.insert(
            "parrotwave6",
            create_emoji(EmojiId(397874138959839233), "parrotwave6", true),
        );
        m.insert(
            "parrotwave5",
            create_emoji(EmojiId(397874134929113088), "parrotwave5", true),
        );
        m.insert(
            "parrotwave4",
            create_emoji(EmojiId(397874133523890178), "parrotwave4", true),
        );
        m.insert(
            "parrotwave3",
            create_emoji(EmojiId(397874131539853322), "parrotwave3", true),
        );
        m.insert(
            "parrotwave2",
            create_emoji(EmojiId(397874132664188930), "parrotwave2", true),
        );
        m.insert(
            "parrotwave1",
            create_emoji(EmojiId(397874130185093131), "parrotwave1", true),
        );
        m.insert(
            "wendyparrot",
            create_emoji(EmojiId(399242434300870658), "wendyparrot", true),
        );

        m
    };
}

fn create_emoji(id: EmojiId, name: &'static str, animated: bool) -> Emoji {
    serde_json::from_value::<Emoji>(json!({
        "animated": animated,
        "id": id,
        "name": name,
        "managed": false,
        "require_colons": true,
        "roles": Vec::<Role>::new(),
    }))
    .unwrap()
}

pub fn zack() -> Emoji {
    EMOJI_MAP["zack"].clone()
}

pub fn maga() -> Emoji {
    EMOJI_MAP["maga"].clone()
}

pub fn trumpgasm() -> Emoji {
    EMOJI_MAP["trumpgasm"].clone()
}

pub fn bepsi() -> Emoji {
    EMOJI_MAP["bepsi"].clone()
}

pub fn gkappa() -> Emoji {
    EMOJI_MAP["gkappa"].clone()
}

pub fn lets_go() -> Emoji {
    EMOJI_MAP["letsgo"].clone()
}

pub fn gzack() -> Emoji {
    EMOJI_MAP["gzack"].clone()
}

pub fn party_parrot() -> Emoji {
    EMOJI_MAP["party_parrot"].clone()
}

pub fn ultra_fast_parrot() -> Emoji {
    EMOJI_MAP["ultrafastparrot"].clone()
}

pub fn parrot_wave_7() -> Emoji {
    EMOJI_MAP["parrotwave7"].clone()
}

pub fn parrot_wave_6() -> Emoji {
    EMOJI_MAP["parrotwave6"].clone()
}

pub fn parrot_wave_5() -> Emoji {
    EMOJI_MAP["parrotwave5"].clone()
}

pub fn parrot_wave_4() -> Emoji {
    EMOJI_MAP["parrotwave4"].clone()
}

pub fn parrot_wave_3() -> Emoji {
    EMOJI_MAP["parrotwave3"].clone()
}

pub fn parrot_wave_2() -> Emoji {
    EMOJI_MAP["parrotwave2"].clone()
}

pub fn parrot_wave_1() -> Emoji {
    EMOJI_MAP["parrotwave1"].clone()
}

pub fn wendy_parrot() -> Emoji {
    EMOJI_MAP["wendyparrot"].clone()
}

pub fn letter(key: char) -> ReactionType {
    let character = match key {
        'a' => "🇦",
        'b' => "🇧",
        'c' => "🇨",
        'd' => "🇩",
        'e' => "🇪",
        'f' => "🇫",
        'g' => "🇬",
        'h' => "🇭",
        'i' => "🇮",
        'j' => "🇯",
        'k' => "🇰",
        'l' => "🇱",
        'm' => "🇲",
        'n' => "🇳",
        'o' => "🇴",
        'p' => "🇵",
        'q' => "🇶",
        'r' => "🇷",
        's' => "🇸",
        't' => "🇹",
        'u' => "🇺",
        'v' => "🇻",
        'w' => "🇼",
        'x' => "🇽",
        'y' => "🇾",
        'z' => "🇿",
        _ => "",
    };

    ReactionType::Unicode(character.to_string())
}
