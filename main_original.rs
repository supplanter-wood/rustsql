//assertEquals! with inserted customerId against SQL searched customerId
use mysql::*; 
use mysql::prelude::*;


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

    // Let's create a table for payments.
conn.query_drop(
    r"CREATE TEMPORARY TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )")?;

let payments = vec![
    Payment { customer_id: 1, amount: 2, account_name: None },
    Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
    Payment { customer_id: 5, amount: 6, account_name: None },
    Payment { customer_id: 7, amount: 8, account_name: None },
    Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
];

// Now let's insert payments to the database
conn.exec_batch(
    r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)",
    payments.iter().map(|p| params! {
        "customer_id" => p.customer_id,
        "amount" => p.amount,
        "account_name" => &p.account_name,
    })
)?;

// Let's select payments from database. Type inference should do the trick here.
let selected_payments = conn
    .query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    )?;

// Let's make sure, that `payments` equals to `selected_payments`.
// Mysql gives no guaranties on order of returned rows
// without `ORDER BY`, so assume we are lucky.
assert_eq!(payments, selected_payments);
println!("Yay!");


Ok(())
}

// pub struct DB {
//     pub conn: PooledConn,
// }

// struct Opts {
//     url: String,
//     ip: Option<String>,
//     tcp_port: u16,
//     socket: Option<String>,
//     user: String,
//     pass: String,
//     db_name: String,
// }
// //Impl struct to support parsing of values
// //Otherwise no use, create manually to dry/test run
// impl Opts {
//     fn from_url(&mut self) -> &mut String {
//         &mut self.url
//     }

//     fn get_ip_or_hostname(&mut self) -> &mut Option<String> {
//         &mut self.ip
//     }

//     fn get_tcp_port(&self) -> &mut u16 {
//         &mut self.tcp_port
//     }

//     fn get_socket(&self) -> &mut Option<String> {
//         &mut self.socket
//     }

//     fn get_user(&self) -> &mut String {
//         &mut self.user
//     }

//     fn get_pass(&self) -> &mut String {
//         &mut self.pass
//     }

//     fn get_db_name(&self) -> &mut String {
//         &mut self.db_name
//     }
// }
// //Insert result outcome here
// fn create_table(conn: PooledConn) {
//     //Creating table for payments
//     conn.query_drop(
//         r"CREATE TEMPORARY TABLE payment (
//         customer_id int not null,
//         amount int not null,
//         account_name text
//     )",
//     )?;
// }

// fn connection() -> Result<PooledConn> {
//     let url = "mysql://root:password@localhost:3307/db_name";

//     let pool = Pool::new(url)?;

//     let mut conn = pool.get_conn()?;

//     Ok(conn)
// }

// fn connection2() {
//     let url = "blahblah";

//     let pool = get_pool(url).unwrap();
// }

//Pass connection into main function
