pub struct Language {
    pub code: &'static str,
    pub name: &'static str,
}

pub static LANGUAGES: &[Language] = &[
    Language {
        code: "af",
        name: "Afrikaans",
    },
    Language {
        code: "sq",
        name: "Albanian",
    },
    Language {
        code: "am",
        name: "Amharic",
    },
    Language {
        code: "ar",
        name: "Arabic",
    },
    Language {
        code: "hy",
        name: "Armenian",
    },
    Language {
        code: "az",
        name: "Azerbaijani",
    },
    Language {
        code: "eu",
        name: "Basque",
    },
    Language {
        code: "be",
        name: "Belarusian",
    },
    Language {
        code: "bn",
        name: "Bengali",
    },
    Language {
        code: "bs",
        name: "Bosnian",
    },
    Language {
        code: "bg",
        name: "Bulgarian",
    },
    Language {
        code: "ca",
        name: "Catalan",
    },
    Language {
        code: "ce",
        name: "Cebuano",
    },
    Language {
        code: "ny",
        name: "Chichewa",
    },
    Language {
        code: "zh",
        name: "Chinese",
    },
    Language {
        code: "co",
        name: "Corsican",
    },
    Language {
        code: "hr",
        name: "Croatian",
    },
    Language {
        code: "cs",
        name: "Czech",
    },
    Language {
        code: "da",
        name: "Danish",
    },
    Language {
        code: "eo",
        name: "Esperanto",
    },
    Language {
        code: "et",
        name: "Estonian",
    },
    Language {
        code: "tl",
        name: "Filipino",
    },
    Language {
        code: "fi",
        name: "Finnish",
    },
    Language {
        code: "fr",
        name: "French",
    },
    Language {
        code: "fy",
        name: "Frisian",
    },
    Language {
        code: "gl",
        name: "Galician",
    },
    Language {
        code: "ka",
        name: "Georgian",
    },
    Language {
        code: "de",
        name: "German",
    },
    Language {
        code: "el",
        name: "Greek",
    },
    Language {
        code: "en",
        name: "English",
    },
    Language {
        code: "gu",
        name: "Gujarati",
    },
    Language {
        code: "ht",
        name: "Haitian Creole",
    },
    Language {
        code: "ha",
        name: "Hawaiian",
    },
    Language {
        code: "iw",
        name: "Hebrew",
    },
    Language {
        code: "hi",
        name: "Hindi",
    },
    Language {
        code: "hm",
        name: "Hmong",
    },
    Language {
        code: "hu",
        name: "Hungarian",
    },
    Language {
        code: "is",
        name: "Icelandic",
    },
    Language {
        code: "ig",
        name: "Igbo",
    },
    Language {
        code: "id",
        name: "Indonesian",
    },
    Language {
        code: "ga",
        name: "Irish",
    },
    Language {
        code: "it",
        name: "Italian",
    },
    Language {
        code: "ja",
        name: "Japanese",
    },
    Language {
        code: "jw",
        name: "Javanese",
    },
    Language {
        code: "kn",
        name: "Kannada",
    },
    Language {
        code: "kk",
        name: "Kazakh",
    },
    Language {
        code: "km",
        name: "Khmer",
    },
    Language {
        code: "rw",
        name: "Kinyarwanda",
    },
    Language {
        code: "ko",
        name: "Korean",
    },
    Language {
        code: "ku",
        name: "Kurdish (Kurmanji)",
    },
    Language {
        code: "ky",
        name: "Kyrgyz",
    },
    Language {
        code: "lo",
        name: "Lao",
    },
    Language {
        code: "la",
        name: "Latin",
    },
    Language {
        code: "lv",
        name: "Latvian",
    },
    Language {
        code: "lt",
        name: "Lithuanian",
    },
    Language {
        code: "lb",
        name: "Luxembourgish",
    },
    Language {
        code: "mk",
        name: "Macedonian",
    },
    Language {
        code: "mg",
        name: "Malagasy",
    },
    Language {
        code: "ms",
        name: "Malay",
    },
    Language {
        code: "ml",
        name: "Malayalam",
    },
    Language {
        code: "mt",
        name: "Maltese",
    },
    Language {
        code: "mi",
        name: "Maori",
    },
    Language {
        code: "mr",
        name: "Marathi",
    },
    Language {
        code: "mn",
        name: "Mongolian",
    },
    Language {
        code: "my",
        name: "Myanmar (Burmese)",
    },
    Language {
        code: "ne",
        name: "Nepali",
    },
    Language {
        code: "nl",
        name: "Dutch",
    },
    Language {
        code: "no",
        name: "Norwegian",
    },
    Language {
        code: "or",
        name: "Odia (Oriya)",
    },
    Language {
        code: "ps",
        name: "Pashto",
    },
    Language {
        code: "fa",
        name: "Persian",
    },
    Language {
        code: "pl",
        name: "Polish",
    },
    Language {
        code: "pt",
        name: "Portuguese",
    },
    Language {
        code: "pa",
        name: "Punjabi",
    },
    Language {
        code: "ro",
        name: "Romanian",
    },
    Language {
        code: "ru",
        name: "Russian",
    },
    Language {
        code: "sm",
        name: "Samoan",
    },
    Language {
        code: "gd",
        name: "Scots Gaelic",
    },
    Language {
        code: "sr",
        name: "Serbian",
    },
    Language {
        code: "st",
        name: "Sesotho",
    },
    Language {
        code: "sn",
        name: "Shona",
    },
    Language {
        code: "sd",
        name: "Sindhi",
    },
    Language {
        code: "si",
        name: "Sinhala",
    },
    Language {
        code: "sk",
        name: "Slovak",
    },
    Language {
        code: "sl",
        name: "Slovenian",
    },
    Language {
        code: "so",
        name: "Somali",
    },
    Language {
        code: "es",
        name: "Spanish",
    },
    Language {
        code: "su",
        name: "Sundanese",
    },
    Language {
        code: "sw",
        name: "Swahili",
    },
    Language {
        code: "sv",
        name: "Swedish",
    },
    Language {
        code: "tg",
        name: "Tajik",
    },
    Language {
        code: "ta",
        name: "Tamil",
    },
    Language {
        code: "tt",
        name: "Tatar",
    },
    Language {
        code: "te",
        name: "Telugu",
    },
    Language {
        code: "th",
        name: "Thai",
    },
    Language {
        code: "tr",
        name: "Turkish",
    },
    Language {
        code: "tk",
        name: "Turkmen",
    },
    Language {
        code: "uk",
        name: "Ukrainian",
    },
    Language {
        code: "ur",
        name: "Urdu",
    },
    Language {
        code: "ug",
        name: "Uyghur",
    },
    Language {
        code: "uz",
        name: "Uzbek",
    },
    Language {
        code: "vi",
        name: "Vietnamese",
    },
    Language {
        code: "cy",
        name: "Welsh",
    },
    Language {
        code: "xh",
        name: "Xhosa",
    },
    Language {
        code: "yi",
        name: "Yiddish",
    },
    Language {
        code: "yo",
        name: "Yoruba",
    },
    Language {
        code: "zu",
        name: "Zulu",
    },
];
