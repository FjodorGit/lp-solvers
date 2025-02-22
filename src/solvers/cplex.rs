//! The IBM CPLEX optimizer.
//! You need to activate the "cplex" feature of this crate to use this solver.

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

use xml::reader::XmlEvent;
use xml::EventReader;

use crate::lp_format::LpProblem;
use crate::solvers::{Solution, SolverProgram, SolverWithSolutionParsing, Status, WithMipGap};
use crate::util::buf_contains;

/// IBM cplex optimizer
#[derive(Debug, Clone)]
pub struct Cplex {
    command: String,
    mipgap: Option<f32>,
}

impl Default for Cplex {
    fn default() -> Self {
        Self {
            command: "cplex".into(),
            mipgap: None,
        }
    }
}

impl Cplex {
    /// Create a cplex solver from the given binary
    pub fn with_command(command: String) -> Self {
        Self {
            command,
            mipgap: None,
        }
    }
}

impl WithMipGap<Cplex> for Cplex {
    fn mip_gap(&self) -> Option<f32> {
        self.mipgap
    }

    fn with_mip_gap(&self, mipgap: f32) -> Result<Cplex, String> {
        if mipgap.is_sign_positive() && mipgap.is_finite() {
            Ok(Cplex {
                mipgap: Some(mipgap),
                ..(*self).clone()
            })
        } else {
            Err("Invalid MIP gap: must be positive and finite".to_string())
        }
    }
}

macro_rules! format_osstr {
    ($($parts:expr)*) => {{
        let mut s = OsString::new();
        $(s.push($parts);)*
        s
    }}
}

impl SolverProgram for Cplex {
    fn command_name(&self) -> &str {
        &self.command
    }

    fn arguments(&self, lp_file: &Path, solution_file: &Path) -> Vec<OsString> {
        let mut args = vec!["-c".into(), format_osstr!("READ \"" lp_file "\"")];

        if let Some(mipgap) = self.mip_gap() {
            args.push(format_osstr!("set mip tolerances mipgap " mipgap.to_string()));
        }

        args.push("optimize".into());
        args.push(format_osstr!("WRITE \"" solution_file "\""));

        args
    }

    fn parse_stdout_status(&self, stdout: &[u8]) -> Option<Status> {
        if buf_contains(stdout, "No solution exists") {
            Some(Status::Infeasible)
        } else {
            None
        }
    }

    fn solution_suffix(&self) -> Option<&str> {
        Some(".sol")
    }
}

impl SolverWithSolutionParsing for Cplex {
    fn read_specific_solution<'a, P: LpProblem<'a>>(
        &self,
        f: &File,
        problem: Option<&'a P>,
    ) -> Result<Solution, String> {
        let len = problem.map(|p| p.variables().size_hint().0).unwrap_or(0);
        let parser = EventReader::new(f);
        let mut solution = Solution {
            status: Status::Optimal,
            results: HashMap::with_capacity(len),
        };
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == "variable" {
                        let mut name = None;
                        let mut value = None;
                        for attr in attributes {
                            match attr.name.local_name.as_str() {
                                "name" => name = Some(attr.value),
                                "value" => {
                                    let parsed = attr.value.parse().map_err(|e| {
                                        format!("invalid variable value for {:?}: {}", name, e)
                                    })?;
                                    value = Some(parsed)
                                }
                                _ => {}
                            };
                        }
                        if let (Some(name), Some(value)) = (name, value) {
                            solution.results.insert(name, value);
                        }
                    }
                }
                Err(e) => return Err(format!("xml error: {}", e)),
                _ => {}
            }
        }
        Ok(solution)
    }
}

#[cfg(test)]
mod tests {
    use crate::solvers::{Cplex, SolverProgram, WithMipGap};
    use std::ffi::OsString;
    use std::path::Path;

    #[test]
    fn cli_args_default() {
        let solver = Cplex::default();
        let args = solver.arguments(Path::new("test.lp"), Path::new("test.sol"));

        let expected: Vec<OsString> = vec![
            "-c".into(),
            "READ \"test.lp\"".into(),
            "optimize".into(),
            "WRITE \"test.sol\"".into(),
        ];

        assert_eq!(args, expected);
    }

    #[test]
    fn cli_args_mipgap() {
        let solver = Cplex::default()
            .with_mip_gap(0.5)
            .expect("mipgap should be valid");

        let args = solver.arguments(Path::new("test.lp"), Path::new("test.sol"));

        let expected: Vec<OsString> = vec![
            "-c".into(),
            "READ \"test.lp\"".into(),
            "set mip tolerances mipgap 0.5".into(),
            "optimize".into(),
            "WRITE \"test.sol\"".into(),
        ];

        assert_eq!(args, expected);
    }

    #[test]
    fn cli_args_mipgap_negative() {
        let solver = Cplex::default().with_mip_gap(-0.05);
        assert!(solver.is_err());
    }

    #[test]
    fn cli_args_mipgap_infinite() {
        let solver = Cplex::default().with_mip_gap(f32::INFINITY);
        assert!(solver.is_err());
    }
}
