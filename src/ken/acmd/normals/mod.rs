mod attacks3sw;
mod attacks4;
mod attack11w;
mod attack12;
mod attackdash;

pub fn install(agent: &mut smashline::Agent) {
    attacks3sw::install(agent);
    attacks4::install(agent);
    attack11w::install(agent);
    attack12::install(agent);
    attackdash::install(agent);
}
