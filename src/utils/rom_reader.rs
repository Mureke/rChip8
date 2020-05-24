
struct RomReader {
    file: str,
    data: Vec<str>
}

impl RomReader {

    fn new(rom: Box<str>) -> Self {
        RomReader {file, data}
    }
}