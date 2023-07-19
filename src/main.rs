mod easycsv;
mod recent1;
mod recent2;
mod recent3;
mod recent8;
mod util;

use std::collections::HashSet;

use anyhow::Result;
use recent1::RecentRecord1;
use recent2::RecentRecord2;
use recent3::RecentRecord3;
use recent2::RecentTable2;
use util::capitals_from_table;
use crate::{easycsv::optionfmt, recent1::RecentTable1, recent8::RecentTable8, recent3::RecentTable3, util::compare_sets};
use crate::util::CapitalDeNotificacao;


fn main() -> Result<()> {

    let rt1 = RecentTable1::get()?;
    if false {
        dbg!(&rt1);
    }
    let c1 = capitals_from_table(&rt1)?;
        
    let rt2 = RecentTable2::get()?;
    if false {
        dbg!(&rt2);
    }
    let c2 = capitals_from_table(&rt2)?;

    let rt3 = RecentTable3::get()?;
    if false {
        dbg!(&rt3);
    }
    let c3 = capitals_from_table(&rt3)?;
    // dbg!(compare_sets(&c2, &c3));
    // dbg!(compare_sets(&c1, &c3));
    // dbg!(compare_sets(&c1, &c2));
    
    let rt8 = RecentTable8::get()?;
    if false {
        let records = &rt8.records;
        println!("{records:?}");

        for record in records {
            let cap = record.capital_de_notificacao()?;
            let ill = optionfmt(*record.illiterate);
            let ed = optionfmt(*record.incomplete_medium_education);
            println!("{cap} {ill} {ed}");
        }
    }
    let c8 = capitals_from_table(&rt8)?;

    assert_eq!(&c1, &c2);
    assert_eq!(&c1, &c3); assert_eq!(&c2, &c3);
    assert_eq!(&c1, &c8);

    Ok(())
}
