

只是一个 server，让浏览器(firefox)能访问到mysql和mongodb中数据。


# 代码之前

`cargo new rss_f`

库

下面的应该是 mysql的， 不是 mysqlx 。。 。。但是 例子中 怎么连了个 3307 。。  
而且 自带 Pool 。 c++的 不知道有没有， 反正没看到。  
crate 确实很不错。 c++ 找 库 太烦了。  
https://docs.rs/mysql/latest/mysql/#  
https://crates.io/crates/mysql  
这2个一样的。  


https://crates.io/crates/sqlx  
rust 用这个的，这个可以连 postgresql，mysql 等。。 但 写都写完了。  
https://dev.to/behainguyen/rust-mysql-connect-execute-sql-statements-and-stored-procs-using-crate-sqlx-3djk  
其他人的 代码


mongodb的就比较奇怪，是 mongodb的，不是 rust的
https://www.mongodb.com/docs/drivers/rust/current/
https://crates.io/crates/mongodb
内容一样的。


看 crates 上， mysql的人好少, 比 mongodb的人都少。 而且 mysql的 版本 是真的高 v25.0.1 
postgreSQL 爆杀 mysql， 10倍。
rust 用的最多的是 sqlite， 吊打 potgreSQL, 2倍


server
- hyper
- actix-web
- tikio?  不算，是底层异步框架， h2, hyper 都用了这个。
- h2
- tide

看下载量 hyper 最强 ， 和h2 差不多。   远超  actix-web

h2 和 hyper 是同一个组织的， github上的 仓库owner 是同一个组织

star 比 actix-web 少。

h2  https://crates.io/crates/h2  中有一句话：
This crate is now used by hyper, which will provide all of these features.

所以下载量 是由于 hyper被下载，所以 h2 也下载了？

hyper vs. actix-web

。。又找到一个 axum，

有2类，
一类是 像java的注释一样，直接将 监听的路径 注释到 方法上
一类 像 gin 那样， main方法中 new一个 router，然后 手工增加 监听的路径，handler。


axum 由 tokio团队 开发， 基于hyper 和 tokio。 不过 才 0.7.5 。 而且是 3个月前更新。 是 gin模式


就axum吧。



---

`cargo add axum`
`cargo add mysql`
`cargo add mongodb`


缺个日志？  html_template ?

https://crates.io/crates/yew-template
https://crates.io/crates/maud
https://crates.io/crates/stpl



https://crates.io/crates/log


`cargo add maud`
`cargo add log`
`cargo add env_logger`   !!..

...报错了， 说没有 serde。

`cargo add serde`
`cargo add tokio`
`cargo add tracing-subscriber`

。。上面是为了 运行 axum 的例子。  target 1.8g .. 很难理解。


`cargo add futures`   mongodb async



## 功能

抄 docs.rs 。
左侧是 列表 ， 点击左侧的 某项后， 就显示 link + 内容。

左侧只需要 mysql， 点击后 才需要 mongodb

每个 rss源 默认展现10条，要有个 more功能。 显示更多的数据。   最好还要一个  折叠功能。

。难搞。 我不会。。 java 我知道 main-layout, 自己 增加 left-layout，之类的， 进行布局。
rust呢。。 手写肯定不行。 这就是前端啊， 布局 都是前端自己搞， 从后端获得数据。

实际上也行。换一种方法，列表占一个页面， 点击后，新开页面 看 内容。
这样应该更好，因为 用的sway，所以 界面挺窄的， 而且  标题 十几 二十个字的话，  左侧是放不下的。

就这样吧， 先能看， 布局 不管了。

内容 需要限制 最大宽度

还有就是， 列表。
more 会跳新页。 单独展现 这个 rss源的数据 (。这里需要分页。)。  原地多展现x条的，做不来。













# 代码中

应该3个文件， http，mysql，mongodb 。再说。

query param  
https://github.com/tokio-rs/axum/blob/main/examples/query-params-with-empty-strings/src/main.rs

uri param，  return html  
https://zhuanlan.zhihu.com/p/675184852

mysql  
https://docs.rs/mysql/latest/mysql/

mysql update  
https://blog.csdn.net/techdashen/article/details/131735261

mongodb  
https://crates.io/crates/mongodb#installation  
还分为 异步api，同步api。 默认是 异步， 同步的话，需要改 cargo.toml
用同步， 异步的find 挺麻烦的。

```text
Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.
```
。。还是得 异步。。


需要修改数据库了， 因为 rss_source 的 中文名 没有。需要人工录入。


------

缺少 时间，文章的发布时间。  但是 没有从 rss中抓 pub-data。  可能使用 rss_item 的 createtime (入库时间) 来 稍微 代替下。

展示下 文章的状态，主要就是 是否已阅。

现在看起来，pubtime 还是非常重要的。


sleep 时 输出 时间 和 pid，  
时间是 判断 有没有执行。  
pid 是为了 kill。 通过 `ps -ef|grep rss` 找不到rss。。



## ddl

`alter table rss.rss_source add column name_cn varchar(32) not null;`

..最好还要一个 字段 用于排序 rss源。。

还有 rss_item 增加字段用于 表示 文章等级。  
-1:删除，0:初始，1:已阅，2:收藏
这些按钮不需要新开页面的。 点一下就可以了。 没写过。。



`alter table rss.rss_source add column priority int default 0 not null;`

`alter table rss.rss_item add column pubdate varchar(45) not null default '';`
`alter table rss.rss_item add column status int not null default 0;`




# exe

`RUST_LOG=INFO cargo run`

下 260+ 个 板条箱。

。。忘记改 cargo的源了。 下到最后几个 警告了。。 怪不得 非常慢。。

.。最后几个死活下不了。。 换源后， 得重新下。。擦。



rust 不爽的就是 这个体积太大了。



可以直接 `cargo run`

release ：  `cargo run --release` 会重新编译 所有的 依赖。 一共365个。。  cpu占满。。

```text
du -sh target/
3.3G	target/

du -sh target/debug/
2.7G	target/debug/

du -sh target/release/
672M	target/release/

du -sh target/release/rss_f
15M	target/release/rss_f

du -sh target/debug/rss_f
144M	target/debug/rss_f
```


`cargo clean`
Removed 6876 files, 3.8GiB total
直接删除了 target 文件夹。

还好我复制了 rss_f 

不过这里的 3.8 和上面的 3.3 对不起来， 不知道 另外500mb 是哪里？ 总不会 把 ~/.cargo 中 下载的库 也删除了吧。 .这些还是在的。



