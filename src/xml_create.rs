// use std::fmt::format;

// use chrono::Local;
// use uuid::Uuid;
// use xml::{name::Name, namespace::Namespace, reader::XmlEvent, attribute::Attribute};

// use crate::read::Transaction;




// pub fn create_transaction_xml(transaction:Transaction) -> Box<[XmlEvent]> {
//     let mut events = Vec::new();
//     //<gnc:transaction version="2.0.0">
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("transaction", "gnc").to_owned(), 
//         attributes: vec![Attribute::new(Name::local("version"), "2.0.0").to_owned()], 
//         namespace: Namespace::empty() 
//     });
//     //<trn:id type="guid">
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("id", "trn").to_owned(), 
//         attributes: vec![Attribute::new(Name::local("type"), "guid").to_owned()], 
//         namespace: Namespace::empty() 
//     });
//     //b0a6a72623234002b0394d30cc807241
//     events.push(XmlEvent::Characters(Uuid::new_v4().to_string()));
//     //</trn:id>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("id", "trn").to_owned() });
//     //<trn:currency>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("currency", "trn").to_owned(),
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //<cmdty:space>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("space", "cmdty").to_owned(), 
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //CURRENCY
//     events.push(XmlEvent::Characters("CURRENCY".to_owned()));
//     //</xmdty:space>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("space", "cmdty").to_owned() });
//     //<cmdty:id>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("id", "cmdty").to_owned(), 
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //EUR
//     events.push(XmlEvent::Characters("EUR".to_owned()));
//     //</cmdty:id>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("id", "cmdty").to_owned() });
//     //</trn:currency>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("currency", "trn").to_owned() });
//     //<trn:date-posted>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("date-posted", "trn").to_owned(), 
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //<ts:date>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("date", "ts").to_owned(), 
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //2024-07-27 10:59:00 +0000
//     events.push(XmlEvent::Characters(Local::now().to_string()));
//     //</ts:date>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("date", "ts").to_owned() });
//     //</trn:date-posted>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("date-posted", "trn").to_owned() });
//     //<trn:date-entered>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("date-entered", "trn").to_owned(), 
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //<ts:date>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("date", "ts").to_owned(), 
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //2024-07-27 10:59:00 +0000
//     events.push(XmlEvent::Characters(Local::now().to_string()));
//     //</ts:date>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("date", "ts").to_owned() });
//     //</trn:date-entered>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("date-entered", "trn").to_owned() });
//     //<trn:description>
//     events.push(XmlEvent::StartElement { 
//         name: Name::prefixed("description", "trn").to_owned(), 
//         attributes: vec![], 
//         namespace: Namespace::empty() 
//     });
//     //Anfangsbestand
//     events.push(XmlEvent::Characters("Anfangsbestand".to_owned()));
//     //</trn:description>
//     events.push(XmlEvent::EndElement { name: Name::prefixed("description", "trn").to_owned() });

//     events.into_boxed_slice()
// }

// pub fn create_transaction_xml(transaction:Transaction) -> String {
//     let mut splits = String::new();
//     for (uuid, (value, quantity)) in transaction.accounts {
//         splits.push_str(format!(XML_SPLIT_TEMPLATE, ).as_str());
//     }
//     format!(XML_TRANSACTION_TEMPLATE)
// }

// const XML_TRANSACTION_TEMPLATE:&str = r#"
// <gnc:transaction version="2.0.0">
//   <trn:id type="guid">{}</trn:id>
//   <trn:currency>
//     <cmdty:space>CURRENCY</cmdty:space>
//     <cmdty:id>EUR</cmdty:id>
//   </trn:currency>
//   <trn:date-posted>
//     <ts:date>{}</ts:date>
//   </trn:date-posted>
//   <trn:date-entered>
//     <ts:date>{}</ts:date>
//   </trn:date-entered>
//   <trn:description>Anfangsbestand</trn:description>
//   <trn:slots>
//     <slot>
//       <slot:key>date-posted</slot:key>
//       <slot:value type="gdate">
//         <gdate>{}</gdate>
//       </slot:value>
//     </slot>
//   </trn:slots>
//   <trn:splits>
//     {}
//   </trn:splits>
// </gnc:transaction>
// "#;

// const XML_SPLIT_TEMPLATE:&str = r#"
//     <trn:split>
//       <split:id type="guid">{}</split:id>
//       <split:reconciled-state>n</split:reconciled-state>
//       <split:value>{}</split:value>
//       <split:quantity>{}</split:quantity>
//       <split:account type="guid">{}</split:account>
//     </trn:split>
// "#;