mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("ken");
    acmd::install(agent);
    agent.install();
}
