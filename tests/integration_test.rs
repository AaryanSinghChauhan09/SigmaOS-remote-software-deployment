//! Integration tests for SigmaOS-remote-software-deployment.

use sigmaos_remote_software_deployment_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
