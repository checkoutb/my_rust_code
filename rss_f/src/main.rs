
use axum::{
    routing::{get//, post
    },
    // http::StatusCode,
    Json, Router,
    extract::{Query, Path},
    response::Html,
};
use serde::{de, Deserialize, 
    // Serialize, 
    Deserializer};
use log::{info, warn, error};
use std::{fmt, str::FromStr};

use maud::{html, PreEscaped};

use std::fs::File;
// use std::io::Write;

const IP_PORT: &'static str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    // env_logger::init();

    let log_path = "/var/log/rss/rss_f.log";
    let log_file = File::create(log_path).unwrap();
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env().target(env_logger::fmt::Target::Pipe(Box::new(log_file))).init();

    warn!("not vvarn...");
    error!("not 3rr0r...");

    log::logger().flush();
    // log_file.sync_all().unwrap();

    let app = Router::new()
        .route("/", get(get_outline))
        .route("/rss_item/:source", get(get_rss_item).put(put_rss_item).delete(delete_rss_item))
        .route("/rss_data/:source", get(get_rss_data));
        // .route("/users", post(create_user));
    
    let listener = tokio::net::TcpListener::bind(IP_PORT).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

const HTMLJS: &'static str = "<!DOCTYPE html><html><head><script>
function change_status(id, st)
{fetch('/rss_item/'+id, {method: 'PUT',headers: {'Content-Type': 'application/json'},body: JSON.stringify({'status':st})})}
function delete_item(id)
{fetch('/rss_item/'+id, {method: 'DELETE'})}
</script></head><body>";

const HTMLTAIL: &'static str = "</body></html>";
// function makeRequest() {
//     fetch('/your-endpoint', {
//         method: 'GET', // 或者 'POST', 取决于你需要的HTTP方法
//         // 如果需要发送数据，可以添加 body 参数
//         // headers: {
//         //     'Content-Type': 'application/json'
//         // },
//         // body: JSON.stringify({ key: 'value' })
//     })
//     .then(response => response.json()) // 如果期望JSON响应
//     .then(data => console.log(data)) // 处理响应数据
//     .catch(error => console.error('Error:', error)); // 错误处理
// }
// <button onclick="makeRequest()">发送请求</button>

#[derive(Debug, Deserialize)]
struct BodyJson {
    status: i32,
}

async fn put_rss_item(Path(path): Path<String>, json_data: Json<BodyJson>) {
    use crate::rss_mysql::*;
    info!("change status, {}, {:?}", path, json_data);
    set_rss_item_status(path, json_data.status);
}

async fn delete_rss_item(Path(path): Path<String>) {
    use crate::rss_mysql::*;
    // info!("{:?}", json_data);
    info!("delete: {}", &path);
    set_rss_item_status(path, -1);
}

async fn get_outline() -> Html<String> {
    use crate::rss_mysql::*;
    info!("start to generate outline");

    let vsi = get_all_source_item().unwrap();
    let webpage = html! {
        @let t2 = PreEscaped(HTMLJS);
        {(t2)}
        @for si in &vsi {
            @let sname = &si.source.name;
            @let items = &si.item;
            @let shref = PreEscaped("<a href=\"".to_string() + "/rss_item/" + &si.source.id.to_string() + "\" target=\"_blank\">" + sname + "</a>");
            p {(shref)}
            ol {
                @for it in items {
                    @let iname = &it.title;
                    @let ahref = PreEscaped(it.status.to_string() + " " + &it.pubdate + " <a href=\"" + &it.link + "\" target=\"_blank\">" + "原文" + "</a> <button onclick=\"delete_item('" + &it.id.to_string() + "')\">删除</button> <button onclick=\"change_status('" + &it.id.to_string() + "',1)\">已阅</button> <button onclick=\"change_status('" + &it.id.to_string() + "',2)\">星标</button>" + " <a href=\"" + "/rss_data/" + &it.id.to_string() + "\" target=\"_blank\">" + iname + "</a>");
                    li {(ahref)}
                }
            }
            hr;
            br;
        }
        @let t3 = PreEscaped(HTMLTAIL);
        {(t3)}
    };
    let htmlstr = webpage.into_string();
    Html(htmlstr)
}

async fn get_rss_item(Path(path): Path<String>, Query(params): Query<Params>) -> Html<String> {
    use crate::rss_mysql::*;
    info!("start to generate rss_item, {:?}, {:?}", path, params);

    let page = params.page.unwrap_or_default();
    
    let item_per_page = 20;

    let limit_st = page * item_per_page;
    let cnt = item_per_page;
    let items = get_item(path.clone(), limit_st, cnt).unwrap();

    // id, title, link, pubdate, status
    let webpage = html! {
        @let t2 = PreEscaped(HTMLJS);
        {(t2)}
        ol {
            @for it in &items {
                @let ahref = PreEscaped(it.status.to_string() + " " + &it.pubdate + " <a href=\"" + &it.link + "\" target=\"_blank\">" + "原文" + "</a> <button onclick=\"delete_item('" + &it.id.to_string() + "')\">删除</button> <button onclick=\"change_status('" + &it.id.to_string() + "',1)\">已阅</button> <button onclick=\"change_status('" + &it.id.to_string() + "',2)\">星标</button>" + " <a href=\"" + "/rss_data/" + &it.id.to_string() + "\" target=\"_blank\">" + &it.title + "</a>");
                li {(ahref)}
            }
        }

        @let nhref = PreEscaped("&emsp;&emsp;&emsp;&emsp;&emsp;&emsp;&emsp;&emsp;&emsp;<a href=\"/rss_item/".to_string() + &path + "?page=" + &(page + 1).to_string() + "\">下一页</a>");

        hr;
        br;
        {(nhref)}

        @let t3 = PreEscaped(HTMLTAIL);
        {(t3)}
    };
    Html(webpage.into_string())
}

async fn get_rss_data(Path(path): Path<String>, Query(params): Query<Params>) -> Html<String> {
    use crate::rss_mongodb::get_rss_data;
    info!("start to generate rss_data, {:?}, {:?}", path, params);

    let desc = get_rss_data(path).await;
    Html(desc.unwrap())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    // foo: Option<i32>,
    // bar: Option<String>,
    page: Option<i32>,
    source: Option<i32>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

// async fn root() -> &'static str {
//     "hello, world!"
// }
// async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
//     let user = User {
//         id: 1333,
//         username: payload.username
//     };
//     (StatusCode::CREATED, Json(user))
// }
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String
// }
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String
// }


/////////////// mysql ///////////////
pub mod rss_mysql {
    use mysql::*;
    use mysql::prelude::*;
    use mysql::params::Params;

    #[derive(Debug)]
    pub struct SourceItem { // outline's element
        pub source: RssSource,
        pub item: Vec<RssItem>,
    }

    #[derive(Debug)]
    pub struct RssSource {
        pub id: i32,
        pub name: String,
    }

    #[derive(Debug)]
    pub struct RssItem {
        pub id: i32,
        pub title: String,
        pub link: String,
        pub pubdate: String,
        pub status: i32,
    }

    const URL: &'static str = "mysql://root:123456@localhost:3306/rss";

    pub fn get_all_source_item() -> Result<Vec<SourceItem>> {
        let pool = Pool::new(URL)?;

        let mut conn = pool.get_conn()?;
        let vsi = conn.query_map(
            "select id,name_cn from rss.rss_source where active=1 order by priority desc limit 3",
            |(id, name_cn)| {
                let rs = RssSource { id, name: name_cn };
                let mut conn2 = pool.get_conn().unwrap();
                let vitem = conn2.query_map("select id,title,link,pubdate,status from rss.rss_item where sourceid=".to_owned() + &id.to_string() + " and status>=0 order by id desc limit 5",
                    |(id, title, link, pubdate, status)| {
                        RssItem {id, title, link, pubdate, status}
                    }
                );
                SourceItem { source: rs, item: vitem.unwrap() }
            },
        );

        vsi
    }

    pub fn get_item(sourceid: String, limit_st: i32, count: i32) -> Result<Vec<RssItem>> {
        let pool = Pool::new(URL)?;
        let mut conn = pool.get_conn()?;
        let vri = conn.query_map(
            "select id,title,link,pubdate,status from rss.rss_item where status>=0 and sourceid=".to_string() + &sourceid + " order by id desc limit " + &limit_st.to_string() + "," + &count.to_string(),
            |(id, title, link, pubdate, status)| {
                RssItem{id, title, link, pubdate, status}
            },
        );

        vri
    }

    pub fn set_rss_item_status(id: String, status: i32) {
        let pool = Pool::new(URL).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let upsql = "update rss.rss_item set status=".to_string() + &status.to_string() + " where id=" + &id;
        // println!("{}", upsql);
        conn.exec_drop(upsql, Params::Empty).unwrap();
    }
}


/////////////// mongodb ///////////////
pub mod rss_mongodb {
    // use mongodb::{
    //     bson::doc,
    //     sync::Client,
    // };
    use serde::{Deserialize, Serialize};
    use std::error::Error;
    // use log::{info};
    use mongodb::{Client, 
        options::ClientOptions, 
        // options::FindOptions,
    };
    use mongodb::bson::{doc, 
        // Document,
    };
    use futures::stream::TryStreamExt;

    #[derive(Debug, Serialize, Deserialize)]
    struct RssMongo {
        pub itemid: i32,
        pub title: String,
        pub link: String,
        pub description: String,
    }

    // sync..
    // pub fn get_rss_data(itemid: String) -> Result<String, Box<dyn Error>> {
    //     let client = Client::with_uri_str("mongodb://localhost:27017")?;
    //     let db = client.database("rss");
    //     let coll = db.collection::<RssMongo>("rssitem");
    //     let cursor = coll.find(doc! {"itemid": itemid.parse::<i32>().unwrap()}, None)?;
    //     for result in cursor {
    //         println!("{:?}", result);
    //     }
    //     Ok("123".to_string())
    // }

    pub async fn get_rss_data(itemid: String) -> Result<String, Box<dyn Error>> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        // for db_name in client.list_database_names(None, None).await? {
        //     println!("{}", db_name);
        // }

        let db = client.database("rss");
        // for coll_name in db.list_collection_names(None).await? {
        //     println!("{}", coll_name);
        // }

        let coll = db.collection::<RssMongo>("rssitem");
        let filter = doc! {"itemid": itemid.parse::<i32>().unwrap()};
        // let find_options = FindOptions::builder().sort(doc! {"title": 1}).build();
        let mut cursor = coll.find(filter, None).await?;
        
        if let Some(rss) = cursor.try_next().await? {
            return Ok(rss.description);
        }
        Ok("".to_string())
    }


}





