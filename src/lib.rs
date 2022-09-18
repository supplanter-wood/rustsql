//use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;

use std::fs::File;

use std::io::BufReader;
use std::path::Path;

//End goal could be to make Payment parameters mutable with JSON parsing

///////////////STRUCTS////////////////
#[derive(Debug, PartialEq, Eq)]
pub struct Payment {
    pub customer_id: i32,
    pub amount: i32,
    pub account_name: Option<String>,
    //Add or remove variables depending on the parameters needed for transaction or authentication
}

#[derive(Default, Clone)]
pub struct Options {
    ip_or_hostname: String,
    user: String,
    pass: String,
    db_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SQLValues {
    #[serde(rename = "IP_OR_HOSTNAME")]
    pub ip_or_hostname: String,

    #[serde(rename = "USER")]
    pub user: String,

    #[serde(rename = "PASS")]
    pub pass: String,

    #[serde(rename = "DB_NAME")]
    pub db_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VecSQLValues {
    #[serde(rename = "SQL_VALUES")]
    list_of_sql_endpoints: Vec<SQLValues>,
}
///////////////IMPLEMENTATIONS////////////////
impl Options {
    fn ip_or_hostname_mut(&mut self) -> &mut String {
        &mut self.ip_or_hostname
    }
    fn user_mut(&mut self) -> &mut String {
        &mut self.user
    }

    fn pass_mut(&mut self) -> &mut String {
        &mut self.pass
    }

    fn db_name_mut(&mut self) -> &mut String {
        &mut self.db_name
    }
}
//Create one instance of Options instead of four???
impl SQLValues {
    fn ip_or_hostname(sql: &VecSQLValues) -> String {
        //Assigns vector values to variables
        let ip_or_hostname = &sql.list_of_sql_endpoints[0].ip_or_hostname;
        //Creates an instance of Options
        //Assigns variable values to mutable setter as string to concatenate
        let mut sql_options = options();
        *sql_options.ip_or_hostname_mut() = ip_or_hostname.to_string();

        return sql_options.ip_or_hostname;
    }
    fn user(sql: &VecSQLValues) -> String {
        //Assigns vector values to variables
        let user = &sql.list_of_sql_endpoints[0].user;
        //Creates instance of Options
        //Assigns variables value to mutable setter as string to concatenate
        let mut sql_options = options();
        *sql_options.user_mut() = user.to_string();

        return sql_options.user;
    }
    fn pass(sql: &VecSQLValues) -> String {
        //Assigns vector values to variables
        let pass = &sql.list_of_sql_endpoints[0].pass;
        let mut sql_options = options();
        *sql_options.pass_mut() = pass.to_string();
        return sql_options.pass;
    }
    fn db_name(sql: &VecSQLValues) -> String {
        let db_name = &sql.list_of_sql_endpoints[0].db_name;
        let mut sql_options = options();
        *sql_options.db_name_mut() = db_name.to_string();
        return sql_options.db_name;
    }
}

///////////////FUNCTIONS////////////////
pub fn read_sql_config_from_file<P: AsRef<Path>>(path: P) -> Result<VecSQLValues> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let sql_config: VecSQLValues = serde_json::from_reader(reader).unwrap();

    // Return the `User`.
    Ok(sql_config)
}

fn options() -> Options {
    let options: Options = Options::default();
    return options;
}

fn get_opts(sql: VecSQLValues) -> OptsBuilder {
    let sql_values = SQLValues {
        ip_or_hostname: SQLValues::ip_or_hostname(&sql),
        user: SQLValues::user(&sql),
        pass: SQLValues::pass(&sql),
        db_name: SQLValues::db_name(&sql),
    };

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(sql_values.ip_or_hostname))
        .user(Some(sql_values.user))
        .pass(Some(sql_values.pass))
        .db_name(Some(sql_values.db_name));

    return opts;
}

//Used in main
pub fn create_conn() -> PooledConn {
    let sql_config = read_sql_config_from_file("../sqlvalues.json").unwrap();
    let opts = get_opts(sql_config);
    let pool: Pool = Pool::new(opts).unwrap();
    let conn: PooledConn = pool.get_conn().unwrap();
    return conn;
}

pub fn data_creation() -> Vec<Payment> {
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
