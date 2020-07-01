use serenity::model::id::UserId;
use std::collections::HashMap;

lazy_static! {
    static ref USER_ID_MAP: HashMap<&'static str, UserId> = {
        let mut m = HashMap::with_capacity(6);

        m.insert("aaron", UserId(65055432095301632));
        m.insert("bacon", UserId(108352053692125184));
        m.insert("jerran", UserId(189006310501646336));
        m.insert("mee6", UserId(159985870458322944));
        m.insert("rizo", UserId(100758264047747072));
        m.insert("zack", UserId(108568431053246464));

        m
    };
}

pub fn aaron() -> UserId {
    USER_ID_MAP["aaron"]
}

pub fn bacon() -> UserId {
    USER_ID_MAP["bacon"]
}

pub fn jerran() -> UserId {
    USER_ID_MAP["jerran"]
}

pub fn _mee_6() -> UserId {
    USER_ID_MAP["mee6"]
}

pub fn rizo() -> UserId {
    USER_ID_MAP["rizo"]
}

pub fn zack() -> UserId {
    USER_ID_MAP["zack"]
}
