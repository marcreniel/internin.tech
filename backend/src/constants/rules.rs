pub struct JobRule {
    pub site: &'static str,
    pub patterns: &'static [&'static str], 
}

pub const UUID_PATTERN: &str = r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b";

pub const GREENHOUSE_PATTERNS: &[&str] = &["/jobs/", "/job_app"];
pub const LEVER_PATTERNS: &[&str] = &[UUID_PATTERN];
pub const ASHBY_PATTERNS: &[&str] = &[UUID_PATTERN];
pub const PAYLOCITY_PATTERNS: &[&str] = &["/jobs/", "/Details/"];
pub const WORKABLE_PATTERNS: &[&str] = &["/view/", "/j/"];
pub const ICIMS_PATTERNS: &[&str] = &["/jobs/", "/job"];
pub const MYWORKDAYJOBS_PATTERNS: &[&str] = &["/job/"];
pub const JOBVITE_PATTERNS: &[&str] = &["/job/"];
pub const BREEZY_PATTERNS: &[&str] = &["/p/"];
pub const SMARTRECRUITERS_PATTERNS: &[&str] = &[];

pub const JOB_RULES: &[JobRule] = &[
    JobRule {
        site: "greenhouse.io",
        patterns: GREENHOUSE_PATTERNS,
    },
    JobRule {
        site: "lever.co",
        patterns: LEVER_PATTERNS,
    },
    JobRule {
        site: "ashbyhq.com",
        patterns: ASHBY_PATTERNS,
    },
    JobRule {
        site: "paylocity.com",
        patterns: PAYLOCITY_PATTERNS,
    },
    JobRule {
        site: "workable.com",
        patterns: WORKABLE_PATTERNS,
    },
    JobRule {
        site: "icims.com",
        patterns: ICIMS_PATTERNS,
    },
    JobRule {
        site: "myworkdayjobs.com",
        patterns: MYWORKDAYJOBS_PATTERNS,
    },
    JobRule {
        site: "jobvite.com",
        patterns: JOBVITE_PATTERNS,
    },
    JobRule {
        site: "breezy.hr",
        patterns: BREEZY_PATTERNS,
    },
    JobRule {
        site: "jobs.smartrecruiters.com",
        patterns: SMARTRECRUITERS_PATTERNS,
    },
];