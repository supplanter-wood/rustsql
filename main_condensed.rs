//assertEquals! with inserted customerId against SQL searched customerId
use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
    //Add or remove variables depending on the parameters needed for transaction or authentication
}

fn main() -> Result<()> {
    let url: &str = "mysql://root:password@127.0.0.1:3306/dbx";
    let opts: Opts = Opts::from_url(url)?;
    let pool: Pool = Pool::new(opts)?;

    let mut conn: PooledConn = pool.get_conn()?;
    condensed_main();

    Ok(())
}

fn create_table(mut conn: PooledConn) -> Result<()> {
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )",
    )?;
    Ok(())
}

fn create_vector() -> Vec<Payment> {
    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];
    return payments;
}

fn insert_data_into_table(mut conn: PooledConn, payments: Vec<Payment>) -> Result<()> {
    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)",
        payments.iter().map(|p| {
            params! {
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }
        }),
    )?;
    Ok(())
}

fn query_results(mut conn: PooledConn) -> Result<()> {
    let selected_payments = conn.query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| Payment {
            customer_id,
            amount,
            account_name,
        },
    )?;

    Ok(())
}

fn condensed_main() -> Result<()> {
    connection();
    create_table(conn);
    create_vector();
    insert_data_into_table(conn, payments);
    query_results(conn);
    Ok(())
}
