#[allow(non_camel_case_types)]
#[allow(unused)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Direction {
    uttar,   // north
    purva,   // east
    paschim, // west
    dakshin, // south

    vayavya,  // North-West
    ishanya,  // North-East
    nairutya, // South-West
    agneya,   // South-East

    urdhwa,     // Akash (skywards)
    adharastha, // Patal (downwards)
}
