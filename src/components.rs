
use crate::utils::{disk_available, get_time, is_interface_up, run_command, run_shell};


const HIGHLIGHT: &str = "^c#63be34^";
const WARN: &str = "^c#e5484d^";
const NORMAL: &str = "^c#bbbbbb^";


pub trait Component {
    fn render(&self) -> String;
}

pub struct ComponentRunner {
    component: Box<dyn Component>,
    pub interval: usize,
}

impl ComponentRunner {
    pub fn new(component: Box<dyn Component>, interval: usize) -> Self {
        Self { component, interval }
    }

    pub fn update(&self) -> String {
        self.component.render()
    }
}

pub struct DiskComponent {
    pub mount: &'static[&'static str],
}
impl Component for DiskComponent {
    fn render(&self) -> String {
        let mut render_str = String::from("Disk: ");
        if self.mount.len() > 1 {
            render_str.push_str("[ ");
        }

        for (n, path) in self.mount.iter().enumerate() {
            let res = disk_available(path);
            if res.is_none() {
                render_str.push_str(&format!("{WARN}unknown{NORMAL}"));
            } else {
                let remain = res.unwrap();
                if remain <= 20_f64 {
                    render_str.push_str(&format!("{WARN}{remain:.1}{NORMAL} GB"));
                } else {
                    render_str.push_str(&format!("{HIGHLIGHT}{remain:.1}{NORMAL} GB"));
                }
            }

            if n != self.mount.len()-1 {
                render_str.push_str(", ");
            }
        }

        if self.mount.len() > 1 {
            render_str.push_str(" ]");
        }

        render_str
    }
}

pub struct TimeComponent;
impl Component for TimeComponent {
    fn render(&self) -> String {
        get_time()
    }
}

pub struct ProxyComponent;
impl Component for ProxyComponent {
    fn render(&self) -> String {
        let res = run_command("proxy_status.sh", &[]);
        if res.is_none() {
            return format!("Proxy: {WARN}unknown{NORMAL}");
        }
        let port_or_status = res.unwrap();
        if port_or_status == "OFF" {
            return format!("Proxy: {WARN}OFF{NORMAL}");
        }
        return format!("Proxy: {HIGHLIGHT}{port_or_status}{NORMAL}");
    }
}

pub struct VolComponent;
impl Component for VolComponent {
    fn render(&self) -> String {
        let script = "pactl get-sink-mute @DEFAULT_SINK@ | grep -q '是' && echo MUTED || pactl get-sink-volume @DEFAULT_SINK@ | awk '{print $5}'";
        if let Some(vol) = run_shell(script) {
            let mut color = HIGHLIGHT;
            if vol == "MUTED" {
                color = WARN;
            }
            return format!("Vol: {color}{vol}{NORMAL}");
        }
        return format!("{WARN}unknown{NORMAL}");
    }
}

pub struct NetComponent {
    pub interface: &'static str
}
impl Component for NetComponent {
    fn render(&self) -> String {
        if is_interface_up(self.interface) {
            return format!("Net: {HIGHLIGHT}ON({}){NORMAL}", self.interface);
        }
        return format!("Net: {WARN}OFF({}){NORMAL}", self.interface);
    }
}


