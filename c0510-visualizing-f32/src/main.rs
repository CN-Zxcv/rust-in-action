// 浮点数编码规则
// | 1 符号位 | 8 指数位 | 23 尾数位 |
// 计算公式: -1 ^ 符号 * 尾数 * 2 ^（ 指数 - 127 ）
// 符号位 0，1
// 指数位 整数编码，为了处理负数，采用 127 偏移
// 尾数位 从右到左 计算权重值，然后将权重值相加 2 ^ -23, 2 ^ -22, .., 2 ^ -1
//   其中还要处理 隐含的 2 ^ 0，
//   指数位全 0 时，尾数是非正规数，非正规数用来表示，0 到 默认编码最小可编码的浮点数之间的数，增加了接近 0 时能表示的数的数量
//   指数位全 1 时，表示 正负无穷大，或者NAN

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field | as bit | as real number");
    println!("sign | {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exp, exp_);
    println!("mantissa | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let sign_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa = 1_f32;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (sign_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
