pub enum Block{
    line,
    square,
    z_style,
    s_style,
    l_style,
    linv_style,
    k_style,
}

/*
pub fn match_definition(block: Block) -> ?Fuck? {
    match block {
        line => [true, true, true, true],
        square => [[true, true],[true, true]],
        z_style => [[true, true, false], [false, true, true]],
        s_style => [[false, true, true], [true, true, false]],
        l_style => [[true, false, false], [true, true, true]],
        linv => [[false, false, true], [true, true, true]],
        k_style => [[false, true, false], [true, true, true]]
    }
}*/