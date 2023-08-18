use image::{ImageBuffer, Rgb, RgbImage};
use irospace::{converter::*, ColorConverterBuilder, HsvColor, RgbColor};
use itertools::Itertools;
use num::{complex::Complex, integer::div_rem};

//画像のサイズを指定 (1000)
static SIZE: u32 = 1000;
//画像サイズに合わせる複素数の範囲を指定
static SPAN: f64 = 0.00002;
//画像中心点の複素数の実数値を設定
static RE_CENTER: f64 = -0.743643135;
//画像中心点の複素数の虚数値を設定
static IM_CENTER: f64 = 0.131825963;

//画像サイズに合わせる複素数の左上の複素数をセット
static CPOINT1: Complex<f64> = Complex::new(RE_CENTER - (SPAN / 2.0), IM_CENTER - (SPAN / 2.0));
//画像サイズに合わせる複素数の右下の複素数をセット
static CPOINT2: Complex<f64> = Complex::new(RE_CENTER + (SPAN / 2.0), IM_CENTER + (SPAN / 2.0));

// 隣のピクセルとの複素数の差（X軸、Y軸）
static DX: Complex<f64> = Complex::new((CPOINT2.re - CPOINT1.re) / SIZE as f64, 0.0);
static DY: Complex<f64> = Complex::new(0.0, (CPOINT2.re - CPOINT1.re) / SIZE as f64);

fn main() {
    // 1000x1000の画像データを作成
    let prot_data: Vec<u32> = plot(SIZE);

    // PNG画像を作成
    let mut img: ImageBuffer<image::Rgb<u8>, Vec<u8>> = RgbImage::new(SIZE, SIZE);
    // HSVからRGBへの変換器を作成
    let converter = ColorConverterBuilder::new().from_hsv().to_rgb().build();

    // 1000x1000の x,y座標の組み合わせを作成
    let grid = (0..SIZE).cartesian_product(0..SIZE);
    // x,y座標に色を割り当てる
    grid.for_each(|x| {
        // 色を取得
        let color = prot_data[(x.0 * SIZE as u32 + x.1) as usize];
        // HSVのhに色を割り当てる
        let color_code = HsvColor::new(color as u16, 255, 255);
        // HSVからRGBへ変換
        let rgb: RgbColor = converter.convert(&color_code).unwrap();
        // 画像に色を割り当てる
        img.put_pixel(x.0, x.1, Rgb([rgb.r(), rgb.g(), rgb.b()]));
    });

    // 画像を保存
    img.save("out_image.png").unwrap();
}

// プロットするデータを作成
fn plot(size: u32) -> Vec<u32> {
    // 複素数のベクトルを作成
    let cvec: Vec<Complex<f64>> = (0..(size * size))
        .map(|c| {
            let (q, r) = div_rem(c, SIZE);
            CPOINT1 + DX.scale(q as f64) + DY.scale(r as f64)
        })
        .collect();
    // 複素数の0を作成
    let complex_num0: num::Complex<f64> = Complex::new(0.0, 0.0);
    // マンデルブロ集合を計算
    cvec.into_iter()
        .map(|c: Complex<f64>| mandelbrpt(&c, complex_num0, 1000))
        .collect()
}
// マンデルブロ集合を計算(漸化式)
fn mandelbrpt(c: &Complex<f64>, z: Complex<f64>, n: u32) -> u32 {
    let z1 = z * z + *c;
    match (c, 2.0, z1, n) {
        //1000回で発散しなかった場合は 0 を返す
        (_, _, _, 0) => 0,
        //発散した場合はその回数を360で割って余りをを返す
        (_, _, z1, _) if z1.norm() > 2.0 => n % 360,
        //それ以外は回数を一つ減らして再帰呼出し
        (..) => mandelbrpt(c, z1, n - 1),
    }
}
