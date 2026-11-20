use arbitrait::{
    ballot::unscored::MandateForbidBallot,
    proposal::Propose,
    report::StandardFormat,
    vote::Outcome,
};
use std::fmt;

// !- Plan structs

#[expect(clippy::struct_excessive_bools, reason = "bools are the correct choice here")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct ConfigPlan {
    pub copy: bool,
    pub default: bool,

    pub infallible_merge: bool,
    pub fallible_merge: bool,

    pub infallible_from_override: bool,
    pub fallible_from_override: bool,
}
impl From<&ConfigConsensus> for ConfigPlan {
    fn from(consensus: &ConfigConsensus) -> Self {
        Self {
            copy: consensus.copy.as_bool(),
            default: consensus.default.as_bool(),
            infallible_merge: consensus.infallible_merge.as_bool(),
            fallible_merge: consensus.fallible_merge.as_bool(),
            infallible_from_override: consensus.infallible_from_override.as_bool(),
            fallible_from_override: consensus.fallible_from_override.as_bool(),
        }
    }
}

#[expect(clippy::struct_excessive_bools, reason = "bools are the correct choice here")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct BuilderPlan {
    pub copy: bool,
    pub default: bool,

    pub infallible_build: bool,
    pub fallible_build: bool,
}
impl From<&BuilderConsensus> for BuilderPlan {
    fn from(consensus: &BuilderConsensus) -> Self {
        Self {
            copy: consensus.copy.as_bool(),
            default: consensus.default.as_bool(),
            infallible_build: consensus.infallible_build.as_bool(),
            fallible_build: consensus.fallible_build.as_bool(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct OverridePlan {
    pub copy: bool,
    pub default: bool,
}
impl From<&OverrideConsensus> for OverridePlan {
    fn from(consensus: &OverrideConsensus) -> Self {
        Self {
            copy: consensus.copy.as_bool(),
            default: consensus.default.as_bool(),
        }
    }
}

// !- Evidence

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Topic {
    Type,
    Attr,
    Proposal,
    Struct,
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "Type support"),
            Self::Attr => write!(f, "Attribute"),
            Self::Proposal => write!(f, "Proposal"),
            Self::Struct => write!(f, "Struct"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Cause {
    AttrPresent(&'static str),
    AttrMissing(&'static str),
    HasType(&'static str),
    NotType(&'static str),
    Forbid(&'static str),
    NoFields,
}
impl Cause {
    pub(crate) fn from_flag_attr(present: bool, attr: impl Into<&'static str>) -> Self {
        if present {
            Self::AttrPresent(attr.into())
        } else {
            Self::AttrMissing(attr.into())
        }
    }
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AttrPresent(attr) => write!(f, "'{attr}' present"),
            Self::AttrMissing(attr) => write!(f, "'{attr}' missing"),
            Self::HasType(ty) => write!(f, "Has '{ty}' type"),
            Self::NotType(ty) => write!(f, "Type is not '{ty}'"),
            Self::Forbid(name) => write!(f, "'{name}' forbidden"),
            Self::NoFields => write!(f, "Has no fields"),
        }
    }
}

pub(crate) type Evidence = arbitrait::evidence::Evidence<Topic, Cause>;

// !- Consensus structs

pub(crate) struct Consensus {
    pub config: ConfigConsensus,
    pub builder: BuilderConsensus,
    pub override_: OverrideConsensus,
    //pub error_consensus: Option<ErrorConsensus>
}
impl Consensus {
    // pub(crate) fn as_plan(&self) -> Plan {
    //     self.into()
    // }
    pub(crate) fn as_rendered_report(&self) -> String {
        [
            self.config.as_rendered_report(),
            self.builder.as_rendered_report(),
            self.override_.as_rendered_report(),
        ].join("\n")
    }
}

pub(crate) struct ConfigConsensus {
    pub copy: Outcome<DeriveCopy>,
    pub default: Outcome<DeriveDefault>,

    pub infallible_merge: Outcome<InfallibleMerge>,
    pub fallible_merge: Outcome<FallibleMerge>,

    pub infallible_from_override: Outcome<InfallibleFromOverride>,
    pub fallible_from_override: Outcome<FallibleFromOverride>,
}
impl ConfigConsensus {
    pub(crate) fn as_plan(&self) -> ConfigPlan {
        self.into()
    }
    pub(crate) fn as_rendered_report(&self) -> String {
        let fmt = StandardFormat;
        [
            "\n-- Config struct --\n".to_string(),
            self.copy.as_rendered_report(fmt),
            self.default.as_rendered_report(fmt),
            self.infallible_merge.as_rendered_report(fmt),
            self.fallible_merge.as_rendered_report(fmt),
            self.infallible_from_override.as_rendered_report(fmt),
            self.fallible_from_override.as_rendered_report(fmt),
        ].join("\n")
    }
}

pub(crate) struct BuilderConsensus {
    pub copy: Outcome<DeriveCopy>,
    pub default: Outcome<DeriveDefault>,

    pub infallible_build: Outcome<InfallibleBuild>,
    pub fallible_build: Outcome<FallibleBuild>,
}
impl BuilderConsensus {
    pub(crate) fn as_plan(&self) -> BuilderPlan {
        self.into()
    }
    pub(crate) fn as_rendered_report(&self) -> String {
        let fmt = StandardFormat;
        [
            "\n-- Builder struct --\n".to_string(),
            self.copy.as_rendered_report(fmt),
            self.default.as_rendered_report(fmt),
            self.infallible_build.as_rendered_report(fmt),
            self.fallible_build.as_rendered_report(fmt),
        ].join("\n")
    }
}

pub(crate) struct OverrideConsensus {
    pub copy: Outcome<DeriveCopy>,
    pub default: Outcome<DeriveDefault>,
}
impl OverrideConsensus {
    pub(crate) fn as_plan(&self) -> OverridePlan {
        self.into()
    }
    pub(crate) fn as_rendered_report(&self) -> String {
        let fmt = StandardFormat;
        [
            "\n-- Override struct --\n".to_string(),
            self.copy.as_rendered_report(fmt),
            self.default.as_rendered_report(fmt),
        ].join("\n")
    }
}

// !- Proposal structs

#[derive(Debug, Copy, Clone)]
pub(crate) struct DeriveCopy;
impl Propose for DeriveCopy {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Derive Copy".to_string() }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct DeriveDefault;
impl Propose for DeriveDefault {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Derive Default".to_string() }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct InfallibleBuild;
impl Propose for InfallibleBuild {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Implement build()".to_string() }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FallibleBuild;
impl Propose for FallibleBuild {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Implement try_build()".to_string() }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct InfallibleMerge;
impl Propose for InfallibleMerge {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Implement merge_with_override()".to_string() }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FallibleMerge;
impl Propose for FallibleMerge {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Implement try_merge_with_override()".to_string() }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct InfallibleFromOverride;
impl Propose for InfallibleFromOverride {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Implement From<Override>".to_string() }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FallibleFromOverride;
impl Propose for FallibleFromOverride {
    type CountedBallot = MandateForbidBallot;
    type Evidence = Evidence;
    fn description() -> String { "Implement TryFrom<Override>".to_string() }
}
