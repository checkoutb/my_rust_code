
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;


fn main() {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("serving on localhost:3000");
    server.bind("127.0.0.1:3000").expect("error binding to ip port").run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>gcd calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">compute gcd</button>
                </form>
            "#
        )
}

#[derive(Deserialize)]      // 这个注解 要求 serde crate 在程序编译时检查此类型并自动生成代码，以便从 HTML 表单 POST 提交过来的格式化数据中解析出此类型的值
struct GcdParam {
    n: u64,
    m: u64
}

fn post_gcd(form: web::Form<GcdParam>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("gcd with zero");
    }
    let response = format!("the gcd of {} and {} \
        is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if b < a {
            let t = b;
            b = a;
            a = t;
        }
        b = b % a;
    }
    a
}