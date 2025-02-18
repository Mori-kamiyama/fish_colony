use nannou::prelude::*;
use nannou::rand::thread_rng;
use nannou::rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

const FISH_AMOUNT: usize = 100;
const FISH_VEC: f32 = 1.0;
const FISH_SIZE: f32 = 7.0;


const WIDTH:f32 = 400.0;
const HEIGHT:f32= 300.0;

// 魚の個々を管理する構造体
struct Fish {
    pos: Vec2,   // 座標
    vec: f32,    // 速度
    theta: f32,  // 角度
}

// アプリケーションの状態を管理する構造体
struct Model {
    fishes: Vec<Fish>,
}

fn model(app: &App) -> Model {
    let mut rng = thread_rng();

    app.new_window()
        .title("Fishes")
        .size((WIDTH * 2.0) as u32, (HEIGHT * 2.0) as u32)
        .view(view)
        .build()
        .unwrap();

    let mut fishes = Vec::new();
    // FISH_AMOUNT匹の魚をランダムな位置と向きで初期化
    for _ in 0..FISH_AMOUNT {
        fishes.push(Fish {
            pos: Vec2::new(
                rng.gen_range(-WIDTH..WIDTH),
                rng.gen_range(-HEIGHT..HEIGHT),
            ),
            vec: FISH_VEC,
            theta: rng.gen_range(0.0..(PI * 2.0)),
        });
    }

    Model { fishes }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut rng = thread_rng();

    // 各ルールのパラメータ設定
    let coh_min = 20.0;
    let coh_max = 50.0;
    let coh_weight = 0.01;

    let ali_min = 20.0;
    let ali_max = 50.0;
    let ali_weight = 0.02;

    let sep_min = 5.0;
    let sep_max = 20.0;
    let sep_weight = 0.03;

    // 全魚の位置と角度をあらかじめコピー
    let positions: Vec<Vec2> = model.fishes.iter().map(|f| f.pos).collect();
    let thetas: Vec<f32> = model.fishes.iter().map(|f| f.theta).collect();

    for i in 0..FISH_AMOUNT {
        let pos = positions[i];
        let theta = thetas[i];

        let mut coh_sum = vec2(0.0, 0.0);
        let mut ali_sum = vec2(0.0, 0.0);
        let mut sep_sum = vec2(0.0, 0.0);
        let mut c = 0;
        let mut a = 0;
        let mut s = 0;

        if rng.gen_range(0..10) < 7 {
            // 他の魚との距離をもとに各ルールを計算
            for j in 0..FISH_AMOUNT {
                if i == j {
                    continue;
                }
                let other_pos = positions[j];
                let other_theta = thetas[j];
                let dis = pos.distance(other_pos);

                // 【凝集】一定距離内にある魚の重心を求める
                if dis > coh_min && dis < coh_max {
                    coh_sum += other_pos;
                    c += 1;
                }
                // 【整列】一定距離内の魚の進行方向（sin, cos）を平均する
                if dis > ali_min && dis < ali_max {
                    ali_sum += vec2(other_theta.sin(), other_theta.cos());
                    a += 1;
                }
                // 【分離】近すぎる魚から離れる方向を計算する
                if dis > sep_min && dis < sep_max {
                    sep_sum += (pos - other_pos) / dis;
                    s += 1;
                }
            }

            // 現在の向き（sin, cos）を初期値とする
            let mut desired_direction = vec2(theta.sin(), theta.cos());

            // 凝集ルールの影響を加算
            if c > 0 {
                let center = coh_sum / c as f32;
                let cohesion_vector = (center - pos).normalize();
                desired_direction += cohesion_vector * coh_weight;
            }

            // 整列ルールの影響を加算
            if a > 0 {
                let alignment_vector = (ali_sum / a as f32).normalize();
                desired_direction += alignment_vector * ali_weight;
            }

            // 分離ルールの影響を加算（条件を s > 0 に修正）
            if s > 0 {
                let separation_vector = if sep_sum.length() > 0.0 {
                    sep_sum.normalize()
                } else {
                    vec2(0.0, 0.0)
                };
                desired_direction += separation_vector * sep_weight;
            }

            // 描画や移動では sin, cos を使っているので、
            // (x, y) = (sin(theta), cos(theta)) となるような theta を求める
            model.fishes[i].theta = desired_direction.x.atan2(desired_direction.y);
        } else {
            model.fishes[i].theta = theta;
        }


        model.fishes[i].vec *= if rng.gen_range(0..2) == 1 {0.999} else {1.001};

        // 魚の位置を移動（速度は fish.vec で定義）
        model.fishes[i].pos.x += model.fishes[i].vec * model.fishes[i].theta.sin();
        model.fishes[i].pos.y += model.fishes[i].vec * model.fishes[i].theta.cos();

        // 画面外に出た場合のラッピング処理
        if model.fishes[i].pos.y < -(HEIGHT + 10.0) {
            model.fishes[i].pos.y = HEIGHT + 5.0;
        }
        if model.fishes[i].pos.x > (WIDTH + 10.0) {
            model.fishes[i].pos.x = -(WIDTH + 5.0);
        }
        if model.fishes[i].pos.y > (HEIGHT + 10.0) {
            model.fishes[i].pos.y = -(HEIGHT + 5.0);
        }
        if model.fishes[i].pos.x < -(WIDTH + 10.0) {
            model.fishes[i].pos.x = WIDTH + 5.0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let l = FISH_SIZE;

    for (i, fish)  in model.fishes.iter().enumerate() {
        // 魚の向きに合わせた三角形の頂点を計算
        let points = vec![
            pt2(
                fish.pos.x + l * fish.theta.sin(),
                fish.pos.y + l * fish.theta.cos(),
            ),
            pt2(
                fish.pos.x + l * (fish.theta + (PI * 3.0 / 4.0)).sin(),
                fish.pos.y + l * (fish.theta + (PI * 3.0 / 4.0)).cos(),
            ),
            pt2(
                fish.pos.x + (l / 2.0) * (fish.theta + PI).sin(),
                fish.pos.y + (l / 2.0) * (fish.theta + PI).cos(),
            ),
            pt2(
                fish.pos.x + l * (fish.theta - (PI * 3.0 / 4.0)).sin(),
                fish.pos.y + l * (fish.theta - (PI * 3.0 / 4.0)).cos(),
            ),
        ];
        if i == 0 {
            draw.polygon()
                .color(RED)
                .stroke(RED)
                .stroke_weight(7.0)
                .join_round()
                .points(points);

            continue;
        }

        // 魚（多角形）を描画
        draw.polygon()
            .color(BLACK)
            .stroke(BLACK)
            .stroke_weight(7.0)
            .join_round()
            .points(points);
    }

    draw.to_frame(app, &frame).unwrap();
}
