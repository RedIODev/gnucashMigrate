use derive_more::derive::From;
use serde::{Deserialize, Serialize};



#[derive(From, Debug, Serialize, Deserialize)]
pub struct GncV2 {
    #[serde(rename = "@xmlns:gnc")]
    pub xmlns_gnc: String,
    #[serde(rename = "@xmlns:act")]
    pub xmlns_act: String,
    #[serde(rename = "@xmlns:book")]
    pub xmlns_book: String,
    #[serde(rename = "@xmlns:cd")]
    pub xmlns_cd: String,
    #[serde(rename = "@xmlns:cmdty")]
    pub xmlns_cmdty: String,
    #[serde(rename = "@xmlns:price")]
    pub xmlns_price: String,
    #[serde(rename = "@xmlns:slot")]
    pub xmlns_slot: String,
    #[serde(rename = "@xmlns:split")]
    pub xmlns_split: String,
    #[serde(rename = "@xmlns:sx")]
    pub xmlns_sx: String,
    #[serde(rename = "@xmlns:trn")]
    pub xmlns_trn: String,
    #[serde(rename = "@xmlns:ts")]
    pub xmlns_ts: String,
    #[serde(rename = "@xmlns:fs")]
    pub xmlns_fs: String,
    #[serde(rename = "@xmlns:bgt")]
    pub xmlns_bgt: String,
    #[serde(rename = "@xmlns:recurrence")]
    pub xmlns_recurrence: String,
    #[serde(rename = "@xmlns:lot")]
    pub xmlns_lot: String,
    #[serde(rename = "@xmlns:addr")]
    pub xmlns_addr: String,
    #[serde(rename = "@xmlns:billterm")]
    pub xmlns_billterm: String,
    #[serde(rename = "@xmlns:bt-days")]
    pub xmlns_bt_days: String,
    #[serde(rename = "@xmlns:bt-prox")]
    pub xmlns_bt_prox: String,
    #[serde(rename = "@xmlns:cust")]
    pub xmlns_cust: String,
    #[serde(rename = "@xmlns:employee")]
    pub xmlns_employee: String,
    #[serde(rename = "@xmlns:entry")]
    pub xmlns_entry: String,
    #[serde(rename = "@xmlns:invoice")]
    pub xmlns_invoice: String,
    #[serde(rename = "@xmlns:job")]
    pub xmlns_job: String,
    #[serde(rename = "@xmlns:order")]
    pub xmlns_order: String,
    #[serde(rename = "@xmlns:owner")]
    pub xmlns_owner: String,
    #[serde(rename = "@xmlns:taxtable")]
    pub xmlns_taxtable: String,
    #[serde(rename = "@xmlns:tte")]
    pub xmlns_tte: String,
    #[serde(rename = "@xmlns:vendor")]
    pub xmlns_vendor: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "count-data")]
    pub gnc_count_data: GncV2GncCountData,
    #[serde(rename = "book")]
    pub gnc_book: GncBook,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct GncV2GncCountData {
    #[serde(rename = "@type")]
    pub cd_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct GncBook {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "id")]
    pub book_id: BookId,
    #[serde(rename = "slots")]
    pub book_slots: Option<BookSlots>,
    #[serde(default)]
    #[serde(rename = "count-data")]
    pub gnc_count_data: Vec<GncBookGncCountData>,
    #[serde(default)]
    #[serde(rename = "commodity")]
    pub gnc_commodity: Vec<GncCommodity>,
    #[serde(default)]
    #[serde(rename = "account")]
    pub gnc_account: Vec<GncAccount>,
    #[serde(default)]
    #[serde(rename = "transaction")]
    pub gnc_transaction: Vec<GncTransaction>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct BookId {
    #[serde(rename = "@type")]
    pub book_id_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct BookSlots {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(default)]
    pub slot: Vec<BookSlotsSlot>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct BookSlotsSlot {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "key")]
    pub slot_key: String,
    #[serde(rename = "value")]
    pub slot_value: BookSlotsSlotSlotValue,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct BookSlotsSlotSlotValue {
    #[serde(rename = "@type")]
    pub slot_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(default)]
    pub slot: Vec<SlotValueSlot>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct SlotValueSlot {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "key")]
    pub slot_key: String,
    #[serde(rename = "value")]
    pub slot_value: SlotValueSlotSlotValue,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct SlotValueSlotSlotValue {
    #[serde(rename = "@type")]
    pub slot_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct GncBookGncCountData {
    #[serde(rename = "@type")]
    pub cd_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct GncCommodity {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "space")]
    pub cmdty_space: String,
    #[serde(rename = "id")]
    pub cmdty_id: String,
    #[serde(rename = "fraction")]
    pub cmdty_fraction: Option<String>,
    #[serde(rename = "xcode")]
    pub cmdty_xcode: Option<String>,
    #[serde(rename = "name")]
    pub cmdty_name: Option<String>,
    #[serde(rename = "get_quotes")]
    pub cmdty_get_quotes: Option<CmdtyGetQuotes>,
    #[serde(rename = "quote_source")]
    pub cmdty_quote_source: Option<String>,
    #[serde(rename = "quote_tz")]
    pub cmdty_quote_tz: Option<CmdtyQuoteTz>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct CmdtyGetQuotes {
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct CmdtyQuoteTz {
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct GncAccount {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "slots")]
    pub act_slots: Option<ActSlots>,
    #[serde(rename = "description")]
    pub act_description: Option<String>,
    #[serde(rename = "name")]
    pub act_name: String,
    #[serde(rename = "id")]
    pub act_id: ActId,
    #[serde(rename = "type")]
    pub act_type: String,
    #[serde(rename = "parent")]
    pub act_parent: Option<ActParent>,
    #[serde(rename = "commodity-scu")]
    pub act_commodity_scu: Option<String>,
    #[serde(rename = "commodity")]
    pub act_commodity: Option<ActCommodity>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct ActSlots {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(default)]
    pub slot: Vec<ActSlotsSlot>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct ActSlotsSlot {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "key")]
    pub slot_key: String,
    #[serde(rename = "value")]
    pub slot_value: ActSlotsSlotSlotValue,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct ActSlotsSlotSlotValue {
    #[serde(rename = "@type")]
    pub slot_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct ActId {
    #[serde(rename = "@type")]
    pub act_id_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct ActParent {
    #[serde(rename = "@type")]
    pub act_parent_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct ActCommodity {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "space")]
    pub cmdty_space: String,
    #[serde(rename = "id")]
    pub cmdty_id: String,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct GncTransaction {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "num")]
    pub trn_num: Option<String>,
    #[serde(rename = "id")]
    pub trn_id: TrnId,
    #[serde(rename = "currency")]
    pub trn_currency: TrnCurrency,
    #[serde(rename = "date-posted")]
    pub trn_date_posted: TrnDatePosted,
    #[serde(rename = "date-entered")]
    pub trn_date_entered: TrnDateEntered,
    #[serde(rename = "description")]
    pub trn_description: String,
    #[serde(rename = "slots")]
    pub trn_slots: Option<TrnSlots>,
    #[serde(rename = "splits")]
    pub trn_splits: TrnSplits,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnId {
    #[serde(rename = "@type")]
    pub trn_id_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnCurrency {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "space")]
    pub cmdty_space: String,
    #[serde(rename = "id")]
    pub cmdty_id: String,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnDatePosted {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "date")]
    pub ts_date: String,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnDateEntered {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "date")]
    pub ts_date: String,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnSlots {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(default)]
    pub slot: Vec<TrnSlotsSlot>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnSlotsSlot {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "key")]
    pub slot_key: String,
    #[serde(rename = "value")]
    pub slot_value: TrnSlotsSlotSlotValue,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnSlotsSlotSlotValue {
    #[serde(rename = "@type")]
    pub slot_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub gdate: String,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnSplits {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(default)]
    #[serde(rename = "split")]
    pub trn_split: Vec<TrnSplit>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct TrnSplit {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "memo")]
    pub split_memo: Option<String>,
    #[serde(rename = "id")]
    pub split_id: SplitId,
    #[serde(rename = "reconciled-state")]
    pub split_reconciled_state: String,
    #[serde(rename = "value")]
    pub split_value: String,
    #[serde(rename = "quantity")]
    pub split_quantity: String,
    #[serde(rename = "account")]
    pub split_account: SplitAccount,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct SplitId {
    #[serde(rename = "@type")]
    pub split_id_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(From, Debug, Serialize, Deserialize)]
pub struct SplitAccount {
    #[serde(rename = "@type")]
    pub split_account_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

