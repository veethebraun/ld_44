use amethyst::core::{ArcThreadPool, SystemBundle};
use amethyst::ecs::{Dispatcher, DispatcherBuilder, System, World};
use amethyst::{DataInit, Error, Result};

pub struct PausableGameData<'a, 'b> {
    core_dispatcher: Dispatcher<'a, 'b>,
    running_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> PausableGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            self.running_dispatcher.dispatch(&world.res);
        }
        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct PausableGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for PausableGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        PausableGameDataBuilder::new()
    }
}

impl<'a, 'b> PausableGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        PausableGameDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
        }
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.core)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.running.add(system, name, dependencies);
        self
    }

    pub fn with_running_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.running)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }
}

impl<'a, 'b> DataInit<PausableGameData<'a, 'b>> for PausableGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> PausableGameData<'a, 'b> {
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();
        core_dispatcher.setup(&mut world.res);
        running_dispatcher.setup(&mut world.res);

        PausableGameData {
            core_dispatcher,
            running_dispatcher,
        }
    }
}
