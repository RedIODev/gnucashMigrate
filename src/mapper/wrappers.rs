use std::{fmt::{Debug, Display}, num::ParseIntError, str::FromStr};

use chrono::{DateTime, Local, NaiveDate, ParseError};
use uuid::Uuid;

use crate::{common::SumExtender, mapper::xml_bindings::SplitAccount, utils::{Fixed, FixedError, FromString, Single, Update}};

use super::xml_bindings::{ActId, ActParent, ActSlots, ActSlotsSlot, ActSlotsSlotSlotValue, GncAccount, GncBook, GncTransaction, SplitId, TrnCurrency, TrnDateEntered, TrnDatePosted, TrnId, TrnSlots, TrnSlotsSlot, TrnSlotsSlotSlotValue, TrnSplit, TrnSplits};

type FI32 = Fixed<i32, u16>;
//todo: check that only Currency EUR can be used
//todo: change all update functions to only use update not from.
//todo: fix reference cycle in Parsed types.

///
/// 
/// Error type
/// 
/// 

#[derive(Debug)]
pub enum Error {
    UuidError(uuid::Error),
    ParseIntError(ParseIntError),
    DateTimeError(ParseError),
    MissingValue(&'static str),
    InvalidValue(&'static str),
    FixedError(FixedError<ParseIntError>),
}

impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Error::UuidError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

impl From<FixedError<ParseIntError>> for Error {
    fn from(value: FixedError<ParseIntError>) -> Self {
        Error::FixedError(value)
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Error::DateTimeError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UuidError(e) => write!(f, "UuidError:{}",e),
            Self::ParseIntError(e) => write!(f, "ParseIntError:{}", e),
            Self::DateTimeError(e) => write!(f, "DateTimeError:{}", e),
            Self::MissingValue(v) => write!(f, "MissingValue:{}", v),
            Self::InvalidValue(v) => write!(f, "InvalidValue:{}", v),
            Self::FixedError(e) => write!(f, "FixedError:{}", e),
        }
    }
}

impl std::error::Error for Error {}


///
/// 
/// Helper
/// 
/// 

const PLACEHOLDER:&str = "placeholder";
const GUID:&str = "guid";
const STRING:&str = "string";
const DATE_POSTED:&str = "date-posted";
const G_DATE:&str = "gdate";
const DATE_FORMAT:&str = "%Y-%m-%d";
const DATE_TIME_FORMAT:&str ="%Y-%m-%d %H:%M:%S +0000";
const GNU_CASH_VER:&str = "2.0.0";
const CURRENCY:&str = "CURRENCY";
const CURRENCY_ID:&str = "EUR";
const DEFAULT_RECONSILE_STATE:&str = "n";
const DEFAULT_TRANSACTION_VALUE:&str = "0/100";

trait MissingValue {
    fn missing(&self, value:&'static str) -> Result<&String, Error>;
}

impl MissingValue for Option<String> {
    fn missing(&self, value:&'static str) -> Result<&String, Error> {
        self.as_ref().ok_or(Error::MissingValue(value))
    }
}

fn parse_uuid(source:&Option<String>, mssing:&'static str) -> Result<Uuid, Error> {
    Ok::<_,Error>(Uuid::from_string(source.missing(mssing)?)?)
}

fn update_readonly(slots:&mut ActSlots, readonly:bool) {
    slots.slot.retain(|slot| slot.slot_key.as_str() != PLACEHOLDER);
    if readonly {
        slots.slot.push(ActSlotsSlot { 
            text: None, 
            slot_key: PLACEHOLDER.to_owned(), 
            slot_value: ActSlotsSlotSlotValue { 
                slot_value_type: STRING.to_owned(), 
                text: Some(true.to_string()) 
            } 
        });
    }
}

fn update_date_posted(slots:&mut TrnSlots, time: NaiveDate) {
    slots.slot.retain(|slot| slot.slot_key.as_str() != DATE_POSTED);
    slots.slot.push(TrnSlotsSlot {
        text: None,
        slot_key: DATE_POSTED.to_owned(),
        slot_value: TrnSlotsSlotSlotValue {
            slot_value_type: G_DATE.to_owned(),
            text: None, 
            gdate: time.format(DATE_FORMAT).to_string()
        }
    });
}

trait GuidWrapper {
    fn from_uuid(id:Uuid) -> Self;

    fn empty() -> Self;
}

impl<T:From<(String, Option<String>)>> GuidWrapper for T {
    fn from_uuid(id:Uuid) -> Self {
        (GUID.to_owned(), Some(id.as_simple().to_string())).into()
    }

    fn empty() -> Self {
        (GUID.to_owned(), None).into()
    }
}

// trait TupleTranspose<T,S> {
//     fn transpose(self) -> Option<(T,S)>;
// }

// impl<T,S> TupleTranspose<T,S> for (Option<T>, Option<S>) {
//     fn transpose(self) -> Option<(T,S)> {
//         match self {
//             (Some(t), Some(s)) => Some((t,s)),
//             _ => None
//         }
//     }
// }



///
/// 
/// Parsed structs
/// 
/// 

#[derive(Debug)]
pub struct Book<'a> {
    accounts:Vec<Account<'a>>,
    transactions:Vec<Transaction<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Account<'a> {
    name: &'a str,
    id: Uuid,
    acc_type:&'a str,
    parent: Option<Uuid>,
    is_readonly: bool,
}

#[derive(Debug)]
pub struct Transaction<'a> {
    id: Uuid,
    date_posted:DateTime<Local>,
    date_entered:DateTime<Local>,
    description: &'a str,
    splits: Vec<Split<'a>>,
}



#[derive(Debug)]
pub struct Split<'a> {
    id: Uuid,
    memo: Option<&'a str>,
    value: FI32,
    quantity: FI32,
    account: Uuid,
}

///
/// 
/// Defaults
/// 
/// 
impl Default for GncAccount {
    fn default() -> Self {
        Self { 
            version: GNU_CASH_VER.to_owned(), 
            text: None, 
            act_slots: None, 
            act_description: None, 
            act_name: "".to_owned(), 
            act_id: ActId::empty(), 
            act_type: "".to_owned(), 
            act_parent: None, 
            act_commodity_scu: None, 
            act_commodity: None 
        }
    }
}

impl Default for GncTransaction {
    fn default() -> Self {
        Self { 
            version: GNU_CASH_VER.to_owned(), 
            text: None, 
            trn_num: None, 
            trn_id: TrnId::empty(), 
            trn_currency: TrnCurrency {
                text: None,
                cmdty_space: CURRENCY.to_owned(),
                cmdty_id: CURRENCY_ID.to_owned(),
            }, 
            trn_date_posted: TrnDatePosted {
                text: None,
                ts_date: "".to_owned(),
            }, 
            trn_date_entered: TrnDateEntered { 
                text: None, 
                ts_date: "".to_owned() 
            }, 
            trn_description: "".to_owned(), 
            trn_slots: None, 
            trn_splits: TrnSplits { 
                text: None, 
                trn_split: Vec::new() 
            }
        }
    }
}

impl Default for TrnSplit {
    fn default() -> Self {
        Self { 
            text: None, 
            split_memo: None, 
            split_id: SplitId::empty(), 
            split_reconciled_state: DEFAULT_RECONSILE_STATE.to_owned(), 
            split_value: DEFAULT_TRANSACTION_VALUE.to_owned(), 
            split_quantity: DEFAULT_TRANSACTION_VALUE.to_owned(), 
            split_account: SplitAccount::empty() 
        }
    }
}

///
/// 
/// Updates
/// 
/// 

impl GncBook {
    pub fn update(&mut self, book:Book) {
        self.gnc_account.iter_mut()
    }
}

impl GncAccount {
    pub fn update(&mut self, acc:Account) {
        self.act_name = acc.name.to_owned();
        self.act_id = ActId::from_uuid(acc.id);
        self.act_type = acc.acc_type.to_owned();
        self.act_parent = acc.parent.map(ActParent::from_uuid);
        self.act_slots.update(|slot| update_readonly(slot, acc.is_readonly));
    }
}

impl GncTransaction {
    pub fn update(&mut self, trn: Transaction) {
        self.trn_id = TrnId::from_uuid(trn.id);
        self.trn_date_posted = TrnDatePosted { text: None, ts_date: trn.date_posted.format(DATE_TIME_FORMAT).to_string() };
        self.trn_slots.update(|slot| update_date_posted(slot, trn.date_posted.date_naive()));
        self.trn_date_entered = TrnDateEntered { text: None, ts_date: trn.date_entered.format(DATE_TIME_FORMAT).to_string() };
        self.trn_description = trn.description.to_owned();
        self.trn_splits = TrnSplits { text: None, trn_split: trn.splits.into_iter().map(TrnSplit::from).collect() };
    }
}

impl TrnSplit {
    pub fn update(&mut self, split: Split) {
        self.split_id = SplitId::from_uuid(split.id);
        self.split_memo = split.memo.map(str::to_string);
        self.split_value = split.value.to_string();
        self.split_quantity = self.split_quantity.to_string();
        self.split_account = SplitAccount::from_uuid(split.account);
    }
}

///
/// 
/// Froms
/// 
/// 

impl<'a> From<Account<'a>> for GncAccount {
    fn from(value: Account<'a>) -> Self {
        let mut acc = GncAccount::default();
        acc.update(value);
        acc
    }
}


impl<'a> From<Transaction<'a>> for GncTransaction {
    fn from(value: Transaction<'a>) -> Self {
        let mut trn = GncTransaction::default();
        trn.update(value);
        trn
    }
}

impl<'a> From<Split<'a>> for TrnSplit {
    fn from(value: Split<'a>) -> Self {
        let mut split = TrnSplit::default();
        split.update(value);
        split
    }
}

///
/// 
/// TryFroms
/// 
/// 

impl<'a> TryFrom<&'a GncBook> for Book<'a>  {
    type Error = Error;

    fn try_from(value: &'a GncBook) -> Result<Self, Self::Error> {
        Ok(Book { 
            accounts: value.gnc_account.iter().map(Account::try_from).collect::<Result<_,_>>()?, 
            transactions: value.gnc_transaction.iter().map(Transaction::try_from).collect::<Result<_,_>>()?
        })
    }
}

impl<'a> TryFrom<&'a GncAccount> for Account<'a> {
    type Error = Error;

    fn try_from(value: &'a GncAccount) -> Result<Self, Self::Error> {
        Ok(Account { 
            name: &value.act_name, 
            id: Uuid::from_string(value.act_id.text.missing("act:id")?)?, 
            acc_type: &value.act_type, 
            parent: value.act_parent.as_ref()
                    .map(|parent| &parent.text)
                    .map(|id| parse_uuid(id, "act:parent->value"))
                    .transpose()?,
            is_readonly: value.act_slots.as_ref()
                        .and_then(|slots| slots.slot.iter()
                            .find(|slot| slot.slot_key==PLACEHOLDER))
                        .map(|slot| Ok::<_,Error>(*slot.slot_value.text.missing("slot:value->value")?==true.to_string()))
                        .transpose()?
                        .unwrap_or(false)
        })
    }
}

impl<'a> TryFrom<&'a GncTransaction> for Transaction<'a> {
    type Error = Error;

    fn try_from(value: &'a GncTransaction) -> Result<Self, Self::Error> {
        Ok(Transaction { 
            id: parse_uuid(&value.trn_id.text, "trn:id")?,
            date_posted: DateTime::from_str(&value.trn_date_posted.ts_date)?, 
            date_entered: DateTime::from_str(&value.trn_date_entered.ts_date)?, 
            description: &value.trn_description, 
            splits: value.trn_splits.trn_split.iter().map(Split::try_from).collect::<Result<_, _>>()?
        })
    }
}

impl<'a> TryFrom<&'a TrnSplit> for Split<'a> {
    type Error = Error;

    fn try_from(value: &'a TrnSplit) -> Result<Self, Self::Error> {
        Ok(Split { 
            id: parse_uuid(&value.split_id.text, "split:id")?,
            memo: value.split_memo.as_deref(),
            value: value.split_value.parse()?,
            quantity: value.split_quantity.parse()?,
            account: parse_uuid(&value.split_account.text, "split:account")? 
        })
    }
}

///
/// 
/// Displays
/// 
/// 

impl<'a> Display for Account<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Act <name={}, id={}, type={}, parent={:?}, readonly={}>",
                self.name, self.id, self.acc_type, self.parent, self.is_readonly)
    }
}

impl<'a> Display for Transaction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trn<id={}, posted={}, entered={}, desc={}, {:?}>",
                self.id, self.date_posted, self.date_entered, self.description, self.splits)
    }
}

impl<'a> Display for Split<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Split<id={}, memo={:?}, value={}, quantity={}, account={}",
                self.id, self.memo, self.value, self.quantity, self.account)
    }
}

///
/// 
/// Others
/// 
/// 

impl<'a> Book<'a> {
    pub fn get_account_total(&self, acc:&Account) -> (FI32, FI32) {
        let x: (SumExtender<_>, SumExtender<_>) = self.transactions.iter()
                .flat_map(|trn| &trn.splits)
                .filter(|split|split.account == acc.id)
                .map(|split| (split.value, split.quantity))
                .unzip();
        (*x.0, *x.1)
    }

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    pub fn translate_account<'b>(source:&Book, target:&'b Book, account:&Account) -> Option<&'b Account<'b>> {
        target.accounts.iter()
                    .filter_map(|acc| Some(acc).filter(|acc| Self::check_parents(source, target, &acc, account)))
                    .single()
    }


    fn check_parents(source: &Book, target: &Book, source_acc:&Account, target_acc:&Account) -> bool {
        if source_acc.name != target_acc.name {
            return false;
        }
        let (Some(source_parent), Some(target_parent)) = (source_acc.parent, target_acc.parent) else {
            return source_acc.parent.is_some() == target_acc.parent.is_some();
        };

        let source_parent = source.accounts.iter().find(|acc| acc.id == source_parent);
        let target_parent = target.accounts.iter().find(|acc| acc.id == target_parent);
        let (Some(source_parent), Some(target_parent)) = (source_parent, target_parent) else {
            panic!("parent not in book");
        };
        Self::check_parents(source, target, source_parent, target_parent)
    }
}













