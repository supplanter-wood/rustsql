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

pub struct DB {
    pub conn: PooledConn,
}

struct Opts {
    url: String,
    ip: Option<String>,
    tcp_port: u16,
    socket: Option<String>,
    user: String,
    pass: String,
    db_name: String,
}
//Impl struct to support parsing of values
//Otherwise no use, create manually to dry/test run
impl Opts {
    fn from_url(&mut self) -> &mut String {
        &mut self.url
    }

    fn get_ip_or_hostname(&mut self) -> &mut Option<String> {
        &mut self.ip
    }

    fn get_tcp_port(&self) -> &mut u16 {
        &mut self.tcp_port
    }

    fn get_socket(&self) -> &mut Option<String> {
        &mut self.socket
    }

    fn get_user(&self) -> &mut String {
        &mut self.user
    }

    fn get_pass(&self) -> &mut String {
        &mut self.pass
    }

    fn get_db_name(&self) -> &mut String {
        &mut self.db_name
    }
}
//Insert result outcome here
fn create_table(conn: PooledConn) {
    //Creating table for payments
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )",
    )?;
}

fn connection() -> Result<PooledConn> {
    let url = "mysql://root:password@localhost:3307/db_name";

    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    Ok(conn)
}

fn connection2() {
    let url = "blahblah";

    let pool = get_pool(url).unwrap();
}

//Pass connection into main function
fn main() {}
