use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = sqlx::sqlite::SqlitePool::connect("mydb.db").await?;

    let N = 1_000_000;
    let mut ab_list = vec![];
    for i in 0..N {
        ab_list.push((i + 1, i + 1));
    }

    let timer = Instant::now();
    for f in 1..=100 {
        let mut trans = pool.begin().await?;
        let mut values = vec![];
        let mut q = "insert into mydb (a, b, c, d, e, f) values ".to_owned();
        for &(a, b) in &ab_list {
            let v = format!("({}, {}, 1, 1, 1.0, {})", a, b, f);
            values.push(v);
        }
        let values = values.join(",");
        q.push_str(&values);
        sqlx::query(&q).execute(&mut trans).await?;
        trans.commit().await?;
    }
    println!("=> {}ms", timer.elapsed().as_millis());

    Ok(())
}
