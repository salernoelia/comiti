use notify::{
    Config, Event, RecommendedWatcher, RecursiveMode, Result as NotifyResult,
    Watcher as NotifyWatcher,
};
use std::path::PathBuf;

pub struct Watcher {
    paths: Vec<PathBuf>,
    watcher: Option<RecommendedWatcher>,
}

impl Watcher {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        println!("{:?}", paths);
        Watcher {
            paths,
            watcher: None,
        }
    }

    pub fn watch<F: Fn(Event) + Send + 'static>(&mut self, trigger: F) {
        let mut watcher: RecommendedWatcher = NotifyWatcher::new(
            move |res: NotifyResult<Event>| {
                if let Ok(event) = res {
                    let ignore = event
                        .paths
                        .iter()
                        .any(|p| p.components().any(|c| c.as_os_str() == ".git"));
                    if !ignore {
                        trigger(event);
                    }
                }
            },
            Config::default(),
        )
        .expect("Failed to create watcher");

        for path in &self.paths {
            watcher
                .watch(path, RecursiveMode::Recursive)
                .expect("Failed to watch path");
        }

        self.watcher = Some(watcher); // Store watcher so it isn't dropped
    }
}
