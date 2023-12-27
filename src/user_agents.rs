use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static USER_AGENTS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [
        ("firefox", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:97.0) Gecko/20100101 Firefox/97.0"),
        ("chrome", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36"),
        ("edge", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36 Edg/98.0.1108.56"),
        ("safari", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0.3 Safari/605.1.15"),
        ("opera", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36 OPR/85.0.4252.0"),
        ("ie11", "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; AS; rv:11.0) like Gecko"),
        ("android", "Mozilla/5.0 (Linux; Android 11; SM-G960U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.101 Mobile Safari/537.36"),
        ("ios", "Mozilla/5.0 (iPhone; CPU iPhone OS 15_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.3 Mobile/15E148 Safari/604.1"),
        ("edge_android", "Mozilla/5.0 (Linux; Android 11; SM-G960U) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.101 Mobile Safari/537.36 EdgA/47.13.4.5140"),
        ("opera_mini", "Mozilla/5.0 (Linux; U; Android 11; en-US; SM-G960U) AppleWebKit/36.0.1985.137 Mobile Safari/36.0.1985.137 Opera Mini/62.3.2254/62.4057"),
        ("uc_browser", "Mozilla/5.0 (Linux; U; Android 11; en-US; SM-G960U) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 UCBrowser/13.5.2.1313 U3/0.8.0 Mobile Safari/534.30"),
        ("blackberry", "Mozilla/5.0 (BB10; Touch) AppleWebKit/537.10+ (KHTML, like Gecko) Version/10.1.0.4181 Mobile Safari/537.10+"),
        ("mozilla", "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"),
        ("bingbot", "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)"),
        ("yahoo_slurp", "Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)"),
        ("duckduckgo", "Mozilla/5.0 (compatible; DuckDuckBot/1.0; +http://duckduckgo.com)"),
        ("baiduspider", "Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)"),
        ("yandexbot", "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)"),
        ("ahrefsbot", "Mozilla/5.0 (compatible; AhrefsBot/7.0; +http://ahrefs.com/robot/)"),
        ("mj12bot", "Mozilla/5.0 (compatible; MJ12bot/v1.4.8; http://www.majestic12.co.uk/bot.php?+)"),
        ("semrushbot", "Mozilla/5.0 (compatible; SemrushBot/7~bl; +http://www.semrush.com/bot.html)"),
        ("sogou_spider", "Sogou web spider/4.0(+http://www.sogou.com/docs/help/webmasters.htm#07)"),
        ("exabot", "Mozilla/5.0 (compatible; Exabot/3.0; +http://www.exabot.com/go/robot)"),
        ("dotbot", "DotBot/1.1 http://www.dotnetdotcom.org/"),
        ("facebook", "facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)"),
        ("pinterest", "Pinterest/0.1 +http://www.pinterest.com/"),
        ("slackbot", "Slackbot-LinkExpanding 1.0 (+https://api.slack.com/robots)"),
        ("discord", "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)"),
        ("zoom", "ZoominfoBot (zoominfobot at zoominfo dot com)"),
        ("whatsapp", "WhatsApp/2.21.11.13 i"),
        ("applebot", "Applebot/0.1; +http://www.apple.com/go/applebot)"),
        ("flipboard", "Flipboard/3.3.25 CFNetwork/711.3.18 Darwin/14.0.0"),
        ("outlook", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:78.0) Gecko/20100101 Firefox/78.0 Outlook/1.0;"),
    ]
    .iter()
    .cloned()
    .collect()
});
