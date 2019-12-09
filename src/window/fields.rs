
/// Valid choices for the window subcommand fields option.
/// Some options are short hand for multiple options.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum ShorthandField {
    PercGC,
    CRI,
    Margolin1,
    Margolin2,
    Di,
    Tri,
    DiNR,
    TriNR,
    All,
}


impl ShorthandField {

    fn domain() -> Vec<Self> {
        use ShorthandField::*;
        vec![PercGC, CRI, Margolin1, Margolin2, Di, Tri, DiNR, TriNR, All]
    }

    fn as_fields(&self) -> Vec<> {
        window::Field
    }

}


impl Display for ShorthandField {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ShorthandField::*;

        let s = match self {
            PercGC => "perc_gc",
            CRI => "cri",
            Margolin1 => "margolin1",
            Margolin2 => "margolin2",
            Di => "di",
            Tri => "tri",
            DiNR => "di_nr",
            TriNR => "tri_nr",
            All => "all",
        };
        write!(f, "{}", s)
    }

}


impl std::str::FromStr for ShorthandField {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "perc_gc" => Ok(PercGC),
            "cri" => Ok(CRI),
            "margolin1" => Ok(Margolin1),
            "margolin2" => Ok(Margolin2),
            "di" => Ok(Di),
            "tri" => Ok(Tri),
            "di_nr" => Ok(DiNR),
            "tri_nr" => Ok(TriNR),
            "all" => Ok(All),
            m => Err(
                Error::ChoiceError {
                    bad_choice: m.to_string(),
                    valid_choices: WindowField::domain()
                }
            ),
        }
    }
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Field {
    PercGC,
    CRI,
    Margolin1,
    Margolin2,
    AA,
    AT,
    AG,
    AC,
    TA,
    TT,
    TG,
    TC,
    GA,
    GT,
    GG,
    GC,
    CA,
    CT,
    CG,
    CC,
    AAA,
    AAT,
    AAG,
    AAC,
    ATA,
    ATT,
    ATG,
    ATC,
    AGA,
    AGT,
    AGG,
    AGC,
    ACA,
    ACT,
    ACG,
    ACC,
    TAA,
    TAT,
    TAG,
    TAC,
    TTA,
    TTT,
    TTG,
    TTC,
    TGA,
    TGT,
    TGG,
    TGC,
    TCA,
    TCT,
    TCG,
    TCC,
    GAA,
    GAT,
    GAG,
    GAC,
    GTA,
    GTT,
    GTG,
    GTC,
    GGA,
    GGT,
    GGG,
    GGC,
    GCA,
    GCT,
    GCG,
    GCC,
    CAA,
    CAT,
    CAG,
    CAC,
    CTA,
    CTT,
    CTG,
    CTC,
    CGA,
    CGT,
    CGG,
    CGC,
    CCA,
    CCT,
    CCG,
    CCC,
}


impl Field {

    fn get_di() -> Vec<Self> {
        use Field::*;
        vec![
            AA,
            AT,
            AG,
            AC,
            TA,
            TT,
            TG,
            TC,
            GA,
            GT,
            GG,
            GC,
            CA,
            CT,
            CG,
            CC,
        ]
    }

    fn smaller_di(&self) -> Self {
        match self {
            AA => AA,
            AT => AT,
            AG => AG,
            AC => AC,
            TA => TA,
            TT => AA,
            TG => CA,
            TC => GA,
            GA => GA,
            GT => AC,
            GG => CC,
            GC => GC,
            CA => CA,
            CT => AG,
            CG => CG,
            CC => CC,
        }
    }

    fn smaller_tri(&self) -> Self {
        match self {
            AAA => AAA,
            AAT => AAT,
            AAG => AAG,
            AAC => AAC,
            ATA => ATA,
            ATT => AAT,
            ATG => ATG,
            ATC => ATC,
            AGA => AGA,
            AGT => ACT,
            AGG => AGG,
            AGC => AGC,
            ACA => ACA,
            ACT => ACT,
            ACG => ACG,
            ACC => ACC,
            TAA => TAA,
            TAT => ATA,
            TAG => CTA,
            TAC => GTA,
            TTA => TAA,
            TTT => AAA,
            TTG => CAA,
            TTC => GAA,
            TGA => TCA,
            TGT => ACA,
            TGG => CCA,
            TGC => GCA,
            TCA => TCA,
            TCT => AGA,
            TCG => CGA,
            TCC => GGA,
            GAA => GAA,
            GAT => ATC,
            GAG => CTC,
            GAC => GAC,
            GTA => GTA,
            GTT => AAC,
            GTG => CAC,
            GTC => GAC,
            GGA => GGA,
            GGT => ACC,
            GGG => CCC,
            GGC => GCC,
            GCA => GCA,
            GCT => AGC,
            GCG => CGC,
            GCC => GCC,
            CAA => CAA,
            CAT => ATG,
            CAG => CAG,
            CAC => CAC,
            CTA => CTA,
            CTT => AAG,
            CTG => CAG,
            CTC => CTC,
            CGA => CGA,
            CGT => ACG,
            CGG => CCG,
            CGC => CGC,
            CCA => CCA,
            CCT => AGG,
            CCG => CCG,
            CCC => CCC,
        }
    }
}
