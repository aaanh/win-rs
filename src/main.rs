use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() {
    let mut s =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    for _ in 1..100 {
        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        // Refresh CPUs again.
        s.refresh_cpu();

        let mut cpu_sum: f32 = 0.0;

        for cpu in s.cpus() {
            cpu_sum += cpu.cpu_usage();
        }

        let cpu_avg = cpu_sum / (s.cpus().len() as f32);

        let mut bars = String::from("");

        for _ in 0..(cpu_avg.ceil() as i64) {
            bars.push('#');
        }

        print!("CPU usage: {:.2}% [{: <100}]\r", cpu_avg, bars);
    }
}
