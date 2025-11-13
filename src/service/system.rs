use std::sync::{Arc, LazyLock, Mutex};

use sysinfo::{Disks, System};

use crate::error::AppError;
use crate::model::bo::system::{InfoBo, InfoDatabaseBo, InfoDiskBo, InfoSystemBo};
use crate::state::AppState;

static SYSTEM_INFO: LazyLock<Mutex<System>> = LazyLock::new(|| Mutex::new(System::new_all()));
static DISKS_INFO: LazyLock<Mutex<Disks>> =
    LazyLock::new(|| Mutex::new(Disks::new_with_refreshed_list()));

pub async fn info(state: Arc<AppState>) -> Result<InfoBo, AppError> {
    let system = {
        let mut system = SYSTEM_INFO.lock().unwrap_or_else(|err| {
            let mut inner = err.into_inner();
            *inner = System::new_all();
            inner
        });
        system.refresh_all();
        InfoSystemBo::from(&*system)
    };
    let disks = {
        let mut disks = DISKS_INFO.lock().unwrap_or_else(|err| {
            let mut inner = err.into_inner();
            *inner = Disks::new_with_refreshed_list();
            inner
        });
        disks.refresh(false);
        disks.into_iter().map(InfoDiskBo::from).collect()
    };
    let database = {
        let idle_connections = state.db.num_idle();
        let total_connections = state.db.size();
        let state = state.db.acquire().await;
        InfoDatabaseBo {
            state: match state {
                Ok(_) => "OK".into(),
                Err(e) => format!("{e}"),
            },
            idle_connections,
            total_connections,
        }
    };
    Ok(InfoBo {
        system,
        disks,
        database,
    })
}
