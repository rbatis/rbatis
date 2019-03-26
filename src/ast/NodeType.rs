pub enum NodeType {
    NArg(String),
    NString(String),
    NIf(String),
    NTrim(String),
    NForEach(String),
    NChoose(String),
    NOtherwise(String),
    NWhen(String),
    NBind(String),
    NInclude(String),
}

