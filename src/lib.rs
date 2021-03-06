const CP866_NON_LATIN: &[char] = &[
    'А','Б','В','Г','Д','Е','Ж','З','И','Й','К','Л','М','Н','О','П',
    'Р','С','Т','У','Ф','Х','Ц','Ч','Ш','Щ','Ъ','Ы','Ь','Э','Ю','Я',
    'а','б','в','г','д','е','ж','з','и','й','к','л','м','н','о','п',
    '░','▒','▓','│','┤','╡','╢','╖','╕','╣','║','╗','╝','╜','╛','┐',
    '└','┴','┬','├','─','┼','╞','╟','╚','╔','╩','╦','╠','═','╬','╧',
    '╨','╤','╥','╙','╘','╒','╓','╫','╪','┘','┌','█','▄','▌','▐','▀',
    'р','с','т','у','ф','х','ц','ч','ш','щ','ъ','ы','ь','э','ю','я',
    'Ё','ё','Є','є','Ї','ї','Ў','ў','°','∙','·','√','№','¤','■',' '
];

pub fn decode_byte(char_index: u8) -> char {
    if char_index < 0x80 {
        char::from(char_index)
    } else {
        CP866_NON_LATIN[(char_index - 0x80) as usize]
    }
}

pub fn decode_bytes(bytes: &[u8]) -> String {
    let mut strlen = 0;
    loop {
        if strlen == bytes.len() {
            break;
        }
        if bytes[strlen] == 0 {
            break;
        }
        strlen += 1;
    }
    let mut result = String::with_capacity(strlen);
    for i in 0..strlen {
        result.push(decode_byte(bytes[i]))
    }
    result
}