use std::collections::HashMap;

// Regiao
// https://en.wikipedia.org/wiki/Regions_of_Brazil

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegionName<'t>(pub &'t str);
#[derive(Debug)]
pub struct Region {
    pub name: &'static str,
    pub most_populous_municipality: CityName<'static>,
}

const REGIONS: &[Region] = &[
    Region {
        name: "Norte",
        most_populous_municipality: CityName("Manaus"),
    },
    Region {
        name: "Nordeste",
        most_populous_municipality: CityName("Salvador"),
    },
    Region {
        name: "Sudeste",
        most_populous_municipality: CityName("São Paulo"),
    },
    Region {
        name: "Sul",
        most_populous_municipality: CityName("Curitiba"),
    },
    Region {
        name: "Centro-oeste",
        most_populous_municipality: CityName("Brasília"),
    },
];

const NORTH: RegionName<'static> = RegionName("Norte");
const NORTHEAST: RegionName<'static> = RegionName("Nordeste");
const SOUTHEAST: RegionName<'static> = RegionName("Sudeste");
const SOUTH: RegionName<'static> = RegionName("Sul");
const CENTRAL_WEST: RegionName<'static> = RegionName("Centro-oeste");

// https://en.wikipedia.org/wiki/States_of_Brazil

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateName<'t>(pub &'t str);
#[derive(Debug)]
pub struct State {
    pub name: &'static str,
    pub capital: CityName<'static>,
    pub largest_city: Option<CityName<'static>>,
    pub region: RegionName<'static>,
}

const STATES: &[State] = &[
    State {
        name: "Acre",
        capital: CityName("Rio Branco"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Alagoas",
        capital: CityName("Maceió"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Amapá",
        capital: CityName("Macapá"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Amazonas",
        capital: CityName("Manaus"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Bahia",
        capital: CityName("Salvador"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Ceará",
        capital: CityName("Fortaleza"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Distrito Federal",
        capital: CityName("Brasília"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Espírito Santo",
        capital: CityName("Vitória"),
        largest_city: Some(CityName("Serra")),
        region: SOUTHEAST,
    },
    State {
        name: "Goiás",
        capital: CityName("Goiânia"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Maranhão",
        capital: CityName("São Luís"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Mato Grosso",
        capital: CityName("Cuiabá"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Mato Grosso do Sul",
        capital: CityName("Campo Grande"),
        largest_city: None,
        region: CENTRAL_WEST,
    },
    State {
        name: "Minas Gerais",
        capital: CityName("Belo Horizonte"),
        largest_city: None,
        region: SOUTHEAST,
    },
    State {
        name: "Pará",
        capital: CityName("Belém"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Paraíba",
        capital: CityName("João Pessoa"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Paraná",
        capital: CityName("Curitiba"),
        largest_city: None,
        region: SOUTH,
    },
    State {
        name: "Pernambuco",
        capital: CityName("Recife"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Piauí",
        capital: CityName("Teresina"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Rio de Janeiro",
        capital: CityName("Rio de Janeiro"),
        largest_city: None,
        region: SOUTHEAST,
    },
    State {
        name: "Rio Grande do Norte",
        capital: CityName("Natal"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Rio Grande do Sul",
        capital: CityName("Porto Alegre"),
        largest_city: None,
        region: SOUTH,
    },
    State {
        name: "Rondônia",
        capital: CityName("Porto Velho"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Roraima",
        capital: CityName("Boa Vista"),
        largest_city: None,
        region: NORTH,
    },
    State {
        name: "Santa Catarina",
        capital: CityName("Florianópolis"),
        largest_city: Some(CityName("Joinville")),
        region: SOUTH,
    },
    State {
        name: "São Paulo",
        capital: CityName("São Paulo"),
        largest_city: None,
        region: SOUTHEAST,
    },
    State {
        name: "Sergipe",
        capital: CityName("Aracaju"),
        largest_city: None,
        region: NORTHEAST,
    },
    State {
        name: "Tocantins",
        capital: CityName("Palmas"),
        largest_city: None,
        region: NORTH,
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CityName<'t>(pub &'t str);
#[derive(Debug)]
pub struct City {
    pub name: &'static str,
    pub region: RegionName<'static>,
    pub is_notification_capital: bool,
}

const CITIES: &[City] = &[
    City {
        name: "São Paulo",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Rio Branco",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Recife",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Curitiba",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Belém",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Salvador",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Rio de Janeiro",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Rio de Janeiro",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Maceió",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Fortaleza",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Manaus",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Macapá",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Brasília",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Florianópolis",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Aracaju",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Vitória",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Belo Horizonte",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "São Luís",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "João Pessoa",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Natal",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Porto Velho",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Porto Alegre",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Campo Grande",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Goiânia",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Cuiabá",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Teresina",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Palmas",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
    City {
        name: "Boa Vista",
        region: RegionName("XX"),
        is_notification_capital: true,
    },
];

pub struct StaticDatabase {
    regions: HashMap<RegionName<'static>, &'static Region>,
    cities: HashMap<CityName<'static>, &'static City>,
    states: HashMap<StateName<'static>, &'static State>,
    states_by_capital: HashMap<CityName<'static>, &'static State>,
}

impl StaticDatabase {
    pub fn get() -> StaticDatabase {
        StaticDatabase {
            // XX: detect duplicates
            regions: REGIONS.iter().map(|v| (RegionName(v.name), v)).collect(),
            cities: CITIES.iter().map(|v| (CityName(v.name), v)).collect(),
            states: STATES.iter().map(|v| (StateName(v.name), v)).collect(),
            states_by_capital: STATES.iter().map(|v| (v.capital.clone(), v)).collect(),
        }
    }
    #[allow(unused)]
    pub fn get_region(&self, key: RegionName) -> Option<&Region> {
        self.regions.get(&key).map(|v| *v)
    }
    pub fn get_city(&self, key: CityName) -> Option<&City> {
        self.cities.get(&key).map(|v| *v)
    }
    #[allow(unused)]
    pub fn get_state(&self, key: StateName) -> Option<&State> {
        self.states.get(&key).map(|v| *v)
    }
    pub fn get_state_by_capital(&self, key: CityName) -> Option<&State> {
        self.states_by_capital.get(&key).map(|v| *v)
    }
    pub fn city_opt_capital_of_state(&self, city: &City) -> Option<&State> {
        self.get_state_by_capital(CityName(city.name))
    }
}
