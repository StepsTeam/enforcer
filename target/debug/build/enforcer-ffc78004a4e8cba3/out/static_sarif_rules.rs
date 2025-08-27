use phf::Map;

pub static SARIF_RULES: Map<&'static str, &'static str> = ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("ENF001", "{\"fullDescription\":{\"text\":\"This rule serves as a placeholder for testing the SARIF rule generation in the build script.\"},\"helpUri\":\"https://enforcer.example.com/rules/ENF001\",\"id\":\"ENF001\",\"name\":\"ExampleRule\",\"shortDescription\":{\"text\":\"This is an example SARIF rule.\"}}"),
    ],
};