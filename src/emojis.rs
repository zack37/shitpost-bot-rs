use serde_json::json;
use serenity::model::{
    channel::ReactionType,
    guild::{Emoji, Role},
    id::EmojiId,
};

fn create_emoji(id: EmojiId, name: &str, animated: bool) -> Emoji {
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

pub fn format_emoji(emoji: &Emoji) -> String {
    let animated_flag = if emoji.animated { "a" } else { "" };
    let f = format!("<{}:{}:{}>", animated_flag, emoji.name, emoji.id);

    f
}

pub fn zack() -> Emoji {
    create_emoji(EmojiId(401543766084943892), "zack", false)
}

pub fn maga() -> Emoji {
    create_emoji(EmojiId(423325774674788352), "maga", false)
}

pub fn trumpgasm() -> Emoji {
    create_emoji(EmojiId(416058928502276106), "trumpgasm", false)
}

pub fn bepsi() -> Emoji {
    create_emoji(EmojiId(410166385918869504), "bepsi", false)
}

pub fn gkappa() -> Emoji {
    create_emoji(EmojiId(423160844650676244), "gkappa", false)
}

pub fn lets_go() -> Emoji {
    create_emoji(EmojiId(436328704059113475), "letsgo", false)
}

pub fn gzack() -> Emoji {
    create_emoji(EmojiId(610223670123560970), "gzack", false)
}

pub fn party_parrot() -> Emoji {
    create_emoji(EmojiId(397874122232954901), "party_parrot", true)
}

pub fn ultra_fast_parrot() -> Emoji {
    create_emoji(EmojiId(397874139848769557), "ultrafastparrot", true)
}

pub fn parrot_wave_7() -> Emoji {
    create_emoji(EmojiId(397874137529319425), "parrotwave7", true)
}

pub fn parrot_wave_6() -> Emoji {
    create_emoji(EmojiId(397874138959839233), "parrotwave6", true)
}

pub fn parrot_wave_5() -> Emoji {
    create_emoji(EmojiId(397874134929113088), "parrotwave5", true)
}

pub fn parrot_wave_4() -> Emoji {
    create_emoji(EmojiId(397874133523890178), "parrotwave4", true)
}

pub fn parrot_wave_3() -> Emoji {
    create_emoji(EmojiId(397874131539853322), "parrotwave3", true)
}

pub fn parrot_wave_2() -> Emoji {
    create_emoji(EmojiId(397874132664188930), "parrotwave2", true)
}

pub fn parrot_wave_1() -> Emoji {
    create_emoji(EmojiId(397874130185093131), "parrotwave1", true)
}

pub fn wendy_parrot() -> Emoji {
    create_emoji(EmojiId(399242434300870658), "wendyparrot", true)
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
