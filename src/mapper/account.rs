use std::{borrow::Borrow, clone, collections::{HashMap, HashSet}, fmt::Display, iter::Peekable, num::ParseIntError, str::{FromStr, ParseBoolError}};

use uuid::Uuid;
use xml::{attribute::Attribute, name::{Name, OwnedName}, namespace::Namespace, reader::XmlEvent};

use super::{FromString, Update};

#[derive(Debug, Clone)]
pub enum Error {
    XmlError(xml::reader::Error),
    UuidError(uuid::Error),
    ParseBoolError(ParseBoolError),
    ParseIntError(ParseIntError),
    InvalidEvent(XmlEvent),
    InvalidAccountType(String),
    InvalidClosingTag(OwnedName),
    MissingElement(&'static str),
    MissingOpeningTag(OwnedName),
}


const ACCOUNT_XML_KEY:Name = Name {local_name: "account", namespace: None, prefix: Some("gnc")};
const SLOT_XML_KEY: Name = Name {local_name: "slot", namespace: None, prefix: None};
const SLOT_KEY_XML_KEY: Name = Name {local_name: "key", namespace: None, prefix:Some("slot")};
const SLOT_VALUE_XML_KEY: Name = Name {local_name: "value", namespace: None, prefix:Some("slot")};
const NAME_XML_KEY:Name = make_account_xml_key("name");
const CODE_XML_KEY:Name = make_account_xml_key("code");
const ID_XML_KEY:Name = make_account_xml_key("id");
const TYPE_XML_KEY:Name = make_account_xml_key("type");
const COMMODITY_XML_KEY:Name = make_account_xml_key("commodity");
const COMMODITY_SCU_XML_KEY:Name = make_account_xml_key("commodity-scu");
const DESCRIPTION_XML_KEY:Name = make_account_xml_key("description");
const PARENT_XML_KEY: Name = make_account_xml_key("parent");
const SLOTS_XML_KEY: Name = make_account_xml_key("slots");
const GUID_TYPE_ATTRIBUTE: Attribute = Attribute {name: Name {local_name: "type", namespace: None, prefix: None}, value: "guid"};




impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            XmlError(e) => write!(f, "XmlError: {}", e),
            UuidError(e) => write!(f, "UuidError: {}", e),
            ParseBoolError(e) => write!(f, "ParseBoolError: {}", e),
            ParseIntError(e) => write!(f, "ParseIntError: {}", e),
            InvalidEvent(e) => write!(f, "Invalid Event: {:?}", e),
            InvalidAccountType(s) => write!(f, "Invalid Account Type: {}", s),
            InvalidClosingTag(n) => write!(f, "Invalid Closing Tag: {}", n),
            MissingElement(e) => write!(f, "Missing element: {}", e),
            MissingOpeningTag(n) => write!(f, "Missing Opening Tag: {}", n)
        }
    }
}

impl std::error::Error for Error {}

impl From<xml::reader::Error> for Error {
    fn from(value: xml::reader::Error) -> Self {
        Error::XmlError(value)
    }
}

impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Error::UuidError(value)
    }
}

impl From<ParseBoolError> for Error {
    fn from(value: ParseBoolError) -> Self {
        Error::ParseBoolError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}


#[derive(Debug, Clone)]
pub enum AccountType {
    ///Offene Verbindlichkeiten
    Payable,
    ///Offene Forderungen
    Receivable,
    ///Aktiva
    Asset,
    ///Bank
    Bank,
    ///Unused
    Cash,
    ///Unused
    CreditCard,
    ///Unused
    Currency,
    ///Eigenkapital
    Equity,
    ///Aufwand
    Expense,
    ///Erträge
    Income,
    ///Fremdkapital
    Liability,
    ///Unused
    MutualFund,
    ///Unused
    Stock,
    ///ROOT
    ROOT,
}

impl FromStr for AccountType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountType::*;
        Ok(match s {
            "PAYABLE" => Payable,
            "RECEIVABLE" => Receivable,
            "ASSET" => Asset,
            "BANK" => Bank,
            "CASH" => Cash,
            "CREDIT CARD" => CreditCard,
            "CURRENCY" => Currency,
            "EQUITY" => Equity,
            "EXPENSE" => Expense,
            "INCOME" => Income,
            "LIABILITY" => Liability,
            "MUTUAL FUND" => MutualFund,
            "STOCK" => Stock,
            "ROOT" => ROOT,
            _ => { return Err(Error::InvalidAccountType(s.to_owned())); } 
        })
    }
}

#[derive(Debug, Clone)]
pub struct SlotValue {
    value_type: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct Commodity {
    space: String,
    id: String,
    scu: Option<u32>
}

impl Commodity {
    fn from_xml(iter: &mut impl Iterator<Item = XmlIterItem>) -> Result<Commodity, Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    name: String,
    code:Option<String>,
    id: Uuid,
    acc_type:AccountType,
    commodity: Option<Commodity>,
    write_protected:bool,
    description:Option<String>,
    parent: Option<Uuid>,
    other: Vec<XmlEvent>,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Account {}

type XmlIterItem = Result<XmlEvent, xml::reader::Error>;

fn skip_start(iter: &mut Peekable<impl Iterator<Item = XmlIterItem>>, xml_key: Name) -> bool {

    iter.next_if(|xml| {
        matches!(xml, Ok(XmlEvent::StartElement { name, .. }) if name.borrow() == xml_key)
    }).is_some()
}

fn parse_slots(iter: &mut impl Iterator<Item = XmlIterItem>) -> Result<HashMap<String, SlotValue>, Error> {
    let mut slots = HashMap::new();
    while let Some(event) = iter.next() {
        use XmlEvent::*;
        match event? {
            StartElement { name, .. } if name.borrow() == SLOT_XML_KEY => {
                let mut key = None;
                let mut value = None;
                for _ in 0..4 {
                    match iter.next().ok_or(Error::MissingElement("slot:x"))?? {
                        StartElement { name, ..} if name.borrow() == SLOT_KEY_XML_KEY => {
                            key = iter.next()
                                .ok_or(Error::MissingElement("text"))?
                                .map(|e| if let Characters(chars) = e { Some(chars) } else { None })?;
                        }
                        StartElement { name, attributes, .. } if name.borrow() == SLOT_VALUE_XML_KEY => {
                            value = Some(SlotValue { 
                                value_type: attributes.into_iter()
                                        .find_map(|s| if s.name.borrow() == TYPE_XML_KEY { Some(s.value) } else { None } )
                                        .ok_or(Error::MissingElement("type="))?,
                                value: iter.next()
                                    .ok_or(Error::MissingElement("text"))?
                                    .map(|e| if let Characters(chars) = e { Ok(chars) } else { Err(Error::InvalidEvent(e)) })??
                            });
                                        
                        }
                        EndElement { name } if name.borrow() == SLOT_KEY_XML_KEY || name.borrow() == SLOT_VALUE_XML_KEY => {}

                        e => return  Err(Error::InvalidEvent(e))
                    }
                }
                if let (Some(key), Some(value)) = (key, value) {
                    slots.insert(key, value);
                }
            }
            EndElement { name } if name.borrow() == SLOTS_XML_KEY => break,
            EndElement { name:_ } => {}
            e => return Err(Error::InvalidEvent(e)),
        }
    }
    todo!()
}

// fn is_start_element(iter: &mut Peekable<impl Iterator<Item = XmlIterItem>>, xml_key: Name) -> bool {
//     let Some(Ok(XmlEvent::StartElement { name, .. })) = iter.peek() else { return false; };
//     xml_key == name.borrow()
// }

fn get_characters(iter: &mut impl Iterator<Item = XmlIterItem>) -> Result<String, Error> {
    iter.next()
            .ok_or(Error::MissingElement("text"))?
            .map_err(From::from)
            .and_then(|xml| if let XmlEvent::Characters(chars) = xml {Ok(chars)} else { Err(Error::InvalidEvent(xml)) })
    
}

const fn make_account_xml_key(local_name:&'static str) -> Name {
    Name { local_name, namespace: None, prefix: Some("act") }
}

impl Account {
    pub fn from_xml(mut iter: Peekable<impl Iterator<Item = XmlIterItem>>) -> Result<Option<Account>, Error> {

        if !skip_start(&mut iter, ACCOUNT_XML_KEY) {
            return Ok(None);
        }

        let mut acc_name = None;
        let mut code = None;
        let mut id = None;
        let mut acc_type = None;
        let mut commodity = None;
        let mut commodity_scu = None;
        let mut write_protected = false;
        let mut description = None;
        let mut parent = None;
        let mut other = Vec::new();

        let mut open_elements = vec![ACCOUNT_XML_KEY.to_owned()];
        while let Some(event) = iter.next() {
            use XmlEvent::*;
            match event? {
                StartElement { name, attributes, namespace } => {
                    open_elements.push(name.clone());
                    let string_opt = match name.borrow() {
                        NAME_XML_KEY => &mut acc_name,
                        CODE_XML_KEY => &mut code,
                        ID_XML_KEY => &mut id,
                        TYPE_XML_KEY if attributes.contains(&GUID_TYPE_ATTRIBUTE.to_owned()) => &mut acc_type,
                        DESCRIPTION_XML_KEY => &mut description,
                        PARENT_XML_KEY if attributes.contains(&GUID_TYPE_ATTRIBUTE.to_owned())=> &mut parent,
                        COMMODITY_XML_KEY => { 
                            let _ = commodity.insert(Commodity::from_xml(&mut iter)?);
                            &mut Some(String::new())
                        },
                        COMMODITY_SCU_XML_KEY => &mut commodity_scu,
                        SLOTS_XML_KEY => {
                            let mut slots = parse_slots(&mut iter)?;
                            write_protected = slots.remove("placeholder").map_or(Ok(false), |val| bool::from_string(val.value))?;
                            //todo: not discrard other slots
                            &mut Some(String::new())
                        },
                        _ => {
                            other.push(StartElement { name, attributes, namespace });
                            &mut Some(String::new())
                        }
                    }; 
                    if string_opt.is_none() {
                        let _ = string_opt.insert(get_characters(&mut iter)?);
                    }
                }
                
                EndElement { name } => {
                    let opended_name = open_elements.pop().ok_or(Error::MissingOpeningTag(name.clone()))?; 
                    if opended_name != name {
                        return Err(Error::InvalidClosingTag(name));
                    }
                    if name.borrow() == ACCOUNT_XML_KEY {
                        break;
                    }
                }
                
                e => other.push(e)
            }
        }
        let commodity_scu = commodity_scu.as_deref().map(str::parse).transpose()?;
        commodity.update(|c| c.scu = commodity_scu);

        Ok(Some(Account { 
            name: acc_name.ok_or(Error::MissingElement("act:name"))?, 
            code, 
            id: id.map(Uuid::from_string).ok_or(Error::MissingElement("act:id"))??, 
            acc_type: acc_type.map(AccountType::from_string).ok_or(Error::MissingElement("act:type"))??, 
            commodity, 
            write_protected, 
            description, 
            parent: parent.map(Uuid::from_string).transpose()?,
            other 
        }))
    }
}