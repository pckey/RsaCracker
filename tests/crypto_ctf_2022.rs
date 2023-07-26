use std::str::FromStr;

use rsacracker::{integer_to_string, run_attacks, Parameters};
use rug::Integer;

#[test]
fn crypto_ctf_2022_cantilever_task_1() {
    // From Crypto CTF 2022 / Cantilever

    let params = Parameters {
        n : Some(Integer::from_str("7069789930583271525053215046247773438899869283661158227309691853515987055334306019600324056376312479212090202373516405860759222837585952590589336295698718699890424169542280710721069784487366121478569760563045886361884895363592898476736269784284754788133722060718026577238640218755539268465317292713320841554802703379684173485217045274942603346947299152498798736808975912326592689302969859834957202716983626393365387411319175917999258829839695189774082810459527737342402920881184864625678296442001837072332161966439361793009893108796934406114288057583563496587655548536011677451960307597573257032154009427010069578913").unwrap()),
        c : Some(Integer::from_str("488692928085934899944055554857568564903346089951134051486941368561567330884363274156339625953702601270565654444836193796061118053575538224794730472032345171432952984560662218697488844007827176184413713651118743456250147472678673801728916283759932987216388378211555067885210167894310696549664382751443669387953644382833924884208966436685137553434532738845959014828804809425096115758364136546390809453200055265653531950423111482644330073443545410319576097902472017235065047191133112557289289189187696092145850680765843608118584107829268136014912479701945735063525799796920293418182776436767911172221104640501952880057").unwrap()),
        ..Default::default()

    };

    let solution = run_attacks(&params).unwrap();
    assert!(solution.pk.is_some());
    assert_eq!(
        integer_to_string(&solution.m.unwrap()).unwrap().trim(),
        "CCTF{5L3Ek_4s_"
    );
}

// #[test]
// fn crypto_ctf_2022_cantilever_task_2() {
//     // From Crypto CTF 2022 / Cantilever

//     let params = Parameters {
//         n : Some(Integer::from_str("7069789930583271525053215046247773438899869283661158227309691853515987055334306019600324056376312479212090202373516405860759222837585952590589336295698718699890424169542280710721069784487366121478569760563045886361884895363592898476736269784284754788133722060718026577238640218755539268465317292713320841554802703379684173485217045274942603346947299152498798736808975912326592689302969859834957202716983626393365387411319175917999258829839695189774082810459527737342402920881184864625678296442001837072332161966439361793009893108796934406114288057583563496587655548536011677451960307597573257032154009427010069578913").unwrap()),
//         c : Some(Integer::from_str("109770827223661560471527567179288748906402603483328748683689436879660543465776899146036833470531024202351087008847594392666852763100570391337823820240726499421306887565697452868723849092658743267256316770223643723095601213088336064635680075206929620159782416078143076506249031972043819429093074684182845530529249907297736582589125917235222921623698038868900282049587768700860009877737045693722732170123306528145661683416808514556360429554775212088169626620488741903267154641722293484797745665402402381445609873333905772582972140944493849645600529147490903067975300304532955461710562911203871840101407995813072692212").unwrap()),
//         ..Default::default()

//     };

//     let solution = run_attacks(&params).unwrap();
//     assert!(solution.pk.is_some());
//     assert_eq!(
//         integer_to_string(&solution.m.unwrap()).unwrap().trim(),
//         "[SNIP]}"
//     );
// }
