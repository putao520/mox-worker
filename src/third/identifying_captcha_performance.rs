use crossbeam_queue::SegQueue;

static Q: SegQueue<i64> = SegQueue::<i64>::new();

pub fn push_identifying_captcha_performance(v: i64) {
    Q.push(v);
}

pub async fn start_identifying_captcha_performance() {
    tokio::spawn(async{
        let mut avg: i64 = 0;
        let mut last_avg: i64 = 0;
        loop {
            if let Some(v) = Q.pop() {
                let n = chrono::Utc::now().timestamp();
                // 60s 输出一次
                if n - last_avg > 60 {
                    println!("identifying_captcha_performance: {}", avg);
                }
                avg = (avg + v)/2;
                last_avg = n;
            }
        }
    }).await.unwrap();
}