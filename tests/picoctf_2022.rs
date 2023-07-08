use std::str::FromStr;

use rsacracker::{integer_to_string, run_attacks, Parameters};
use rug::Integer;

#[test]
fn picoctf_2022_sum_o_primes() {
    // From picoCTF 2022 / Sum-O-Primes
    // https://ctftime.org/task/19785

    let params = Parameters {
        n: Some(Integer::from_str("19594979821655183104856721200876279688744199212596318791410700925214020940999368124753876379046101491755637328180352524777390418669210709696131801135729820817964419360433309338628094186567119917735965630612618443318361476838501333469511238156792898753876667912964148133223421606462036819614830830346046983259407147596111671725522875516790826213635619398696281968888325882416381776971733880221909903860599888903194248107358128483103962534467323374352040906477803568664482713174891860915973023918444553550090873773281086421418960484839799410173913761912529789181262819973734812402783319741028408456027454148088452256679").unwrap()),
        sum_pq: Some(Integer::from_str("283448763112935396292672508494677342956387692709613190747337505254745038440112219196768813935476747610338768373312897468544362172877862811371116345292731982030202799279648808568611019652855145745874995725323535434244721756466664844450642633991884547297913224974132790120791402207195730139176850623910443635096").unwrap()),
        c: Some(Integer::from_str("18636447717166163110007308398125020106909243310909125985906865345153482499267369958513495573419378863382357479160311974668714164532528899272402180073813155145433400182510795244538100416833606047258427705348582210646875194338618258368402339207317746389594547067680540045952368498889488396986833144559958039693126438281109691155065365460131459707251556570006509693787178631970133834115551125099304598811737751381172581006892694680226799934890914643996419824656177589214394712830071259775111524779849375845055047510408226934137701774131311188919457501441536828353905914245941981372795258464522301244658509567566277162831").unwrap()),
        ..Default::default()
    };

    let solution = run_attacks(&params).unwrap();
    assert!(solution.pk.is_some());
    assert_eq!(
        integer_to_string(&solution.m.unwrap()).unwrap().trim(),
        "picoCTF{24929c45}"
    );
}

#[test]
fn picoctf_2022_very_smooth() {
    // From picoCTF 2022 / Very Smooth
    // https://ctftime.org/task/19827

    let params = Parameters {
        n: Some(Integer::from_str("24887720361085291721024326096731793930799398455971483203000645952258569431554549829965232039203319572285189312193415691801940975559717390968864595267721784991998174669984312621794975861592210322337219578190870420827008587133784860199116133710855097114985719038499134184753051647804801382767640985461932945654570736349990816093313683182981390666699529816262031851045047223491961572602660185616823932776628166086022004649842351708250335718864338439758985618255607848920428570594709761536677991211502014960637876339835504410007728881274218515227043173123235914446576604179022875919868801367329778453868515004646190019061").unwrap()),
        c: Some(Integer::from_str("3953381982462210642215015115798135784111499156802961488726071903266941596448756503427980418166697589759497427658920794766885164404802857085596477959469560407803466294790288990969263289113232250215878886687127388389357027459035126754251458589237262171770425949866854021803564999958458007480713046100430588400396867842280886880407283209706163211609020762384196310982185233793917611843032836059769905428359064482478635828132718292747330033266574246581946025734579939479641212378288638362196455188309978851457712119581584018298047419228419622390631184157610230589190026479454735263990411479168862280590771470719997491863").unwrap()),
        ..Default::default()
    };

    let solution = run_attacks(&params).unwrap();
    assert!(solution.pk.is_some());
    assert_eq!(
        integer_to_string(&solution.m.unwrap()).unwrap().trim(),
        "picoCTF{7c8625a1}"
    );
}
