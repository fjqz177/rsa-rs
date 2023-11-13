use num_bigint::BigInt; //用于超大整数的计算
use num_traits::{One, Zero}; //用于BigInt的常量0和1
use std::time::Instant; //用于计算程序运行时间

// 扩展欧几里得算法
fn ext_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if *b == Zero::zero() {
        (a.clone(), One::one(), Zero::zero())
    } else {
        let (r, x1, y1) = ext_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - ((a / b) * &y1);
        (r, x, y)
    }
}

// 超大整数超大次幂再对超大的整数取模
fn exp_mode(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    let zero = Zero::zero();
    let one: BigInt = One::one();
    let two = &one + &one;
    let mut result = one.clone();
    let mut base = base % modulus;
    let mut exponent = exponent.clone();

    while exponent > zero {
        if &exponent % &two == one {
            result = result * &base % modulus;
        }
        exponent = exponent >> 1;
        base = &base * &base % modulus;
    }
    result
}

// 计算公钥和私钥
fn gen_key(p: &BigInt, q: &BigInt) -> ((BigInt, BigInt), (BigInt, BigInt)) {
    let n = p * q;
    let fy = (p - 1u32) * (q - 1u32);
    let e: BigInt = 65537u32.into();

    let (_, x, _) = ext_gcd(&e, &fy);

    let d = if x < Zero::zero() { x + &fy } else { x };

    ((n.clone(), e), (n, d))
}

// 加密函数
fn encrypt(m: &BigInt, pubkey: &(BigInt, BigInt)) -> (BigInt, f64) {
    let start_time = Instant::now();

    let c = exp_mode(m, &pubkey.1, &pubkey.0);

    let time_elapsed = start_time.elapsed().as_secs_f64();

    (c, time_elapsed)
}

// 解密函数
fn decrypt(c: &BigInt, selfkey: &(BigInt, BigInt)) -> (BigInt, f64) {
    let start_time = Instant::now();

    let m = exp_mode(c, &selfkey.1, &selfkey.0);

    let time_elapsed = start_time.elapsed().as_secs_f64();

    (m, time_elapsed)
}

// 判断字符串是否为纯数字
fn is_str_numeric(s: &str) -> bool {
    let re = regex::Regex::new(r"^\d+$").unwrap();
    re.is_match(s)
}

fn main() {
    // 此处应使用实际的用户输入，注意需要处理错误和异常的输入
    let p = "106697219132480173106064317148705638676529121742557567770857687729397446898790451577487723991083173010242416863238099716044775658681981821407922722052778958942891831033512463262741053961681512908218003840408526915629689432111480588966800949428079015682624591636010678691927285321708935076221951173426894836169".parse::<BigInt>().unwrap();
    let q = "144819424465842307806353672547344125290716753535239658417883828941232509622838692761917211806963011168822281666033695157426515864265527046213326145174398018859056439431422867957079149967592078894410082695714160599647180947207504108618794637872261572262805565517756922288320779308895819726074229154002310375209".parse::<BigInt>().unwrap();

    // 生成公钥和私钥
    let (pubkey, selfkey) = gen_key(&p, &q);

    loop {
        // 此处应使用实际的用户输入，注意需要处理错误和异常的输入
        println!("请输入要加密或解密的数字:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // 判断输入是否为纯数字
        if is_str_numeric(&input.trim()) { // 输入是纯数字
            // 将输入转换为BigInt类型
            let message = input.trim().parse::<BigInt>().unwrap();
            println!("请输入要执行的操作(1为加密,2为解密,3为退出):");
            let mut op = String::new();
            std::io::stdin().read_line(&mut op).unwrap();
            let op = op.trim().parse::<u32>().unwrap();

            // 根据用户输入执行相应操作
            match op {
                1 => {
                    println!("待加密数字: {}", message);
                    let (c, d) = encrypt(&message, &pubkey);
                    println!("加密结果: {}", c);
                    println!("加密耗时: {:?}秒", d);
                    ()
                }
                2 => {
                    let (m, d) = decrypt(&message, &selfkey);
                    println!("解密结果: {}", m);
                    println!("解密耗时: {:?}秒", d);
                    ()
                }
                3 => {
                    println!("退出程序");
                    break;
                }
                _ => {
                    println!("无效操作");
                    continue;
                }
            };
        } else { // 输入不是纯数字
            println!("密码必须是纯数字！");
            continue;
        }
    }
}
