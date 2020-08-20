
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Transaction{

    #[serde(alias = "Posting Date")]
    #[serde(alias = "Processed Date")]
    posting_date: String,
    #[serde(alias = "Transaction Type")]
    #[serde(alias = "Credit or Debit")]
    transaction_type: String,
    #[serde(alias = "Amount")]
    amount: f64,
    #[serde(alias = "Check Number")]
    check_number: String,
    #[serde(alias = "Description")]
    description: String,

}

pub fn pull_transactions_from_csv() -> Vec<f64> {

    let rdr = csv::Reader::from_path("/Users/tjohnson/Documents/ExportedTransactions-revised.csv");

    rdr.unwrap().deserialize().into_iter().map( |t| {
        let transaction: Transaction = t.unwrap();
        transaction.amount
    }).collect()
}
pub fn pull_transactions_for_silverlake() -> Vec<f64> {

    let rdr = csv::Reader::from_path("/Users/tjohnson/Downloads/Reg_DDA_0001_Transactions_2020-01-01_2020-08-20.csv");

    rdr.unwrap().deserialize().into_iter().map( |t| {
        let transaction: Transaction = t.unwrap();
        transaction.amount
    }).collect()
}