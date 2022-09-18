//assertEquals! with inserted customerId against SQL searched customerId
mod lib;
use mysql::prelude::*;
use mysql::*;

//potential lib of SQL commands.
fn main() -> Result<()> {
    let mut conn = lib::create_conn();
    let payments = lib::data_creation();
    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )",
    )?;

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

    // Let's select payments from database. Type inference should do the trick here.
    let selected_payments = conn.query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| lib::Payment {
            customer_id,
            amount,
            account_name,
        },
    )?;

    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");

    Ok(())
}
