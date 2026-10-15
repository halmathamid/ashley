use ashley::{delete, insert, select, update, Value};

fn main() {
    let select_q = select("users")
        .columns(["id", "name", "email"])
        .filter("active", true)
        .filter("role", "admin")
        .order_by("name")
        .limit(10)
        .build();

    let insert_q = insert("users")
        .columns(["name", "email"])
        .values(["Ada", "ada@example.com"])
        .returning(["id"])
        .build();

    let update_q = update("users")
        .set("name", "Grace")
        .set("active", false)
        .filter("id", 42_i64)
        .returning(["id", "name"])
        .build();

    let delete_q = delete("sessions")
        .filter("user_id", 7_i64)
        .filter("expired", true)
        .build();

    print_query("SELECT", &select_q.sql, &select_q.params);
    print_query("INSERT", &insert_q.sql, &insert_q.params);
    print_query("UPDATE", &update_q.sql, &update_q.params);
    print_query("DELETE", &delete_q.sql, &delete_q.params);
}

fn print_query(kind: &str, sql: &str, params: &[Value]) {
    println!("--- {kind} ---");
    println!("{sql}");
    println!("params: {params:?}");
    println!();
}
