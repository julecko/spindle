use crate::models::cd::Cd;

/// The I/O boundary: anything that would touch disk/network lives in
/// `data/`, never inline in `screens/` or `widgets/`. This stub just seeds
/// sample rows — swap the body for a real file/JSON load later without
/// touching any other directory.
pub fn load_cds() -> Vec<Cd> {
    vec![
        Cd {
            skupina: "Iron Maiden".into(),
            nazov_albumu: "Powerslave".into(),
        },
        Cd {
            skupina: "Kabát".into(),
            nazov_albumu: "Vlny".into(),
        },
    ]
}
