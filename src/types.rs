pub struct Config {
    pub root: Root,
    pub process: Process,
    pub hostname: Option<String>,
    pub namespaces: Vec<Namespace>,
    pub uid_mappings: Option<Vec<UgidMapping>>,
    pub gid_mappings: Option<Vec<UgidMapping>>,
    pub cgroups_path: Option<String>,
    pub resources: Resources,
}

pub struct Root {
    pub path: String,
}

pub struct Process {
    pub cwd: String,
    pub env: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub user: ProcessUser,
}

pub struct ProcessUser {
    pub uid: usize,
    pub gid: usize,
}

pub struct Namespace {
    pub r#type: NamespaceType,
    pub path: Option<String>,
}

pub enum NamespaceType {
    Pid,
    Network,
    Mount,
    Ipc,
    Uts,
    User,
    Cgroup,
}

pub struct UgidMapping {
    pub container_id: usize,
    pub host_id: usize,
    pub size: usize,
}

pub struct Resources {
    pub memory: ResourcesMemory,
    pub cpu: ResourcesCPU,
}

pub struct ResourcesMemory {
    pub limit: i64,
    pub reservation: i64,
    pub swap: i64,
}

pub struct ResourcesCPU {
    pub shares: u64,
    pub quota: i64,
    pub period: u64,
}
