/// One entry in the CD library — plain data, no egui types. This is the
/// shape that flows: data (loads it) -> state (holds it) -> screens (reads
/// it) -> widgets (renders it).
pub struct Cd {
    pub skupina: String,      // band
    pub nazov_albumu: String, // title
}
