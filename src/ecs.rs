
use ggez::graphics;
use ggez::graphics::{Drawable, DrawParam};
use ggez::event;
use ggez::{Context, GameResult};

/// GEZZ implementaion and ECS implemenation

pub struct ECS <C: Components> 
{
	entities: Vec<Entity<C>>,
	systems: Vec<Box<SystemUpdate<C>>>,
}

impl <C: Components> ECS <C> {

    pub fn new(_ctx: &mut Context) -> GameResult<ECS<C>> {
        let s = ECS { entities: Vec::new(), systems: Vec::new() };
        Ok(s)
    }

    pub fn register_for_update<S: SystemUpdate<C> + 'static>(&mut self, system: S) {
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
        for entity in &mut self.entities {
			match entity.properties.drawable {
				DrawEntity::Image(ref i) => graphics::draw_ex(ctx, i, entity.properties.params).unwrap(),
				DrawEntity::Text(ref t) => graphics::draw_ex(ctx, t, entity.properties.params).unwrap(),
			}
		}
        graphics::present(ctx);
        Ok(())
    }
}



/// ECS types & properties

pub enum DrawEntity {
	Image(graphics::Image),
	Text(graphics::Text),
}

/// ENTITY
#[derive(Default)]
pub struct Entity <C: Components> {
	pub properties: EntityProperties,
	pub components: C::ComponentsType,
}

pub struct EntityProperties {
	pub name: String,
    pub params: DrawParam,
    pub drawable: DrawEntity, 
}

impl Default for EntityProperties {
	fn default () -> Self {
		EntityProperties{ ..Default::default() }
	}
}

/*
impl Default for EntityProperties {
    fn default() -> EntityProperties { 
    	EntityProperties {name: Default::default(), ..Default::default() }
    }
}*/

impl <C: Components> Entity <C> {

	pub fn new (properties: EntityProperties, components: C::ComponentsType) -> Entity<C> {
		Entity {properties:properties, components: components }
	}
}


/// COMPONENT
pub trait Components {

    type ComponentsType;
}


/// SYSTEM
pub trait SystemUpdate <C: Components>
{	
	/// update the entity using this system
	fn update (&self, entity: &mut Entity<C>);
}

pub trait SystemDraw<C: Components>
{	
	/// each system has the ability to render to the ggez context
	fn draw (&self, ctx: &mut Context, entity: &mut Entity<C>);

}

pub trait SystemKeyboard
{	
	/// each system has the ability to render to the ggez context
	fn draw (&self, ctx: &mut Context);

}
