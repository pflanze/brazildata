use anyhow::{Result, bail};
use serde::Deserialize;

use crate::{easycsv::{CsvMonth, self}, util::RecentTable};

const SOURCE : &str = "tab 1 of 'CASOS CONFIRMADOS CAPITAL REGIAO E PLUVIOSIDADE.xlsx'";

#[derive(Debug, Deserialize)]
pub struct CasosPorRegiaoRecord {
    pub month: CsvMonth,
    pub regiao_norte: u64,
    pub regiao_nordeste: u64,
    pub regiao_sudeste: u64,
    pub regiao_sul: u64,
    pub regiao_centro_oeste: u64,
    pub total: u64,
}

impl CasosPorRegiaoRecord {
    fn check(&self) -> Result<()> {
        let total = self.regiao_norte + self.regiao_nordeste
            + self.regiao_sudeste + self.regiao_sul + self.regiao_centro_oeste;
        if total != self.total {
            bail!("total != self.total: {total} != {}", self.total)
        }
        Ok(())
    }
    pub fn get_column(&self, col: usize) -> u64 {
        match col {
            1 => self.regiao_norte,
            2 => self.regiao_nordeste,
            3 => self.regiao_sudeste,
            4 => self.regiao_sul,
            5 => self.regiao_centro_oeste,
            6 => self.total,
            _ => panic!("invalid col number {col}")
        }
    }
}

// CASOS CONFIRMADOS POR REGIAO DE NOTIFICAÇAO de 2005 a 2014
const TSV: &'static str = "	REGIAO NORTE	REGIAO NORDESTE	REGIAO SUDESTE	REGIAO SUL	REGIAO CENTRO OESTE	TOTAL
Janeiro	504	225	2,418	1,657	59	4,863
Fevereiro	681	266	1,856	1,691	65	4,559
Marco	1,058	377	1,629	1,499	70	4,633
Abril	685	645	1,272	893	57	3,552
Dezembro	298	205	1,245	1,221	40	3,009
Maio	593	1,084	788	568	44	3,077
Junho	426	1,051	623	501	32	2,633
Julho	337	898	547	498	21	2,301
Novembro	252	213	1,154	736	27	2,382
Outubro	248	223	695	621	25	1,812
Agosto	293	539	501	465	26	1,824
Setembro	249	325	552	527	23	1,676
";
const TOTAL: &[Option<u64>] = &[None, Some(5624), Some(6051), Some(13280), Some(10877), Some(489), Some(36321)];

#[derive(Debug)]
pub struct CasosPorRegiaoTable {
    name: String,
    records: Vec<CasosPorRegiaoRecord>,
}
    
impl CasosPorRegiaoTable {
    fn check(&self) -> Result<()> {
        let mut totals = [None, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0)];
        for record in &self.records {
            record.check()?;
            for i in 1..7 {
                *(totals[i].as_mut().unwrap()) += record.get_column(i);
            }
        }
        if totals != TOTAL {
            bail!("totals do not match: got {totals:?}, expected {TOTAL:?}")
        } 
        Ok(())
    }
    
    pub fn get() -> Result<CasosPorRegiaoTable> {
        let records = easycsv::parse_tsv::<CasosPorRegiaoRecord>(TSV.as_bytes())?;
        let tabl = CasosPorRegiaoTable {
            name: String::from(file!()) + " (" + SOURCE + ")",
            records
        };
        tabl.check()?;
        Ok(tabl)
    }
}

impl RecentTable<CasosPorRegiaoRecord> for CasosPorRegiaoTable {
    fn name(&self) -> &str { &self.name }
    fn records(&self) -> &Vec<CasosPorRegiaoRecord> { &self.records }
}
