pub fn encode_function(input: &[u8]) -> String
{
    const ENCODING_CHARS: [char; 64] =
    [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];
    let mut conversion_result = String::new();
    for bit_chunk in input.chunks_exact(3)
    {
        let parsed_chunk = (bit_chunk[0] as u32) << 16 | (bit_chunk.get(1).copied().unwrap_or(0) as u32) << 8 | (bit_chunk.get(2).copied().unwrap_or(0) as u32);
    
        let bit_1 = (parsed_chunk >> 18) & 0b111111;
        let bit_2 = (parsed_chunk >> 12) & 0b111111;
        let bit_3 = (parsed_chunk >> 6) & 0b111111;
        let bit_4 = parsed_chunk & 0b111111;

        let bit_map_1 = ENCODING_CHARS[bit_1 as usize];
        let bit_map_2 = ENCODING_CHARS[bit_2 as usize];
        let bit_map_3 = if bit_chunk.len() == 1 { '=' } else { ENCODING_CHARS[bit_3 as usize] };
        let bit_map_4 = if bit_chunk.len() < 3 { '=' } else { ENCODING_CHARS[bit_4 as usize] };

        conversion_result.push(bit_map_1);
        conversion_result.push(bit_map_2);
        conversion_result.push(bit_map_3);
        conversion_result.push(bit_map_4);

    }

    let m_remainder = input.chunks_exact(3).remainder();
    if m_remainder.len() > 0
    {
        let parsed_chunk = (m_remainder[0] as u32) << 16 | (m_remainder.get(1).copied().unwrap_or(0) as u32) << 8 | (m_remainder.get(2).copied().unwrap_or(0) as u32);

        let bit_1 = (parsed_chunk >> 18) & 0b111111;
        let bit_2 = (parsed_chunk >> 12) & 0b111111;
        let bit_3 = (parsed_chunk >> 6) & 0b111111;
        let bit_4 = parsed_chunk & 0b111111;

        let bit_map_1 = ENCODING_CHARS[bit_1 as usize];
        let bit_map_2 = ENCODING_CHARS[bit_2 as usize];
        println!("{}", m_remainder.len());
        let bit_map_3 = if m_remainder.len() == 1 { '=' } else { ENCODING_CHARS[bit_3 as usize] };
        let bit_map_4 = if m_remainder.len() < 3 { '=' } else { ENCODING_CHARS[bit_4 as usize] };

        conversion_result.push(bit_map_1);
        conversion_result.push(bit_map_2);
        conversion_result.push(bit_map_3);
        conversion_result.push(bit_map_4);
    }
    return conversion_result;
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_function()
    {
        assert_eq!(encode_function(b"buna ziua domn profesor cum v a fost ziua"), "YnVuYSB6aXVhIGRvbW4gcHJvZmVzb3IgY3VtIHYgYSBmb3N0IHppdWE=");
        assert_eq!(encode_function(b"mama sita"), "bWFtYSBzaXRh");
        assert_eq!(encode_function(b"10101222345aaaxxZZ"), "MTAxMDEyMjIzNDVhYWF4eFpa");
        assert_eq!(encode_function(b""), "");
    }
}
