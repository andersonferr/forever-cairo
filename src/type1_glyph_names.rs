use ::libc;
pub type __int16_t = libc::c_short;
pub type int16_t = __int16_t;
static mut glyph_name_symbol: [libc::c_char; 1590] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1590],
        &[libc::c_char; 1590],
    >(
        b"\0space\0exclam\0quotedbl\0numbersign\0dollar\0percent\0ampersand\0quoteright\0parenleft\0parenright\0asterisk\0plus\0comma\0hyphen\0period\0slash\0zero\0one\0two\0three\0four\0five\0six\0seven\0eight\0nine\0colon\0semicolon\0less\0equal\0greater\0question\0at\0A\0B\0C\0D\0E\0F\0G\0H\0I\0J\0K\0L\0M\0N\0O\0P\0Q\0R\0S\0T\0U\0V\0W\0X\0Y\0Z\0bracketleft\0backslash\0bracketright\0asciicircum\0underscore\0quoteleft\0a\0b\0c\0d\0e\0f\0g\0h\0i\0j\0k\0l\0m\0n\0o\0p\0q\0r\0s\0t\0u\0v\0w\0x\0y\0z\0braceleft\0bar\0braceright\0asciitilde\0exclamdown\0cent\0sterling\0fraction\0yen\0florin\0section\0currency\0quotesingle\0quotedblleft\0guillemotleft\0guilsinglleft\0guilsinglright\0fi\0fl\0endash\0dagger\0daggerdbl\0periodcentered\0paragraph\0bullet\0quotesinglbase\0quotedblbase\0quotedblright\0guillemotright\0ellipsis\0perthousand\0questiondown\0grave\0acute\0circumflex\0tilde\0macron\0breve\0dotaccent\0dieresis\0ring\0cedilla\0hungarumlaut\0ogonek\0caron\0emdash\0AE\0ordfeminine\0Lslash\0Oslash\0OE\0ordmasculine\0ae\0dotlessi\0lslash\0oslash\0oe\0germandbls\0Euro\0Scaron\0Zcaron\0trademark\0scaron\0zcaron\0Ydieresis\0brokenbar\0copyright\0logicalnot\0registered\0degree\0plusminus\0twosuperior\0threesuperior\0mu\0onesuperior\0onequarter\0onehalf\0threequarters\0Agrave\0Aacute\0Acircumflex\0Atilde\0Adieresis\0Aring\0Ccedilla\0Egrave\0Eacute\0Ecircumflex\0Edieresis\0Igrave\0Iacute\0Icircumflex\0Idieresis\0Eth\0Ntilde\0Ograve\0Oacute\0Ocircumflex\0Otilde\0Odieresis\0multiply\0Ugrave\0Uacute\0Ucircumflex\0Udieresis\0Yacute\0Thorn\0agrave\0aacute\0acircumflex\0atilde\0adieresis\0aring\0ccedilla\0egrave\0eacute\0ecircumflex\0edieresis\0igrave\0iacute\0icircumflex\0idieresis\0eth\0ntilde\0ograve\0oacute\0ocircumflex\0otilde\0odieresis\0divide\0ugrave\0uacute\0ucircumflex\0udieresis\0yacute\0thorn\0ydieresis\0\0",
    )
};
static mut ps_standard_encoding_offset: [int16_t; 256] = [
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    1 as libc::c_int as int16_t,
    7 as libc::c_int as int16_t,
    14 as libc::c_int as int16_t,
    23 as libc::c_int as int16_t,
    34 as libc::c_int as int16_t,
    41 as libc::c_int as int16_t,
    49 as libc::c_int as int16_t,
    59 as libc::c_int as int16_t,
    70 as libc::c_int as int16_t,
    80 as libc::c_int as int16_t,
    91 as libc::c_int as int16_t,
    100 as libc::c_int as int16_t,
    105 as libc::c_int as int16_t,
    111 as libc::c_int as int16_t,
    118 as libc::c_int as int16_t,
    125 as libc::c_int as int16_t,
    131 as libc::c_int as int16_t,
    136 as libc::c_int as int16_t,
    140 as libc::c_int as int16_t,
    144 as libc::c_int as int16_t,
    150 as libc::c_int as int16_t,
    155 as libc::c_int as int16_t,
    160 as libc::c_int as int16_t,
    164 as libc::c_int as int16_t,
    170 as libc::c_int as int16_t,
    176 as libc::c_int as int16_t,
    181 as libc::c_int as int16_t,
    187 as libc::c_int as int16_t,
    197 as libc::c_int as int16_t,
    202 as libc::c_int as int16_t,
    208 as libc::c_int as int16_t,
    216 as libc::c_int as int16_t,
    225 as libc::c_int as int16_t,
    228 as libc::c_int as int16_t,
    230 as libc::c_int as int16_t,
    232 as libc::c_int as int16_t,
    234 as libc::c_int as int16_t,
    236 as libc::c_int as int16_t,
    238 as libc::c_int as int16_t,
    240 as libc::c_int as int16_t,
    242 as libc::c_int as int16_t,
    244 as libc::c_int as int16_t,
    246 as libc::c_int as int16_t,
    248 as libc::c_int as int16_t,
    250 as libc::c_int as int16_t,
    252 as libc::c_int as int16_t,
    254 as libc::c_int as int16_t,
    256 as libc::c_int as int16_t,
    258 as libc::c_int as int16_t,
    260 as libc::c_int as int16_t,
    262 as libc::c_int as int16_t,
    264 as libc::c_int as int16_t,
    266 as libc::c_int as int16_t,
    268 as libc::c_int as int16_t,
    270 as libc::c_int as int16_t,
    272 as libc::c_int as int16_t,
    274 as libc::c_int as int16_t,
    276 as libc::c_int as int16_t,
    278 as libc::c_int as int16_t,
    280 as libc::c_int as int16_t,
    292 as libc::c_int as int16_t,
    302 as libc::c_int as int16_t,
    315 as libc::c_int as int16_t,
    327 as libc::c_int as int16_t,
    338 as libc::c_int as int16_t,
    348 as libc::c_int as int16_t,
    350 as libc::c_int as int16_t,
    352 as libc::c_int as int16_t,
    354 as libc::c_int as int16_t,
    356 as libc::c_int as int16_t,
    358 as libc::c_int as int16_t,
    360 as libc::c_int as int16_t,
    362 as libc::c_int as int16_t,
    364 as libc::c_int as int16_t,
    366 as libc::c_int as int16_t,
    368 as libc::c_int as int16_t,
    370 as libc::c_int as int16_t,
    372 as libc::c_int as int16_t,
    374 as libc::c_int as int16_t,
    376 as libc::c_int as int16_t,
    378 as libc::c_int as int16_t,
    380 as libc::c_int as int16_t,
    382 as libc::c_int as int16_t,
    384 as libc::c_int as int16_t,
    386 as libc::c_int as int16_t,
    388 as libc::c_int as int16_t,
    390 as libc::c_int as int16_t,
    392 as libc::c_int as int16_t,
    394 as libc::c_int as int16_t,
    396 as libc::c_int as int16_t,
    398 as libc::c_int as int16_t,
    400 as libc::c_int as int16_t,
    410 as libc::c_int as int16_t,
    414 as libc::c_int as int16_t,
    425 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    436 as libc::c_int as int16_t,
    447 as libc::c_int as int16_t,
    452 as libc::c_int as int16_t,
    461 as libc::c_int as int16_t,
    470 as libc::c_int as int16_t,
    474 as libc::c_int as int16_t,
    481 as libc::c_int as int16_t,
    489 as libc::c_int as int16_t,
    498 as libc::c_int as int16_t,
    510 as libc::c_int as int16_t,
    523 as libc::c_int as int16_t,
    537 as libc::c_int as int16_t,
    551 as libc::c_int as int16_t,
    566 as libc::c_int as int16_t,
    569 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    572 as libc::c_int as int16_t,
    579 as libc::c_int as int16_t,
    586 as libc::c_int as int16_t,
    596 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    611 as libc::c_int as int16_t,
    621 as libc::c_int as int16_t,
    628 as libc::c_int as int16_t,
    643 as libc::c_int as int16_t,
    656 as libc::c_int as int16_t,
    670 as libc::c_int as int16_t,
    685 as libc::c_int as int16_t,
    694 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    706 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    719 as libc::c_int as int16_t,
    725 as libc::c_int as int16_t,
    731 as libc::c_int as int16_t,
    742 as libc::c_int as int16_t,
    748 as libc::c_int as int16_t,
    755 as libc::c_int as int16_t,
    761 as libc::c_int as int16_t,
    771 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    780 as libc::c_int as int16_t,
    785 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    793 as libc::c_int as int16_t,
    806 as libc::c_int as int16_t,
    813 as libc::c_int as int16_t,
    819 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    826 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    829 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    841 as libc::c_int as int16_t,
    848 as libc::c_int as int16_t,
    855 as libc::c_int as int16_t,
    858 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    871 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    874 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    883 as libc::c_int as int16_t,
    890 as libc::c_int as int16_t,
    897 as libc::c_int as int16_t,
    900 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
];
static mut winansi_encoding_offset: [int16_t; 256] = [
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    1 as libc::c_int as int16_t,
    7 as libc::c_int as int16_t,
    14 as libc::c_int as int16_t,
    23 as libc::c_int as int16_t,
    34 as libc::c_int as int16_t,
    41 as libc::c_int as int16_t,
    49 as libc::c_int as int16_t,
    498 as libc::c_int as int16_t,
    70 as libc::c_int as int16_t,
    80 as libc::c_int as int16_t,
    91 as libc::c_int as int16_t,
    100 as libc::c_int as int16_t,
    105 as libc::c_int as int16_t,
    111 as libc::c_int as int16_t,
    118 as libc::c_int as int16_t,
    125 as libc::c_int as int16_t,
    131 as libc::c_int as int16_t,
    136 as libc::c_int as int16_t,
    140 as libc::c_int as int16_t,
    144 as libc::c_int as int16_t,
    150 as libc::c_int as int16_t,
    155 as libc::c_int as int16_t,
    160 as libc::c_int as int16_t,
    164 as libc::c_int as int16_t,
    170 as libc::c_int as int16_t,
    176 as libc::c_int as int16_t,
    181 as libc::c_int as int16_t,
    187 as libc::c_int as int16_t,
    197 as libc::c_int as int16_t,
    202 as libc::c_int as int16_t,
    208 as libc::c_int as int16_t,
    216 as libc::c_int as int16_t,
    225 as libc::c_int as int16_t,
    228 as libc::c_int as int16_t,
    230 as libc::c_int as int16_t,
    232 as libc::c_int as int16_t,
    234 as libc::c_int as int16_t,
    236 as libc::c_int as int16_t,
    238 as libc::c_int as int16_t,
    240 as libc::c_int as int16_t,
    242 as libc::c_int as int16_t,
    244 as libc::c_int as int16_t,
    246 as libc::c_int as int16_t,
    248 as libc::c_int as int16_t,
    250 as libc::c_int as int16_t,
    252 as libc::c_int as int16_t,
    254 as libc::c_int as int16_t,
    256 as libc::c_int as int16_t,
    258 as libc::c_int as int16_t,
    260 as libc::c_int as int16_t,
    262 as libc::c_int as int16_t,
    264 as libc::c_int as int16_t,
    266 as libc::c_int as int16_t,
    268 as libc::c_int as int16_t,
    270 as libc::c_int as int16_t,
    272 as libc::c_int as int16_t,
    274 as libc::c_int as int16_t,
    276 as libc::c_int as int16_t,
    278 as libc::c_int as int16_t,
    280 as libc::c_int as int16_t,
    292 as libc::c_int as int16_t,
    302 as libc::c_int as int16_t,
    315 as libc::c_int as int16_t,
    327 as libc::c_int as int16_t,
    719 as libc::c_int as int16_t,
    348 as libc::c_int as int16_t,
    350 as libc::c_int as int16_t,
    352 as libc::c_int as int16_t,
    354 as libc::c_int as int16_t,
    356 as libc::c_int as int16_t,
    358 as libc::c_int as int16_t,
    360 as libc::c_int as int16_t,
    362 as libc::c_int as int16_t,
    364 as libc::c_int as int16_t,
    366 as libc::c_int as int16_t,
    368 as libc::c_int as int16_t,
    370 as libc::c_int as int16_t,
    372 as libc::c_int as int16_t,
    374 as libc::c_int as int16_t,
    376 as libc::c_int as int16_t,
    378 as libc::c_int as int16_t,
    380 as libc::c_int as int16_t,
    382 as libc::c_int as int16_t,
    384 as libc::c_int as int16_t,
    386 as libc::c_int as int16_t,
    388 as libc::c_int as int16_t,
    390 as libc::c_int as int16_t,
    392 as libc::c_int as int16_t,
    394 as libc::c_int as int16_t,
    396 as libc::c_int as int16_t,
    398 as libc::c_int as int16_t,
    400 as libc::c_int as int16_t,
    410 as libc::c_int as int16_t,
    414 as libc::c_int as int16_t,
    425 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    911 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    628 as libc::c_int as int16_t,
    474 as libc::c_int as int16_t,
    643 as libc::c_int as int16_t,
    685 as libc::c_int as int16_t,
    579 as libc::c_int as int16_t,
    586 as libc::c_int as int16_t,
    731 as libc::c_int as int16_t,
    694 as libc::c_int as int16_t,
    916 as libc::c_int as int16_t,
    537 as libc::c_int as int16_t,
    855 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    923 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    338 as libc::c_int as int16_t,
    59 as libc::c_int as int16_t,
    510 as libc::c_int as int16_t,
    656 as libc::c_int as int16_t,
    621 as libc::c_int as int16_t,
    572 as libc::c_int as int16_t,
    819 as libc::c_int as int16_t,
    742 as libc::c_int as int16_t,
    930 as libc::c_int as int16_t,
    940 as libc::c_int as int16_t,
    551 as libc::c_int as int16_t,
    897 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    947 as libc::c_int as int16_t,
    954 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    436 as libc::c_int as int16_t,
    447 as libc::c_int as int16_t,
    452 as libc::c_int as int16_t,
    489 as libc::c_int as int16_t,
    470 as libc::c_int as int16_t,
    964 as libc::c_int as int16_t,
    481 as libc::c_int as int16_t,
    771 as libc::c_int as int16_t,
    974 as libc::c_int as int16_t,
    829 as libc::c_int as int16_t,
    523 as libc::c_int as int16_t,
    984 as libc::c_int as int16_t,
    0 as libc::c_int as int16_t,
    995 as libc::c_int as int16_t,
    748 as libc::c_int as int16_t,
    1006 as libc::c_int as int16_t,
    1013 as libc::c_int as int16_t,
    1023 as libc::c_int as int16_t,
    1035 as libc::c_int as int16_t,
    725 as libc::c_int as int16_t,
    1049 as libc::c_int as int16_t,
    611 as libc::c_int as int16_t,
    596 as libc::c_int as int16_t,
    785 as libc::c_int as int16_t,
    1052 as libc::c_int as int16_t,
    858 as libc::c_int as int16_t,
    670 as libc::c_int as int16_t,
    1064 as libc::c_int as int16_t,
    1075 as libc::c_int as int16_t,
    1083 as libc::c_int as int16_t,
    706 as libc::c_int as int16_t,
    1097 as libc::c_int as int16_t,
    1104 as libc::c_int as int16_t,
    1111 as libc::c_int as int16_t,
    1123 as libc::c_int as int16_t,
    1130 as libc::c_int as int16_t,
    1140 as libc::c_int as int16_t,
    826 as libc::c_int as int16_t,
    1146 as libc::c_int as int16_t,
    1155 as libc::c_int as int16_t,
    1162 as libc::c_int as int16_t,
    1169 as libc::c_int as int16_t,
    1181 as libc::c_int as int16_t,
    1191 as libc::c_int as int16_t,
    1198 as libc::c_int as int16_t,
    1205 as libc::c_int as int16_t,
    1217 as libc::c_int as int16_t,
    1227 as libc::c_int as int16_t,
    1231 as libc::c_int as int16_t,
    1238 as libc::c_int as int16_t,
    1245 as libc::c_int as int16_t,
    1252 as libc::c_int as int16_t,
    1264 as libc::c_int as int16_t,
    1271 as libc::c_int as int16_t,
    1281 as libc::c_int as int16_t,
    848 as libc::c_int as int16_t,
    1290 as libc::c_int as int16_t,
    1297 as libc::c_int as int16_t,
    1304 as libc::c_int as int16_t,
    1316 as libc::c_int as int16_t,
    1326 as libc::c_int as int16_t,
    1333 as libc::c_int as int16_t,
    900 as libc::c_int as int16_t,
    1339 as libc::c_int as int16_t,
    1346 as libc::c_int as int16_t,
    1353 as libc::c_int as int16_t,
    1365 as libc::c_int as int16_t,
    1372 as libc::c_int as int16_t,
    1382 as libc::c_int as int16_t,
    871 as libc::c_int as int16_t,
    1388 as libc::c_int as int16_t,
    1397 as libc::c_int as int16_t,
    1404 as libc::c_int as int16_t,
    1411 as libc::c_int as int16_t,
    1423 as libc::c_int as int16_t,
    1433 as libc::c_int as int16_t,
    1440 as libc::c_int as int16_t,
    1447 as libc::c_int as int16_t,
    1459 as libc::c_int as int16_t,
    1469 as libc::c_int as int16_t,
    1473 as libc::c_int as int16_t,
    1480 as libc::c_int as int16_t,
    1487 as libc::c_int as int16_t,
    1494 as libc::c_int as int16_t,
    1506 as libc::c_int as int16_t,
    1513 as libc::c_int as int16_t,
    1523 as libc::c_int as int16_t,
    890 as libc::c_int as int16_t,
    1530 as libc::c_int as int16_t,
    1537 as libc::c_int as int16_t,
    1544 as libc::c_int as int16_t,
    1556 as libc::c_int as int16_t,
    1566 as libc::c_int as int16_t,
    1573 as libc::c_int as int16_t,
    1579 as libc::c_int as int16_t,
];
#[no_mangle]
pub unsafe extern "C" fn _cairo_ps_standard_encoding_to_glyphname(
    mut glyph: libc::c_int,
) -> *const libc::c_char {
    if ps_standard_encoding_offset[glyph as usize] != 0 {
        return glyph_name_symbol
            .as_ptr()
            .offset(ps_standard_encoding_offset[glyph as usize] as libc::c_int as isize)
    } else {
        return 0 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn _cairo_winansi_to_glyphname(
    mut glyph: libc::c_int,
) -> *const libc::c_char {
    if winansi_encoding_offset[glyph as usize] != 0 {
        return glyph_name_symbol
            .as_ptr()
            .offset(winansi_encoding_offset[glyph as usize] as libc::c_int as isize)
    } else {
        return 0 as *const libc::c_char
    };
}