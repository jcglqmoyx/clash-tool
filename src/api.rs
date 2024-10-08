pub mod mail_tm {
    pub const GET_DOMAIN_API: &str = "https://api.mail.tm/domains";
    pub const CREATE_ACCOUNT_API: &str = "https://api.mail.tm/accounts";
    pub const ACCESS_TOKEN_API: &str = "https://api.mail.tm/token";
    pub const GET_MESSAGE_API: &str = "https://api.mail.tm/messages";
}

pub mod gou {
    pub const MAIL_VERIFICATION_CODE_API: &str = "https://jiasugou.me/auth/send";
    pub const REGISTRATION_API: &str = "https://jiasugou.me/auth/register";
    pub const LOGIN_API: &str = "https://jiasugou.me/auth/login";
    pub const USER_PROFILE_API: &str = "https://jiasugou.me/user";
}

pub mod xfx_ssr {
    pub const REGISTRATION_API: &str = "https://xfxssr.top/api/v1/passport/auth/register";
    pub const LOGIN_API: &str = "https://xfxssr.top/api/v1/passport/auth/login";
    pub const SUBSCRIPTION_LINK_PREFIX: &str = "http://xfxssr.info/api/v1/client/subscribe?token=";
}

pub mod wall {
    pub const MAIL_VERIFICATION_CODE_API: &str = "https://www.qlgq.top/auth/send";
    pub const REGISTRATION_API: &str = "https://www.qlgq.top/auth/register";
    pub const LOGIN_API: &str = "https://www.qlgq.top/auth/login";
    pub const USER_PROFILE_API: &str = "https://www.qlgq.top/user";
}