fn mock_rand(n: u8) -> f32 {
    // (n as f32) / 255.0
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

pub fn main() {
    println!("入力範囲の最大値: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("入力範囲の中央値: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("入力範囲の最小値: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
