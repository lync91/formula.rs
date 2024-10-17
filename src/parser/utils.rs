pub fn cell_string_to_usize(cell_chars: &str) -> Option<(usize, usize)> {
    let mut col = 0;
    let mut row_str = String::new();

    // Tách ký tự cột (chữ cái) và hàng (số)
    for c in cell_chars.chars() {
        if c.is_ascii_alphabetic() {
            // Chuyển đổi ký tự cột thành số
            col = col * 26 + (c.to_ascii_uppercase() as usize - 'A' as usize);
        } else if c.is_ascii_digit() {
            // Tạo chuỗi cho hàng
            row_str.push(c);
        } else {
            return None; // Nếu có ký tự không hợp lệ
        }
    }

    // Chuyển chuỗi hàng thành số
    if let Ok(row) = row_str.parse::<usize>() {
        Some((row - 1, col))
    } else {
        None
    }
}
