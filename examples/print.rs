use jcl::step::dd::in_stream::{Delimiter, InStream};
use jcl::step::dd::{DdGroup, DdStatement};
use jcl::step::Step;
use jcl::Job;

fn main() {
    let mut job = Job::new("MYJOB1");

    let mut step = Step::new("MYSTEP", "MYPROGRAM");

    let mut sysout_group = DdGroup::new("SYSIN");

    let mut in_stream = InStream::new("HERE IS SOME TEXT".to_string());
    in_stream.delimiter(Delimiter::new("@@").unwrap());

    sysout_group.add_statement(DdStatement::InStream(in_stream));

    step.add_dd_group(sysout_group);

    job.add_step(step);

    print!("{}", job);
}
