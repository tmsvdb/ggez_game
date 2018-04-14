
use ggez::graphics;
use ggez::event;
use ggez::{Context, GameResult};

/// GEZZ implementaion and ECS implemenation

pub struct ECS <C: Components> 
{
	entities: Vec<Entity<C>>,
	systems: Vec<Box<System<C>>>,
}

impl <C: Components> ECS <C> {

    pub fn new(_ctx: &mut Context) -> GameResult<ECS<C>> {
        let s = ECS { entities: Vec::new(), systems: Vec::new() };
        Ok(s)
    }

    pub fn register_system<S: System<C> + 'static>(&mut self, system: S) {
    	self.systems.push(Box::new(system));
    }

    pub fn register_entity (&mut self, entity: Entity<C>) {
    	self.entities.push(entity);
    }
    /*
    pub fn borrow_entity (&mut self, name: String) -> Result<&mut Entity<C>, EcsError> {
    	for e in &mut self.entities {
    		if e.properties.name == name {
    			return Ok(e);
    		}
    	}
    	Err(EcsError::EntityNotFound)
    }
    */
}

impl <C: Components> event::EventHandler for ECS<C>{

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    	for system in &self.systems {
    		for entity in &mut self.entities {
    			system.update(entity);
    		}
    	}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        for system in &self.systems {
    		for entity in &mut self.entities {
    			system.draw(ctx, entity);
    		}
    	}
        graphics::present(ctx);
        Ok(())
    }
}



/// ECS types & properties


/// ENTITY
#[derive(Default)]
pub struct Entity <C: Components> {
	pub properties: EntityProperties,
	pub components: C::ComponentsType,
}

pub struct EntityProperties {
	pub name: String,
}

impl Default for EntityProperties {
    fn default() -> EntityProperties { 
    	EntityProperties {name: Default::default() }
    }
}

impl <C: Components> Entity <C> {

	pub fn new (name: &str, components: C::ComponentsType) -> Entity<C> {
		Entity {properties:EntityProperties {name: String::from(name)}, components: components }
	}
}


/// COMPONENT
pub trait Components {

    type ComponentsType;
}


/// SYSTEM
pub trait System <C: Components>
{	

	/// update the entity using this system
	fn update (&self, entity: &mut Entity<C>);

	/// each system has the ability to render to the ggez context
	fn draw (&self, ctx: &mut Context, entity: &mut Entity<C>);
}
