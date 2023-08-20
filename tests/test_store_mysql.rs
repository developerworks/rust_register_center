use rust_register_center::store::mysql::{MicroService, MysqlStore};
use sqlx::mysql::MySqlPoolOptions;

// 单元测试函数
#[actix_rt::test]
async fn test_get_micro_service() {
    // 创建一个内存数据库的连接池用于测试
    let pool = MySqlPoolOptions::new()
        .max_connections(2)
        .connect("mysql://root:root@localhost/test")
        .await
        .expect("Failed to create pool for testing");

    // 创建一个 MysqlStore 实例用于测试
    let store = MysqlStore { pool };

    // 调用 get 方法，传入测试用的 service 和 url
    let result = store.get("service_name_1", "service_url_1").await;

    // 预期的 MicroService 结构
    let expected_micro_service = MicroService {
        name: "service_name_1".to_string(),
        url: "service_url_1".to_string(),
    };

    // 断言返回结果是否与预期一致
    assert_eq!(result, vec![expected_micro_service]);
}
